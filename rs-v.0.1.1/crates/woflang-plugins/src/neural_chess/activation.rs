//! Activation functions for neural networks.
//!
//! Implements common activation functions and their derivatives
//! for use in forward and backward passes.

use super::tensor::Tensor;

// ═══════════════════════════════════════════════════════════════════════════
// ACTIVATION FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// ReLU (Rectified Linear Unit): max(0, x)
pub fn relu(x: &Tensor) -> Tensor {
    let data: Vec<f32> = x.data.iter().map(|&v| v.max(0.0)).collect();
    Tensor::from_data(data, &x.shape)
}

/// ReLU derivative: 1 if x > 0, else 0
pub fn relu_derivative(x: &Tensor) -> Tensor {
    let data: Vec<f32> = x.data.iter().map(|&v| if v > 0.0 { 1.0 } else { 0.0 }).collect();
    Tensor::from_data(data, &x.shape)
}

/// Leaky ReLU: x if x > 0, else alpha * x
pub fn leaky_relu(x: &Tensor, alpha: f32) -> Tensor {
    let data: Vec<f32> = x.data.iter().map(|&v| if v > 0.0 { v } else { alpha * v }).collect();
    Tensor::from_data(data, &x.shape)
}

/// Leaky ReLU derivative
pub fn leaky_relu_derivative(x: &Tensor, alpha: f32) -> Tensor {
    let data: Vec<f32> = x.data.iter().map(|&v| if v > 0.0 { 1.0 } else { alpha }).collect();
    Tensor::from_data(data, &x.shape)
}

/// Sigmoid: 1 / (1 + exp(-x))
pub fn sigmoid(x: &Tensor) -> Tensor {
    let data: Vec<f32> = x.data.iter().map(|&v| {
        // Numerically stable sigmoid
        if v >= 0.0 {
            let e = (-v).exp();
            1.0 / (1.0 + e)
        } else {
            let e = v.exp();
            e / (1.0 + e)
        }
    }).collect();
    Tensor::from_data(data, &x.shape)
}

/// Sigmoid derivative: sigmoid(x) * (1 - sigmoid(x))
pub fn sigmoid_derivative(x: &Tensor) -> Tensor {
    let sig = sigmoid(x);
    let one_minus = sig.scale(-1.0).add_scalar(1.0);
    sig.mul(&one_minus)
}

/// Tanh: (exp(x) - exp(-x)) / (exp(x) + exp(-x))
pub fn tanh(x: &Tensor) -> Tensor {
    let data: Vec<f32> = x.data.iter().map(|&v| v.tanh()).collect();
    Tensor::from_data(data, &x.shape)
}

/// Tanh derivative: 1 - tanh²(x)
pub fn tanh_derivative(x: &Tensor) -> Tensor {
    let t = tanh(x);
    let t_squared = t.square();
    t_squared.scale(-1.0).add_scalar(1.0)
}

/// Softmax: exp(x_i) / sum(exp(x_j))
/// Applied along last dimension.
pub fn softmax(x: &Tensor) -> Tensor {
    // For numerical stability, subtract max before exp
    let max_val = x.max();
    let shifted = x.add_scalar(-max_val);
    let exp_x = shifted.exp();
    let sum = exp_x.sum();
    exp_x.scale(1.0 / sum)
}

/// Softmax for 2D tensor (apply softmax to each row).
pub fn softmax_2d(x: &Tensor) -> Tensor {
    assert_eq!(x.ndim(), 2, "softmax_2d requires 2D tensor");
    
    let (rows, cols) = (x.shape[0], x.shape[1]);
    let mut result = Tensor::zeros(&x.shape);
    
    for i in 0..rows {
        // Get row
        let row_start = i * cols;
        let row_data: Vec<f32> = x.data[row_start..row_start + cols].to_vec();
        let row = Tensor::vector(row_data);
        
        // Apply softmax
        let soft_row = softmax(&row);
        
        // Copy back
        for j in 0..cols {
            result.data[row_start + j] = soft_row.data[j];
        }
    }
    
    result
}

/// Log softmax (more numerically stable for cross-entropy).
pub fn log_softmax(x: &Tensor) -> Tensor {
    let max_val = x.max();
    let shifted = x.add_scalar(-max_val);
    let exp_x = shifted.exp();
    let sum = exp_x.sum();
    shifted.add_scalar(-sum.ln())
}

/// GELU (Gaussian Error Linear Unit): x * Φ(x)
/// Approximation: 0.5 * x * (1 + tanh(sqrt(2/π) * (x + 0.044715 * x³)))
pub fn gelu(x: &Tensor) -> Tensor {
    let sqrt_2_pi = (2.0 / std::f32::consts::PI).sqrt();
    let data: Vec<f32> = x.data.iter().map(|&v| {
        let inner = sqrt_2_pi * (v + 0.044715 * v * v * v);
        0.5 * v * (1.0 + inner.tanh())
    }).collect();
    Tensor::from_data(data, &x.shape)
}

/// Swish: x * sigmoid(x)
pub fn swish(x: &Tensor) -> Tensor {
    let sig = sigmoid(x);
    x.mul(&sig)
}

