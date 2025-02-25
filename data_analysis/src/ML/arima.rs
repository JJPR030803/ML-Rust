use ndarray::{Array1, Array2, s};
use ndarray_linalg::Solve;

/// ARIMA model implementation
pub struct ARIMA {
    p: usize,  // AR order
    d: usize,  // Differencing order
    q: usize,  // MA order
    ar_params: Option<Array1<f64>>,  // AR parameters
    ma_params: Option<Array1<f64>>,  // MA parameters
    intercept: Option<f64>,  // Constant term
}

impl ARIMA {
    /// Create a new ARIMA model with specified orders
    pub fn new(p: usize, d: usize, q: usize) -> Self {
        ARIMA {
            p,
            d,
            q,
            ar_params: None,
            ma_params: None,
            intercept: None,
        }
    }
    
    /// Difference the time series to achieve stationarity
    fn difference(&self, series: &Array1<f64>) -> Array1<f64> {
        let mut result = series.clone();
        for _ in 0..self.d {
            let n = result.len();
            let mut temp = Array1::zeros(n - 1);
            for i in 0..n - 1 {
                temp[i] = result[i + 1] - result[i];
            }
            result = temp;
        }
        result
    }
    
    /// Reverse differencing to get original scale predictions
    fn undifference(&self, diff_preds: &Array1<f64>, orig_series: &Array1<f64>) -> Array1<f64> {
        let mut result = diff_preds.clone();
        
        for _ in 0..self.d {
            let n = result.len();
            let last_orig_idx = orig_series.len() - 1;
            let mut temp = Array1::zeros(n + 1);
            
            temp[0] = orig_series[last_orig_idx];
            for i in 0..n {
                temp[i + 1] = temp[i] + result[i];
            }
            
            result = temp.slice(s![1..]).to_owned();
        }
        
        result
    }
    
    /// Fit ARIMA model to the data using OLS for parameter estimation
    pub fn fit(&mut self, data: &Array1<f64>) -> Result<(), String> {
        if data.len() <= self.p + self.d + self.q {
            return Err("Not enough data points to fit the model".to_string());
        }
        
        // Perform differencing to achieve stationarity
        let diff_data = self.difference(data);
        
        // Construct matrices for OLS estimation
        let n = diff_data.len();
        let effective_n = n - self.p.max(self.q);
        
        if effective_n <= 0 {
            return Err("Not enough data points after accounting for lags".to_string());
        }
        
        // Design matrix X: includes AR terms, MA terms, and intercept
        let mut X = Array2::zeros((effective_n, self.p + self.q + 1));
        
        // Set intercept column (all ones)
        for i in 0..effective_n {
            X[[i, 0]] = 1.0;
        }
        
        // Fill AR terms
        for i in 0..effective_n {
            for j in 0..self.p {
                X[[i, j + 1]] = diff_data[i + self.p - j - 1];
            }
        }
        
        // Initialize residuals to zeros (will be updated iteratively)
        let mut residuals = Array1::zeros(n);
        
        // Iterative process for MA terms (since we don't know errors yet)
        for _ in 0..5 {  // Limit iterations for simplicity
            // Fill MA terms using current residuals
            for i in 0..effective_n {
                for j in 0..self.q {
                    let idx = i + self.p - j - 1;
                    if idx < residuals.len() {
                        X[[i, self.p + j + 1]] = residuals[idx];
                    }
                }
            }
            
            // Response vector y
            let y = diff_data.slice(s![self.p..]).to_owned();
            
            // Solve OLS: coefficients = (X'X)^(-1) X'y
            let X_t = X.t();
            let X_t_X = X_t.dot(&X);
            let X_t_y = X_t.dot(&y);
            
            let coefficients = match X_t_X.solve(&X_t_y) {
                Ok(c) => c,
                Err(_) => return Err("Failed to solve linear system".to_string()),
            };
            
            // Extract parameters
            self.intercept = Some(coefficients[0]);
            
            if self.p > 0 {
                self.ar_params = Some(coefficients.slice(s![1..=self.p]).to_owned());
            } else {
                self.ar_params = Some(Array1::zeros(0));
            }
            
            if self.q > 0 {
                self.ma_params = Some(coefficients.slice(s![self.p+1..]).to_owned());
            } else {
                self.ma_params = Some(Array1::zeros(0));
            }
            
            // Recompute residuals
            let fitted = self.predict_in_sample(&diff_data);
            for i in 0..fitted.len() {
                residuals[self.p + i] = diff_data[self.p + i] - fitted[i];
            }
        }
        
        Ok(())
    }
    
