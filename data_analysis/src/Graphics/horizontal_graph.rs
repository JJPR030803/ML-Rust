use std::vec;

use gnuplot::PlotOption::WhiskerBars;
use plotters::prelude::*;

pub fn generar_grafica(
    x:Vec<i32>,
    y:Vec<i32>,
    titulo: &str,
    etiqueta_x: &str,
    etiqueta_y:&str,
    file_name: &str,
)->
Result<(),Box<dyn std::error::Error>>{

    if x.len() != y.len(){
        return  Err("Tiene que ser la misma cantidad de datos: X e Y".into());
    }


    //Crear el area de dibujo
    let area_dibujo = BitMapBackend::new(file_name, (800,600)).into_drawing_area();
    area_dibujo.fill(&WHITE)?;

    //Crear grafica
    let mut chart = ChartBuilder::on(&area_dibujo)
    .caption(titulo, ("sans-serif",30))
    .margin(20)
    .x_label_area_size(40)
    .y_label_area_size(40)
    .build_cartesian_2d(
        *x.iter().min().unwrap()..*x.iter().max().unwrap(),
        *y.iter().min().unwrap()..*y.iter().max().unwrap(),
    )?;


    //Axis labels
    chart.draw_series(LineSeries::new(
        x.iter().zip(y.iter()).map(|(&x,&y)| (x,y)),
        &BLUE,
    ))?
    .label("Datos")
    .legend(|(x,y)| PathElement::new(vec![(x,y),(x+20,y)], &BLUE));


    //Agregar puntos
    chart.draw_series(
        x.iter().zip(y.iter())
        .map(|(&x,&y)| Circle::new((x,y),5,RED.filled())),
    )?;


    //Mostrar leyenda
    chart.configure_series_labels()
    .background_style(&WHITE)
    .border_style(&BLACK)
    .draw()?;

    chart.configure_mesh()
    .x_desc(etiqueta_x)
    .y_desc(etiqueta_y)
    .axis_desc_style(("sans-serif",18).into_font())
    .x_labels(10)
    .y_labels(10)
    .light_line_style(&TRANSPARENT)
    .draw()?;


    Ok(())
}