use arduino_rust::{arduino::{VID,PID},buscar_puerto_arduino,data::csv_handler::write_sensor_data};
use El_Embebidos::buscar_puerto_arduino;
use std::io;
use std::thread;
use std::time::Duration;


fn main() -> io::Result<()>{
    let puerto = buscar_puerto_arduino(VID, PID);

    if puerto.is_none(){
        println!("No se encontro el arduino");
        return Ok(());
    }

    let puerto = puerto.unwrap();


    match serialport::new(&puerto, 9600)
    .timeout(Duration::from_millis(1000)).open(){
        Ok(port) => {
            thread::sleep(Duration::from_secs(10));
            write_sensor_data(port,"sensor_data.csv")
        }
        Err(e) => {
            println!("Error de conexion serial: {}",e);
            Ok(())
        }
    }
}