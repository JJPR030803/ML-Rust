// src/ml/genetic_optimizer.rs
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;

use crate::ML::crossover_mutation::{crossover, mutation};
use crate::ML::selection::{environmental_selection, FloatKey};
use crate::ML::tournament::binary_tournament;

/// A genetic optimizer that uses tournament selection, crossover, and mutation.
pub struct GeneticOptimizer {
    pub pop_size: usize,
    pub vector_size: usize,
    pub num_parents: usize,
    pub mutation_rate: f64,
    pub selection_ratio: f64,
    pub gene_min: f64,
    pub gene_max: f64,
    population: Vec<Vec<f64>>,
}

impl GeneticOptimizer {
    /// Create a new genetic optimizer with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `pop_size` - Population size
    /// * `vector_size` - Size of each individual's genetic vector
    /// * `num_parents` - Number of parents to select in each generation
    /// * `mutation_rate` - Probability of mutation (0 to 1)
    /// * `selection_ratio` - Ratio of population to select for next generation (0 to 1)
    /// * `gene_min` - Minimum value for genes
    /// * `gene_max` - Maximum value for genes
    ///
    /// # Panics
    ///
    /// If parameters are invalid
    pub fn new(
        pop_size: usize,
        vector_size: usize,
        num_parents: usize,
        mutation_rate: f64,
        selection_ratio: f64,
        gene_min: f64,
        gene_max: f64,
    ) -> Self {
        // Validate parameters
        if pop_size == 0 {
            panic!("Population size must be positive");
        }
        if vector_size == 0 {
            panic!("Vector size must be positive");
        }
        if num_parents == 0 {
            panic!("Number of parents must be positive");
        }
        
        // Ensure population size is sufficient and even
        let pop_size = std::cmp::max(4, pop_size);
        
        // Ensure number of parents is even and not larger than population
        let num_parents = if num_parents > pop_size {
            let adjusted = pop_size - (pop_size % 2);
            println!("Warning: Adjusted number of parents to {} to match population size", adjusted);
            adjusted
        } else if num_parents % 2 != 0 {
            let adjusted = num_parents - 1;
            println!("Warning: Adjusted number of parents to {} to ensure even number", adjusted);
            adjusted
        } else {
            num_parents
        };
        
        // Ensure mutation rate is between 0 and 1
        let mutation_rate = mutation_rate.clamp(0.0, 1.0);
        
        // Ensure selection ratio is between 0.1 and 1
        let selection_ratio = selection_ratio.clamp(0.1, 1.0);
        
        // Ensure gene range is valid
        let gene_min = gene_min.min(gene_max);
        let gene_max = gene_max.max(gene_min);
        
        // Initialize random population
        let mut rng = rand::thread_rng();
        let distribution = Uniform::from(gene_min..=gene_max);
        
        let mut population = Vec::with_capacity(pop_size);
        for _ in 0..pop_size {
            let individual: Vec<f64> = (0..vector_size)
                .map(|_| distribution.sample(&mut rng))
                .collect();
            population.push(individual);
        }
        
        Self {
            pop_size,
            vector_size,
            num_parents,
            mutation_rate,
            selection_ratio,
            gene_min,
            gene_max,
            population,
        }
    }
    
    /// Ensure population size remains constant.
    fn maintain_population_size(&mut self) {
        let current_size = self.population.len();
        
        if current_size < self.pop_size {
            // Add new random individuals
            let additional_needed = self.pop_size - current_size;
            let mut rng = rand::thread_rng();
            let distribution = Uniform::from(self.gene_min..=self.gene_max);
            
            for _ in 0..additional_needed {
                let individual: Vec<f64> = (0..self.vector_size)
                    .map(|_| distribution.sample(&mut rng))
                    .collect();
                self.population.push(individual);
            }
        } else if current_size > self.pop_size {
            // Trim excess individuals
            self.population.truncate(self.pop_size);
        }
    }
    
    /// Perform one generation of evolution.
    ///
    /// # Returns
    ///
    /// The best fitness value in the current generation
    pub fn step(&mut self) -> f64 {
        // Ensure population size is correct
        self.maintain_population_size();
        
        // Parent selection
        let parents = binary_tournament(&self.population, self.num_parents);
        
        // Crossover
        let offspring = crossover(&parents, 2);
        
        // Mutation
        let mutated = mutation(self.mutation_rate, self.gene_min, self.gene_max, &offspring);
        
        // Environmental selection
        let selected = environmental_selection(self.selection_ratio, &mutated);
        
        // Update population with selected individuals
        self.population = selected.values().cloned().collect();
        
        // Maintain population size
        self.maintain_population_size();
        
        // Return best fitness (find minimum fitness key)
        let best_fitness = selected.keys()
                                  .map(|k| k.0)
                                  .fold(f64::INFINITY, |a, b| a.min(b));
        
        best_fitness
    }
    
    /// Run the genetic algorithm optimization.
    ///
    /// # Arguments
    ///
    /// * `max_generations` - Maximum number of generations to run
    /// * `target_fitness` - Optional target fitness to stop early
    ///
    /// # Returns
    ///
    /// A vector of best fitness values for each generation
    pub fn optimize(&mut self, max_generations: usize, target_fitness: Option<f64>) -> Vec<f64> {
        let mut fitness_history = Vec::with_capacity(max_generations);
        
        for gen in 0..max_generations {
            let best_fitness = self.step();
            fitness_history.push(best_fitness);
            
            if let Some(target) = target_fitness {
                if best_fitness <= target {
                    println!("Target fitness reached at generation {}", gen);
                    break;
                }
            }
        }
        
        fitness_history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_optimizer() {
        let optimizer = GeneticOptimizer::new(
            100,   // pop_size
            10,    // vector_size
            20,    // num_parents
            0.5,   // mutation_rate
            0.5,   // selection_ratio
            0.0,   // gene_min
            100.0, // gene_max
        );
        
        assert_eq!(optimizer.pop_size, 100);
        assert_eq!(optimizer.vector_size, 10);
        assert_eq!(optimizer.population.len(), 100);
        assert_eq!(optimizer.population[0].len(), 10);
    }

    #[test]
    fn test_step() {
        let mut optimizer = GeneticOptimizer::new(
            20,   // small pop for testing
            5,    // small vectors
            10,   // num_parents
            0.5,  // mutation_rate
            0.5,  // selection_ratio
            0.0,  // gene_min
            10.0, // gene_max
        );
        
        let best_fitness = optimizer.step();
        assert!(best_fitness >= 0.0);
    }

    #[test]
    fn test_optimize() {
        let mut optimizer = GeneticOptimizer::new(
            20,   // small pop for testing
            5,    // small vectors
            10,   // num_parents
            0.5,  // mutation_rate
            0.5,  // selection_ratio
            0.0,  // gene_min
            10.0, // gene_max
        );
        
        let history = optimizer.optimize(10, None);
        assert_eq!(history.len(), 10);
        
        // Check that fitness generally improves
        if !history.is_empty() {
            assert!(history[history.len() - 1] <= history[0] * 1.2);  // Allow some fluctuation
        }
    }

    #[test]
    #[should_panic(expected = "Population size must be positive")]
    fn test_invalid_pop_size() {
        GeneticOptimizer::new(0, 10, 20, 0.5, 0.5, 0.0, 100.0);
    }
}