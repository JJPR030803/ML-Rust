use serialport::SerialPort;
use std::io::{self, BufRead, BufReader};
use std::time::Duration;

/// Receives signals from an Arduino and parses them into a matrix
pub fn recibir_senales_arduino(
    puerto_arduino: Option<String>,
    baud_rate: u32,
    matriz: &mut Vec<Vec<String>>,
    debug: bool,
) -> Result<(), io::Error> {
    let puerto = match puerto_arduino {
        Some(p) => p,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Arduino port not found",
            ))
        }
    };

    match serialport::new(&puerto, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()
    {
        Ok(port) => {
            println!("Conexion establecida");
            let mut reader = BufReader::new(port);

            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        let line = line.trim();
                        if !line.is_empty() {
                            if debug {
                                println!("Valores recibidos: {}", line);
                            }

                            let items: Vec<String> = line.split('-').map(String::from).collect();
                            let mut parsed_line = Vec::new();

                            for item in items {
                                match item.parse::<i32>() {
                                    Ok(num) => parsed_line.push(num.to_string()),
                                    Err(_) => parsed_line.push(item),
                                }
                            }

                            matriz.push(parsed_line);
                        }
                    }
                    Err(e) => {
                        println!("Error al procesar datos: {}", e);
                        break;
                    }
                }
            }

            Ok(())
        }
        Err(e) => {
            println!("Error en la conexion serial: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        }
    }
}