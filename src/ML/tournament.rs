// src/ml/tournament.rs
use rand::prelude::*;
use crate::ML::selection::calculate_fitness;

/// Type alias for a parent with its fitness and genetic vector
pub type Parent = (f64, Vec<f64>);

/// Perform binary tournament selection ensuring even number of parents.
///
/// # Arguments
///
/// * `population` - A slice of individuals (vectors)
/// * `num_parents` - Number of parents to select (will be adjusted to be even)
///
/// # Returns
///
/// A vector of selected parents with their fitness values
///
/// # Panics
///
/// If num_parents <= 0 or population is empty
pub fn binary_tournament(population: &[Vec<f64>], mut num_parents: usize) -> Vec<Parent> {
    if num_parents <= 0 {
        panic!("Number of parents must be positive");
    }
    
    if population.is_empty() {
        panic!("Population cannot be empty");
    }

    // Ensure even number of parents
    num_parents = num_parents - (num_parents % 2);
    if num_parents == 0 {
        num_parents = 2; // Minimum number of parents
    }
    
    if num_parents > population.len() {
        num_parents = population.len() - (population.len() % 2);
        println!("Warning: Adjusted number of parents to {} to match population size", num_parents);
    }
    
    let pop_size = population.len();
    let mut rng = thread_rng();
    let mut result = Vec::with_capacity(num_parents);
    
    for _ in 0..num_parents {
        // Select two random individuals
        let idx1 = rng.gen_range(0..pop_size);
        // Make sure we pick a different second individual
        let mut idx2 = rng.gen_range(0..pop_size);
        while idx2 == idx1 {
            idx2 = rng.gen_range(0..pop_size);
        }
        
        let fitness1 = calculate_fitness(&population[idx1]);
        let fitness2 = calculate_fitness(&population[idx2]);
        
        // Choose the one with better fitness (lower is better)
        if fitness1 < fitness2 {
            result.push((fitness1, population[idx1].clone()));
        } else {
            result.push((fitness2, population[idx2].clone()));
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tournament() {
        let population = vec![
            vec![1.0, 1.0],     // fitness = 2
            vec![2.0, 2.0],     // fitness = 8
            vec![3.0, 3.0],     // fitness = 18
            vec![4.0, 4.0],     // fitness = 32
        ];
        
        let parents = binary_tournament(&population, 4);
        
        // Should return 4 parents
        assert_eq!(parents.len(), 4);
        
        // Each parent should have a fitness value and vector
        for (fitness, parent) in &parents {
            // Check that the parent's fitness matches the expected calculation
            assert_eq!(*fitness, calculate_fitness(parent));
        }
    }

    #[test]
    #[should_panic(expected = "Number of parents must be positive")]
    fn test_binary_tournament_invalid_num_parents() {
        let population = vec![vec![1.0, 2.0]];
        binary_tournament(&population, 0);
    }
    
    #[test]
    #[should_panic(expected = "Population cannot be empty")]
    fn test_binary_tournament_empty_population() {
        let population: Vec<Vec<f64>> = Vec::new();
        binary_tournament(&population, 2);
    }
}