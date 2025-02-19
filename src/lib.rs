pub mod arduino;
pub mod utils;
pub mod data;

// re exporta items mas usados
pub use arduino::connection::recibir_senales_arduino;
pub use arduino::port::buscar_puerto_arduino;
pub use data::csv_handler::write_sensor_data;