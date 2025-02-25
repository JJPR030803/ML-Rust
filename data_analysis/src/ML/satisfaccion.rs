
pub fn calcular_satisfaccion(vo:f64,vmin:f64,vmax:f64,modo:&str) -> Result<f64,String>{
    if vmax <= vmin{
        return Err("Valor maximo no puede ser menor al minimo".to_string());
    }

    match modo{
        "minimizacion" => Ok(((vmax-vo) / (vmax - vmin)).powi(2)),
        "maximizacion" => Ok(((vo-vmin) / (vmax - vmin)).powi(2)), 
        _ => Err("Modo debe ser minimizacion o maximizacion".to_string()),
}
}

pub fn calcular_satisfaccion_lista(){

}