    /// Make in-sample predictions on the differenced data
    fn predict_in_sample(&self, diff_data: &Array1<f64>) -> Array1<f64> {
        let n = diff_data.len();
        let effective_n = n - self.p;
        let mut predictions = Array1::zeros(effective_n);
        
        // Use stored parameters for prediction
        let intercept = self.intercept.unwrap_or(0.0);
        let ar_params = self.ar_params.clone().unwrap_or_else(|| Array1::zeros(0));
        let ma_params = self.ma_params.clone().unwrap_or_else(|| Array1::zeros(0));
        
        // Initialize residuals to zeros
        let mut residuals = Array1::zeros(n);
        
        for i in 0..effective_n {
            // Start with intercept
            let mut pred = intercept;
            
            // Add AR component
            for j in 0..self.p {
                if j < ar_params.len() {
                    pred += ar_params[j] * diff_data[i + self.p - j - 1];
                }
            }
            
            // Add MA component
            for j in 0..self.q {
                if j < ma_params.len() && i >= j {
                    pred += ma_params[j] * residuals[i - j + self.p - 1];
                }
            }
            
            predictions[i] = pred;
            
            // Update residuals
            residuals[i + self.p] = diff_data[i + self.p] - pred;
        }
        
        predictions
    }
    
    /// Forecast future values
    pub fn forecast(&self, data: &Array1<f64>, steps: usize) -> Result<Array1<f64>, String> {
        if !self.is_fitted() {
            return Err("Model must be fitted before forecasting".to_string());
        }
        
        let diff_data = self.difference(data);
        let n = diff_data.len();
        
        // Use stored parameters
        let intercept = self.intercept.unwrap_or(0.0);
        let ar_params = self.ar_params.clone().unwrap_or_else(|| Array1::zeros(0));
        let ma_params = self.ma_params.clone().unwrap_or_else(|| Array1::zeros(0));
        
        // Latest observed values and errors for forecasting
        let mut latest_values = Vec::new();
        for i in (n - self.p.max(1))..n {
            latest_values.push(diff_data[i]);
        }
        
        // Calculate latest residuals
        let fitted = self.predict_in_sample(&diff_data);
        let mut latest_residuals = Vec::new();
        for i in (fitted.len() - self.q.max(1))..fitted.len() {
            latest_residuals.push(diff_data[i + self.p] - fitted[i]);
        }
        
        // Forecast on differenced data
        let mut diff_forecasts = Array1::zeros(steps);
        
        for i in 0..steps {
            // Start with intercept
            let mut forecast = intercept;
            
            // Add AR component
            for j in 0..self.p {
                if j < ar_params.len() {
                    let idx = if i <= j {
                        latest_values.len() - 1 - (j - i)
                    } else {
                        diff_forecasts.len() - 1 - (j - i)
                    };
                    
                    let value = if i <= j {
                        latest_values[idx]
                    } else {
                        diff_forecasts[i - j - 1]
                    };
                    
                    forecast += ar_params[j] * value;
                }
            }
            
            // Add MA component (only using known errors, not forecasting errors)
            for j in 0..self.q {
                if j < ma_params.len() && j < latest_residuals.len() && i <= j {
                    forecast += ma_params[j] * latest_residuals[latest_residuals.len() - 1 - (j - i)];
                }
            }
            
            diff_forecasts[i] = forecast;
        }
        
        // Transform forecasts back to original scale
        let diff_forecasts_arr = diff_forecasts.clone();
        let forecasts = self.undifference(&diff_forecasts_arr, data);
        
        Ok(forecasts)
    }
    
    /// Check if the model has been fitted
    fn is_fitted(&self) -> bool {
        self.ar_params.is_some() && self.ma_params.is_some() && self.intercept.is_some()
    }
}