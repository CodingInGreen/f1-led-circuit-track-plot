use csv::Reader;
use plotters::prelude::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Location {
    x: i32,
    y: i32,
    z: i32,  
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the CSV file
    let csv_file_path = "zandvoort_data.csv";

    // Create a reader for the CSV
    let mut rdr = Reader::from_path(csv_file_path)?;

    // Set up plot
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Zandvoort Track X and Y Positions", ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1000..1000, -1000..1000)?;

    chart.configure_mesh().draw()?;

    // Iterate over records and plot each point
    for result in rdr.deserialize() {
        let record: Location = result?;
        chart.draw_series(PointSeries::of_element(
            vec![(record.x, record.y)],
            1, // This is the diameter for each point
            &RED,
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled())
            },
        ))?;
    }

    // To make sure the data is visible on the drawing area
    root.present()?;

    println!("Plot is generated and saved as plot.png.");
    Ok(())
}