/// ELU (Exponential Linear Unit)
pub fn elu(x: &Tensor, alpha: f32) -> Tensor {
    let data: Vec<f32> = x.data.iter().map(|&v| {
        if v > 0.0 { v } else { alpha * (v.exp() - 1.0) }
    }).collect();
    Tensor::from_data(data, &x.shape)
}

/// Hard sigmoid (fast approximation)
pub fn hard_sigmoid(x: &Tensor) -> Tensor {
    let data: Vec<f32> = x.data.iter().map(|&v| {
        ((v * 0.2 + 0.5) as f32).clamp(0.0, 1.0)
    }).collect();
    Tensor::from_data(data, &x.shape)
}

/// Hard tanh (fast approximation)
pub fn hard_tanh(x: &Tensor) -> Tensor {
    let data: Vec<f32> = x.data.iter().map(|&v| v.clamp(-1.0, 1.0)).collect();
    Tensor::from_data(data, &x.shape)
}

// ═══════════════════════════════════════════════════════════════════════════
// ACTIVATION ENUM
// ═══════════════════════════════════════════════════════════════════════════

/// Enumeration of activation functions for layer configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Activation {
    None,
    ReLU,
    LeakyReLU(f32),
    Sigmoid,
    Tanh,
    Softmax,
    GELU,
    Swish,
    ELU(f32),
}

impl Activation {
    /// Apply the activation function.
    pub fn apply(&self, x: &Tensor) -> Tensor {
        match self {
            Activation::None => x.clone(),
            Activation::ReLU => relu(x),
            Activation::LeakyReLU(alpha) => leaky_relu(x, *alpha),
            Activation::Sigmoid => sigmoid(x),
            Activation::Tanh => tanh(x),
            Activation::Softmax => softmax(x),
            Activation::GELU => gelu(x),
            Activation::Swish => swish(x),
            Activation::ELU(alpha) => elu(x, *alpha),
        }
    }

    /// Apply the activation derivative.
    pub fn derivative(&self, x: &Tensor) -> Tensor {
        match self {
            Activation::None => Tensor::ones(&x.shape),
            Activation::ReLU => relu_derivative(x),
            Activation::LeakyReLU(alpha) => leaky_relu_derivative(x, *alpha),
            Activation::Sigmoid => sigmoid_derivative(x),
            Activation::Tanh => tanh_derivative(x),
            Activation::Softmax => Tensor::ones(&x.shape), // Handled specially in loss
            Activation::GELU => Tensor::ones(&x.shape),    // Approximate
            Activation::Swish => Tensor::ones(&x.shape),   // Approximate
            Activation::ELU(_) => Tensor::ones(&x.shape),  // Approximate
        }
    }
}

impl Default for Activation {
    fn default() -> Self {
        Activation::ReLU
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LOSS FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Mean Squared Error: (1/n) * Σ(y_pred - y_true)²
pub fn mse_loss(y_pred: &Tensor, y_true: &Tensor) -> f32 {
    let diff = y_pred.sub(y_true);
    let squared = diff.square();
    squared.mean()
}

/// MSE gradient: 2 * (y_pred - y_true) / n
pub fn mse_gradient(y_pred: &Tensor, y_true: &Tensor) -> Tensor {
    let diff = y_pred.sub(y_true);
    let n = y_pred.size() as f32;
    diff.scale(2.0 / n)
}

/// Cross-entropy loss: -Σ(y_true * log(y_pred))
pub fn cross_entropy_loss(y_pred: &Tensor, y_true: &Tensor) -> f32 {
    let epsilon = 1e-15;
    let clipped = y_pred.clamp(epsilon, 1.0 - epsilon);
    let log_pred = clipped.ln();
    let product = y_true.mul(&log_pred);
    -product.sum()
}

/// Cross-entropy gradient (for softmax output).
pub fn cross_entropy_gradient(y_pred: &Tensor, y_true: &Tensor) -> Tensor {
    y_pred.sub(y_true)
}

/// Binary cross-entropy loss.
pub fn binary_cross_entropy_loss(y_pred: &Tensor, y_true: &Tensor) -> f32 {
    let epsilon = 1e-15;
    let clipped = y_pred.clamp(epsilon, 1.0 - epsilon);
    
    let mut sum = 0.0;
    for (pred, &true_val) in clipped.data.iter().zip(&y_true.data) {
        sum += true_val * pred.ln() + (1.0 - true_val) * (1.0 - pred).ln();
    }
    
    -sum / y_pred.size() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relu() {
        let x = Tensor::vector(vec![-1.0, 0.0, 1.0, 2.0]);
        let y = relu(&x);
        assert_eq!(y.data, vec![0.0, 0.0, 1.0, 2.0]);
    }

    #[test]
    fn test_sigmoid() {
        let x = Tensor::vector(vec![0.0]);
        let y = sigmoid(&x);
        assert!((y.data[0] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_softmax() {
        let x = Tensor::vector(vec![1.0, 2.0, 3.0]);
        let y = softmax(&x);
        assert!((y.sum() - 1.0).abs() < 1e-6);
    }
}
