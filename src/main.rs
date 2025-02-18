use std::vec;

use gnuplot::{Figure,Caption,Color};

fn main() {
   let x = vec![1.0,2.0,3.0,4.0,5.0];
   let y = vec![2.0,3.5,1.5,4.0,2.5];


   let mut fg = Figure::new();

   fg.axes2d().lines(&x, &y,&[Caption("Sample Line"),Color(("blue"))]);


   fg.show().unwrap();


}
