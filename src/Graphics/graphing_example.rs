use gnuplot::{Figure,Caption,Color};

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

pub fn boxplot_graph(){

}