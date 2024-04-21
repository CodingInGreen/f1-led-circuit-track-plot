use csv::Reader;
use plotters::prelude::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Location {
    x: i32,
    y: i32,
    z: i32,  // Not used in this plot, but retained for completeness
    date: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the CSV file
    let csv_file_path = "zandvoort_data.csv";

    // Create a reader for the CSV
    let mut rdr = Reader::from_path(csv_file_path)?;

    // Load data and sort by the 'date' column to maintain proper order
    let mut locations: Vec<Location> = rdr.deserialize().collect::<Result<_, _>>()?;
    locations.sort_by(|a, b| a.date.cmp(&b.date));

    // Set up plot
    let root = BitMapBackend::new("zandvoort_track.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Zandvoort Track - X and Y Positions", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-4500..8500, -16000..5500)?;  // Adjusted limits

    chart.configure_mesh().draw()?;

    // Plot each point with a larger size for better visibility
    chart.draw_series(PointSeries::of_element(
        locations.iter().map(|loc| (loc.x, loc.y)),
        2, // Point size
        &RED,
        &|coord, size, style| {
            EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled())
        },
    ))?;

    // Save the plot
    root.present()?;
    println!("Plot is generated and saved as zandvoort_track.png.");
    
    Ok(())
}
