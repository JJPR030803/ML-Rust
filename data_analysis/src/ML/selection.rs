// src/ml/selection.rs
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// Custom wrapper for f64 to implement Hash and Eq
#[derive(Clone, Copy, Debug)]
pub struct FloatKey(pub f64);

impl PartialEq for FloatKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for FloatKey {}

impl Hash for FloatKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

/// Calculate the fitness of a vector using sum of squares.
///
/// # Arguments
///
/// * `vector` - Reference to the vector to evaluate
///
/// # Returns
///
/// Sum of squares of the vector elements
pub fn calculate_fitness(vector: &[f64]) -> f64 {
    vector.iter().map(|&x| x * x).sum()
}

/// Select the best individuals from the population based on their fitness.
///
/// # Arguments
///
/// * `ratio` - Selection ratio (0 to 1)
/// * `population` - A slice of individuals (vectors)
///
/// # Returns
///
/// A HashMap mapping fitness values (wrapped in FloatKey) to selected individuals
///
/// # Panics
///
/// If ratio is not between 0 and 1
pub fn environmental_selection(ratio: f64, population: &[Vec<f64>]) -> HashMap<FloatKey, Vec<f64>> {
    if !(0.0..=1.0).contains(&ratio) {
        panic!("Selection ratio must be between 0 and 1");
    }

    // Calculate all fitnesses and sort
    let mut population_with_fitness: Vec<(f64, &Vec<f64>)> = population
        .iter()
        .map(|vec| (calculate_fitness(vec), vec))
        .collect();

    population_with_fitness.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // Select the best individuals based on ratio
    let num_selected = (population.len() as f64 * ratio).round() as usize;
    
    // Use HashMap to store the selected individuals
    let mut selected = HashMap::new();
    for (fitness, vec) in population_with_fitness.into_iter().take(num_selected) {
        selected.insert(FloatKey(fitness), vec.clone());
    }

    selected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fitness() {
        let vector = vec![1.0, 2.0, 3.0];
        assert_eq!(calculate_fitness(&vector), 14.0); // 1^2 + 2^2 + 3^2 = 14
    }

    #[test]
    fn test_environmental_selection() {
        let population = vec![
            vec![3.0, 4.0],     // fitness = 25
            vec![1.0, 1.0],     // fitness = 2
            vec![2.0, 2.0],     // fitness = 8
            vec![5.0, 5.0],     // fitness = 50
        ];
        
        let selected = environmental_selection(0.5, &population);
        
        // Should select 2 individuals with lowest fitness
        assert_eq!(selected.len(), 2);
        
        // Check if keys with correct fitness values exist
        assert!(selected.contains_key(&FloatKey(2.0)));  // fitness of [1.0, 1.0]
        assert!(selected.contains_key(&FloatKey(8.0)));  // fitness of [2.0, 2.0]
    }

    #[test]
    #[should_panic(expected = "Selection ratio must be between 0 and 1")]
    fn test_environmental_selection_invalid_ratio() {
        let population = vec![vec![1.0, 2.0]];
        environmental_selection(1.5, &population);
    }
}