use serialport::SerialPortType;

pub fn buscar_puerto_arduino(vid:u16,pid:u16) -> Option<String>{
    match serialport::available_ports(){
        Ok(ports) =>{
            for port in ports{
                if let SerialPortType::UsbPort(info) = port.port_type{
                    if info.vid == vid && info.pid == pid{
                        return  Some(port.port_name);
                    }
                }
            }
            None
        }
        Err(_) => None,
    }
}