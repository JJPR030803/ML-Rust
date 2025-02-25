mod ML;
mod Misc;
use core::f64;

use ML::satisfaccion::calcular_satisfaccion;
use Misc::random_num::rand_list_f64;

fn main() {


    let lista = rand_list_f64(100, 10.0, 40.0);
    let min = lista.iter().cloned().fold(f64::INFINITY, f64::min);
    let max:f64 = lista.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let vos:Vec<f64> = vec![24.0,40.0,90.0];


    match calcular_satisfaccion(vos[0], min, max, "maximizacion") {
        Ok(sati) => println!("Satisfaccion exitosa {:.}",sati),
        Err(e) => println!("Error: {}",e),
    }

    let satisfacciones: Vec<Result<f64,String>> = vos.iter()
    .map(|&val| calcular_satisfaccion(val, min, max, "minimizacion"))
    .collect();


    println!("Satisfacciones: {:?}", satisfacciones);


}
