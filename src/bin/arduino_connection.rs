use arduino_rust::{arduino::{VID, PID}, buscar_puerto_arduino, recibir_senales_arduino};

fn main() {
    let puerto_arduino = buscar_puerto_arduino(VID, PID);
    let baud_rate = 9600;
    let mut matriz = Vec::new();

    match recibir_senales_arduino(puerto_arduino, baud_rate, &mut matriz, false) {
        Ok(_) => println!("Collected {} rows of data", matriz.len()),
        Err(e) => {
            println!("Error: {}", e);
            println!("Collected {} rows of data before error", matriz.len());
        }
    }
}