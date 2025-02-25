// src/ml/crossover_mutation.rs
use crate::ML::tournament::Parent;
use rand::prelude::*;

/// Perform multi-point crossover between pairs of parents.
///
/// # Arguments
///
/// * `parents` - A vector of parents with their fitness values 
/// * `num_crosspoints` - Number of crossover points
///
/// # Returns
///
/// A vector of offspring (genetic vectors)
///
/// # Panics
///
/// If number of parents is not even or if num_crosspoints is too large
pub fn crossover(parents: &[Parent], num_crosspoints: usize) -> Vec<Vec<f64>> {
    let num_parents = parents.len();
    
    if num_parents % 2 != 0 {
        panic!("Number of parents must be even");
    }
    
    if parents.is_empty() {
        return Vec::new();
    }
    
    let vector_length = parents[0].1.len();
    
    if num_crosspoints >= vector_length {
        panic!("Number of crosspoints must be less than vector length");
    }
    
    let mut offspring = Vec::with_capacity(num_parents);
    let mut rng = thread_rng();
    
    for i in (0..num_parents).step_by(2) {
        let parent1 = &parents[i].1;
        let parent2 = &parents[i + 1].1;
        
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();
        
        // Generate random crosspoints
        let mut indices: Vec<usize> = (1..vector_length).collect();
        indices.shuffle(&mut rng);
        let mut crosspoints: Vec<usize> = indices.into_iter().take(num_crosspoints).collect();
        crosspoints.sort_unstable();
        
        for j in 0..crosspoints.len() {
            if j % 2 == 0 {
                let start = crosspoints[j];
                let end = if j + 1 < crosspoints.len() {
                    crosspoints[j + 1]
                } else {
                    vector_length
                };
                
                // Swap segments between children
                for k in start..end {
                    std::mem::swap(&mut child1[k], &mut child2[k]);
                }
            }
        }
        
        offspring.push(child1);
        offspring.push(child2);
    }
    
    offspring
}

/// Perform mutation on offspring population.
///
/// # Arguments
///
/// * `mutation_rate` - Probability of mutation (0 to 1)
/// * `gene_min` - Minimum value for genes
/// * `gene_max` - Maximum value for genes
/// * `offspring` - Offspring population to mutate
///
/// # Returns
///
/// Mutated offspring
///
/// # Panics
///
/// If mutation_rate is not between 0 and 1
pub fn mutation(mutation_rate: f64, gene_min: f64, gene_max: f64, offspring: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if !(0.0..=1.0).contains(&mutation_rate) {
        panic!("Mutation rate must be between 0 and 1");
    }
    
    let mut rng = thread_rng();
    let mut mutated = Vec::with_capacity(offspring.len());
    
    for individual in offspring {
        let mut mutated_individual = individual.clone();
        
        for gene in &mut mutated_individual {
            if rng.gen::<f64>() < mutation_rate {
                *gene = rng.gen_range(gene_min..=gene_max);
            }
        }
        
        mutated.push(mutated_individual);
    }
    
    mutated
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_parents() -> Vec<Parent> {
        vec![
            (2.0, vec![1.0, 1.0, 1.0, 1.0]),
            (8.0, vec![2.0, 2.0, 2.0, 2.0]),
            (18.0, vec![3.0, 3.0, 3.0, 3.0]),
            (32.0, vec![4.0, 4.0, 4.0, 4.0]),
        ]
    }

    #[test]
    fn test_crossover() {
        let parents = create_test_parents();
        let offspring = crossover(&parents, 2);
        
        // Should return same number of offspring as parents
        assert_eq!(offspring.len(), parents.len());
        
        // Each offspring should have same length as parents
        for child in &offspring {
            assert_eq!(child.len(), parents[0].1.len());
        }
    }

    #[test]
    #[should_panic(expected = "Number of parents must be even")]
    fn test_crossover_odd_parents() {
        let mut parents = create_test_parents();
        parents.pop();  // Make it odd
        crossover(&parents, 2);
    }

    #[test]
    fn test_mutation() {
        let offspring = vec![
            vec![1.0, 1.0, 1.0, 1.0],
            vec![2.0, 2.0, 2.0, 2.0],
        ];
        
        // With mutation_rate = 0, should be identical
        let mutated_zero = mutation(0.0, 0.0, 10.0, &offspring);
        assert_eq!(mutated_zero, offspring);
        
        // With mutation_rate = 1, every gene should be different
        // Note: There's a tiny probability this could fail by random chance
        let mutated_all = mutation(1.0, 0.0, 10.0, &offspring);
        let all_different = mutated_all.iter().enumerate().all(|(i, vec)| {
            vec.iter().enumerate().any(|(j, &val)| val != offspring[i][j])
        });
        assert!(all_different);
    }

    #[test]
    #[should_panic(expected = "Mutation rate must be between 0 and 1")]
    fn test_mutation_invalid_rate() {
        let offspring = vec![vec![1.0, 1.0]];
        mutation(1.5, 0.0, 10.0, &offspring);
    }
}