pub fn test_arima_implementation(){
    // Load your time series data
    let data = Array1::from(vec![100.0, 105.0, 110.0, 112.0, 115.0, 120.0, 125.0, 130.0]);
    
    // Create ARIMA model with p=1, d=1, q=1
    let mut model = ARIMA::new(1, 1, 1);
    
    // Fit the model
    match model.fit(&data) {
        Ok(_) => println!("Model fitted successfully"),
        Err(e) => println!("Error: {}", e),
    }
    
    // Forecast next 3 steps
    match model.forecast(&data, 3) {
        Ok(forecasts) => println!("Forecasts: {:?}", forecasts),
        Err(e) => println!("Error: {}", e),
    }
}

fn test_gen_optimizer(){
// Create a new genetic optimizer for minimizing the sum of squares
let mut optimizer = GeneticOptimizer::new(
    100,   // pop_size
    10,    // vector_size
    20,    // num_parents
    0.5,   // mutation_rate
    0.5,   // selection_ratio
    -10.0, // gene_min
    10.0,  // gene_max
);

println!("Starting genetic optimization...");
println!("Target: Find a vector of 10 values that minimizes the sum of squares.");

// Run for 100 generations with a target fitness of 0.1
let history = optimizer.optimize(100, Some(0.1));

// Report results
println!("Optimization completed after {} generations", history.len());
println!("Initial best fitness: {:.6}", history[0]);
println!("Final best fitness: {:.6}", history[history.len() - 1]);

// Calculate improvement
let improvement = 100.0 * (history[0] - history[history.len() - 1]) / history[0];
println!("Fitness improved by {:.2}%", improvement);
}