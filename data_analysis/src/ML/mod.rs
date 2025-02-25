// src/ml/mod.rs

pub mod data_imputation;
pub mod genetic_optimizer;
pub mod tournament;
pub mod crossover_mutation;
pub mod selection;
pub mod arima;
pub mod satisfaccion;
// Re-export main components for easier access
pub use genetic_optimizer::GeneticOptimizer;
pub use data_imputation::{linear_interpolation, simple_exponential_smoothing, median};
