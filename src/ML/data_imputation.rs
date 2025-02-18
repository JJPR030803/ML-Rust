// src/ml/data_imputation.rs

/// Linear interpolation for a missing value in a list.
///
/// # Arguments
///
/// * `values` - A slice of values
/// * `missing_index` - The index of the missing value
///
/// # Returns
///
/// The interpolated value
pub fn linear_interpolation(values: &[f64], missing_index: usize) -> f64 {
    values[missing_index - 1]
        + ((values[missing_index + 1] - values[missing_index - 1])
            / ((missing_index + 1) as f64 - (missing_index - 1) as f64)
            * (missing_index as f64 - (missing_index - 1) as f64))
}

/// Simple exponential smoothing for a sequence of values.
///
/// # Arguments
///
/// * `values` - A slice of values to smooth
/// * `alpha` - The smoothing factor (0 < alpha < 1)
///
/// # Returns
///
/// A vector of smoothed values
pub fn simple_exponential_smoothing(values: &[f64], alpha: f64) -> Vec<f64> {
    if values.is_empty() {
        return Vec::new();
    }

    let mut smoothed_values = Vec::with_capacity(values.len());
    smoothed_values.push(values[0]);

    for t in 1..values.len() {
        let smoothed_value = alpha * values[t] + (1.0 - alpha) * smoothed_values[t - 1];
        smoothed_values.push(smoothed_value);
    }

    smoothed_values
}

/// Calculate the median values for segments of the data.
///
/// # Arguments
///
/// * `data` - A slice of values
/// * `time_interval` - The size of each segment
///
/// # Returns
///
/// A vector of median values for each segment
pub fn median(data: &[f64], time_interval: usize) -> Vec<f64> {
    if data.is_empty() || time_interval == 0 {
        return Vec::new();
    }

    let mut result = Vec::new();
    
    for chunk in data.chunks(time_interval) {
        let mut segment = chunk.to_vec();
        segment.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let median = if segment.len() % 2 == 0 {
            let mid = segment.len() / 2;
            (segment[mid - 1] + segment[mid]) / 2.0
        } else {
            segment[segment.len() / 2]
        };
        
        result.push(median);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_interpolation() {
        let values = [1.0, 2.0, 5.0, 10.0, 20.0];
        let result = linear_interpolation(&values, 2);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_simple_exponential_smoothing() {
        let values = [1.0, 3.0, 5.0, 7.0, 9.0];
        let result = simple_exponential_smoothing(&values, 0.5);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 1.0);
        // Alpha 0.5 means average of current and previous smoothed value
        assert!((result[1] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_median() {
        let data = [1.0, 5.0, 3.0, 7.0, 9.0, 2.0, 4.0, 8.0];
        let result = median(&data, 4);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 4.0);  // median of [1.0, 5.0, 3.0, 7.0]
        assert_eq!(result[1], 6.0);  // median of [9.0, 2.0, 4.0, 8.0]
    }
}