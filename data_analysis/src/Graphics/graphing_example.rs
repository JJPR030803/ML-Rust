use std::{clone, vec};

use gnuplot::{AutoOption::{self, Auto, Fix}, AxesCommon, Caption, Color, Figure, LabelOption, Tick::{self, Minor}, TickOption::{self, OnAxis}};
use statrs::statistics::{Data, OrderStatistics};

fn ejemplo(){
    let x = vec![1.0,2.0,3.0,4.0,5.0];
    let y = vec![2.0,3.5,1.5,4.0,2.5];
    let y2 = vec![1.0,2.0,3.0,3.5,4.5];
 
 
    let mut fg = Figure::new();
 
    fg.axes2d()
    .lines(&x, &y, &[Caption("Linea 1"), Color("blue")])
    .lines(&x, &y2, &[Caption("Linea 2"),Color("red")]);
 
 
    fg.set_terminal("pngcairo", "plot2.png");
    fg.show().unwrap();
}


#[derive(Debug)]
pub struct BoxplotData {
    pub label: String,
    pub values: Vec<f64>,
    quartiles: Option<(f64, f64, f64)>, // Q1, median, Q3
    whiskers: Option<(f64, f64)>,       // min, max (excluding outliers)
    pub outliers: Vec<f64>,
}

impl BoxplotData {
    pub fn new(label: &str, values: Vec<f64>) -> Self {
        let mut data = BoxplotData {
            label: label.to_string(),
            values,
            quartiles: None,
            whiskers: None,
            outliers: Vec::new(),
        };use gnuplot::{Figure,Caption,Color};
        data.calculate_statistics();
        data
    }

    fn calculate_statistics(&mut self) {
        if self.values.is_empty() {
            return;
        }

        let mut sorted_values = self.values.clone();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let mut data = Data::new(sorted_values.clone());
        
        // Calculate quartiles
        let q1 = data.lower_quartile();
        let median = data.median();
        let q3 = data.upper_quartile();
        self.quartiles = Some((q1, median, q3));

        // Calculate IQR and whisker bounds
        let iqr = q3 - q1;
        let lower_bound = q1 - 1.5 * iqr;
        let upper_bound = q3 + 1.5 * iqr;

        // Find whiskers and outliers
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        self.outliers.clear();

        for &value in &sorted_values {
            if value < lower_bound || value > upper_bound {
                self.outliers.push(value);
            } else {
                min = min.min(value);
                max = max.max(value);
            }
        }

        self.whiskers = Some((min, max));
    }

    pub fn get_statistics(&self) -> Option<BoxplotStatistics> {
        match (self.quartiles, self.whiskers) {
            (Some((q1, median, q3)), Some((min, max))) => Some(BoxplotStatistics {
                min,
                q1,
                median,
                q3,
                max,
                outliers: self.outliers.clone(),
            }),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BoxplotStatistics {
    pub min: f64,
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
    pub max: f64,
    pub outliers: Vec<f64>,
}



pub fn create_boxplot(data: &[BoxplotData], title: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut fg = Figure::new();
    
    fg.set_terminal("pngcairo", output_file);

    // Prepare the vectors of data
    let mut positions = Vec::new();
    let mut whisker_min = Vec::new();
    let mut box_min = Vec::new();
    let mut medians = Vec::new();
    let mut box_max = Vec::new();
    let mut whisker_max = Vec::new();
    let mut outlier_x = Vec::new();
    let mut outlier_y = Vec::new();

    // Create properly formatted ticks
    let ticks: Vec<(f64, String)> = data.iter().enumerate()
        .map(|(i, d)| (i as f64, d.label.clone()))
        .collect();

    for (i, d) in data.iter().enumerate() {
        if let Some(stats) = d.get_statistics() {
            positions.push(i as f64);
            whisker_min.push(stats.min);
            box_min.push(stats.q1);
            medians.push(stats.median);
            box_max.push(stats.q3);
            whisker_max.push(stats.max);

            for &outlier in &stats.outliers {
                outlier_x.push(outlier);
                outlier_y.push(i as f64);
            }
        }
    }
    
    let mut axes = fg.axes2d();
    
    axes.set_title(title, &[])
        .set_y_range(Fix(-0.5), Fix(data.len() as f64 - 0.5))
        // Rotate x-axis labels for better readability
        .set_x_ticks_custom(ticks.iter().map(|(x, label)| 
            Tick::Major(*x, Fix(label.to_string()))
        ), &[], &[])
        .set_x_label("Samples", &[])
        .set_y_label("Values", &[])
        .box_and_whisker(
           &positions,
           &whisker_min,
           &box_min,
           &medians,
           &box_max,
           &[Color("black"), Caption("")]
        )
        .points(&outlier_x, &outlier_y, &[Color("red"), Caption("Outliers")])
        // Add grid for better readability
        .set_grid_options(true, &[Color("gray"), Caption("")])
        .set_grid_options(true, &[Color("gray"), Caption("")]);

    fg.show()?;
    
    Ok(())
}