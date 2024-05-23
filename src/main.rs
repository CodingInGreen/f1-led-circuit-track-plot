use csv::Reader;
use plotters::prelude::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Location {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the CSV file
    let csv_file_path = "zandvoort_data.csv";

    // Create a reader for the CSV
    let mut rdr = Reader::from_path(csv_file_path)?;

    // Load data without sorting by 'date'
    let locations: Vec<Location> = rdr.deserialize().collect::<Result<_, _>>()?;

    // Set up plot with adjusted Cartesian limits
    let root = BitMapBackend::new("zandvoort_track_adjusted.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;

    // Adjust the limits based on the data range
    let x_min = locations.iter().map(|loc| loc.x).min().unwrap_or(-5000);
    let x_max = locations.iter().map(|loc| loc.x).max().unwrap_or(5000);
    let y_min = locations.iter().map(|loc| loc.y).min().unwrap_or(-5000);
    let y_max = locations.iter().map(|loc| loc.y).max().unwrap_or(5000);

    let mut chart = ChartBuilder::on(&root)
        .caption("Zandvoort Track - X and Y Positions", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    // Plot each point with a larger size for better visibility
    chart.draw_series(PointSeries::of_element(
        locations.iter().map(|loc| (loc.x, loc.y)),
        3, // Larger point size
        &RED,
        &|coord, size, style| {
            EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled())
        },
    ))?;

    // Save the plot
    root.present()?;
    println!("Plot is generated and saved as zandvoort_track_adjusted.png.");

    Ok(())
}
