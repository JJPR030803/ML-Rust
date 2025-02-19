use csv::Writer;
use std::fs::File;
use std::io::{self, BufRead,BufReader};
use serialport::{ClearBuffer, SerialPort};


pub fn write_sensor_data(port:impl SerialPort, file_path: &str)-> io::Result<()>{
    let file = File::create(file_path)?;
    let mut writer = Writer::from_writer(file);

    writer.write_record(&["Timestamp","Temperature","Humidity"])?;

    writer.flush()?;

    let mut reader = BufReader::new(port);

    loop {
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => break, //EOF
            Ok(_) => {
                let line = line.trim();
                if !line.is_empty(){
                    let data: Vec<&str> = line.split(',').collect();
                    if data.len() == 3{
                        writer.write_record(&data)?;
                        writer.flush()?;
                    }
                }
            }
            Err(e) => {
                println!("Error al leer datos: {}",e);
                break;
            }
        }
    }
    Ok(())
}