use plotters::prelude::*;
use plotters::coord::types::RangedCoordf32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the output image
    let root = BitMapBackend::new("boxplot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Temperature Boxplot", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-1..2, 15.0f32..40.0f32)?;  // Adjust Y-axis range for temperature

    chart.configure_mesh()
    .disable_x_mesh()
    .bold_line_style(&BLACK.mix(0.3))
    .y_desc("Temperatura (C)")
    .draw()?;

    // Example temperature data (min, Q1, median, Q3, max)
    let min:f32 = 18.0;
    let q1: f32 = 22.0;
    let median: f32 = 25.0;
    let q3: f32 = 28.0;
    let max: f32 = 35.0;

    let cuartiles: Quartiles = Quartiles::new(&[min,q1,median,q3,max]);

    // Draw the boxplot
    chart.draw_series(vec![
        Boxplot::new_vertical(0, &cuartiles)
            .style(ShapeStyle {
                color: RGBColor(0, 100, 200).to_rgba(),
                filled: true,
                stroke_width: 2,
            }),
    ])?;

    // Finalize and save the plot
    root.present()?;
    println!("Boxplot has been saved to 'boxplot.png'");

    Ok(())
}
