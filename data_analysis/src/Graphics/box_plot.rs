use plotters::{chart, prelude::*};


pub struct TemperatureQuartiles{
   pub min: f32,
    pub q1:f32,
    pub median:f32,
    pub q3:f32,
    pub max:f32
}


pub fn boxplotting(filepath:String,titulo:String,cuartiles: TemperatureQuartiles) -> Result<(),Box<dyn std::error::Error>>{
    let root = BitMapBackend::new(&filepath, (800,600)).into_drawing_area();
    root.fill(&WHITE)?;



    //Crear chart
    let mut chart = ChartBuilder::on(&root)
    .caption(&titulo, ("sans-serif",30))
    .margin(10)
    .x_label_area_size(40)
    .y_label_area_size(40)
    .build_cartesian_2d(-2..2, 5.0f32..50.0f32)?;

    chart.configure_mesh()
    .disable_x_mesh()
    .bold_line_style(&BLACK.mix(0.3))
    .y_desc("Temperatura")
    .draw()?;


    let cuartiles: Quartiles = Quartiles::new(
        &[
            cuartiles.min,
            cuartiles.q1,
            cuartiles.median,
            cuartiles.q3,
            cuartiles.max
        ]
    );

    //Dibujar el boxplot

    chart.draw_series(vec![
        Boxplot::new_vertical(0, &cuartiles)
        .style(ShapeStyle{
            color: RGBColor(0,100,200).to_rgba(),
            filled: true,
            stroke_width:2,

        })
    ])?;

    root.present()?;


    Ok(())
}