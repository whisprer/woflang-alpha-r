//! Neural network layers.
//!
//! Implements common layer types for building neural networks:
//! - Dense (fully connected)
//! - Conv2D (2D convolution)
//! - BatchNorm (batch normalization)
//! - Dropout (regularization)

use super::tensor::Tensor;
use super::activation::Activation;

// ═══════════════════════════════════════════════════════════════════════════
// LAYER TRAIT
// ═══════════════════════════════════════════════════════════════════════════

/// Trait for neural network layers.
pub trait Layer {
    /// Forward pass: compute output from input.
    fn forward(&mut self, input: &Tensor) -> Tensor;
    
    /// Backward pass: compute gradient with respect to input.
    fn backward(&mut self, grad_output: &Tensor) -> Tensor;
    
    /// Update weights using gradients.
    fn update(&mut self, learning_rate: f32);
    
    /// Get number of parameters.
    fn num_params(&self) -> usize;
    
    /// Get layer name.
    fn name(&self) -> &str;
}

// ═══════════════════════════════════════════════════════════════════════════
// DENSE LAYER (FULLY CONNECTED)
// ═══════════════════════════════════════════════════════════════════════════

/// Dense (fully connected) layer.
/// y = activation(x @ W + b)
pub struct Dense {
    /// Weight matrix [input_size, output_size]
    pub weights: Tensor,
    /// Bias vector [output_size]
    pub bias: Tensor,
    /// Activation function
    pub activation: Activation,
    
    // Cached for backward pass
    input_cache: Option<Tensor>,
    pre_activation: Option<Tensor>,
    
    // Gradients
    weight_grad: Tensor,
    bias_grad: Tensor,
}

impl Dense {
    /// Create a new dense layer.
    pub fn new(input_size: usize, output_size: usize, activation: Activation) -> Self {
        Dense {
            weights: Tensor::xavier(&[input_size, output_size]),
            bias: Tensor::zeros(&[output_size]),
            activation,
            input_cache: None,
            pre_activation: None,
            weight_grad: Tensor::zeros(&[input_size, output_size]),
            bias_grad: Tensor::zeros(&[output_size]),
        }
    }

    /// Create with specific weights.
    pub fn with_weights(weights: Tensor, bias: Tensor, activation: Activation) -> Self {
        let weight_shape = weights.shape.clone();
        let bias_shape = bias.shape.clone();
        
        Dense {
            weights,
            bias,
            activation,
            input_cache: None,
            pre_activation: None,
            weight_grad: Tensor::zeros(&weight_shape),
            bias_grad: Tensor::zeros(&bias_shape),
        }
    }
}

impl Layer for Dense {
    fn forward(&mut self, input: &Tensor) -> Tensor {
        // Store input for backward pass
        self.input_cache = Some(input.clone());
        
        // Compute linear transformation
        let linear = if input.ndim() == 1 {
            // Single sample: reshape to row vector, matmul, extract
            let input_row = input.reshape(&[1, input.shape[0]]);
            let out = input_row.matmul(&self.weights);
            out.reshape(&[self.bias.shape[0]])
        } else {
            // Batch: input is [batch, input_size]
            input.matmul(&self.weights)
        };
        
        // Add bias (broadcasting)
        let pre_act = if linear.ndim() == 1 {
            linear.add(&self.bias)
        } else {
            linear.add_broadcast_row(&self.bias)
        };
        
        self.pre_activation = Some(pre_act.clone());
        
        // Apply activation
        self.activation.apply(&pre_act)
    }

    fn backward(&mut self, grad_output: &Tensor) -> Tensor {
        let input = self.input_cache.as_ref().expect("Forward must be called first");
        let pre_act = self.pre_activation.as_ref().expect("Forward must be called first");
        
        // Gradient through activation
        let act_grad = self.activation.derivative(pre_act);
        let grad = grad_output.mul(&act_grad);
        
        // Gradient w.r.t. weights: input^T @ grad
        if input.ndim() == 1 {
            // Single sample
            self.weight_grad = input.outer(&grad);
        } else {
            // Batch
            let input_t = input.transpose();
            self.weight_grad = input_t.matmul(&grad);
        }
        
        // Gradient w.r.t. bias: sum over batch
        if grad.ndim() == 1 {
            self.bias_grad = grad.clone();
        } else {
            self.bias_grad = grad.sum_axis(0);
        }
        
        // Gradient w.r.t. input: grad @ W^T
        let weights_t = self.weights.transpose();
        if grad.ndim() == 1 {
            let grad_row = grad.reshape(&[1, grad.shape[0]]);
            let result = grad_row.matmul(&weights_t);
            result.reshape(&[self.weights.shape[0]])
        } else {
            grad.matmul(&weights_t)
        }
    }

    fn update(&mut self, learning_rate: f32) {
        // SGD update: W = W - lr * grad
        let w_update = self.weight_grad.scale(learning_rate);
        self.weights = self.weights.sub(&w_update);
        
        let b_update = self.bias_grad.scale(learning_rate);
        self.bias = self.bias.sub(&b_update);
    }

    fn num_params(&self) -> usize {
        self.weights.size() + self.bias.size()
    }

    fn name(&self) -> &str {
        "Dense"
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CONV2D LAYER
// ═══════════════════════════════════════════════════════════════════════════

/// 2D Convolutional layer.
pub struct Conv2D {
    /// Filters [num_filters, kernel_h, kernel_w]
    pub filters: Vec<Tensor>,
    /// Bias for each filter
    pub biases: Vec<f32>,
    /// Stride
    pub stride: usize,
    /// Activation function
    pub activation: Activation,
    
    // Cached for backward
    input_cache: Option<Tensor>,
    
    // Gradients
    filter_grads: Vec<Tensor>,
    bias_grads: Vec<f32>,
}

impl Conv2D {
    /// Create a new Conv2D layer.
    pub fn new(
        num_filters: usize,
        kernel_size: usize,
        stride: usize,
        activation: Activation,
    ) -> Self {
        let filters: Vec<Tensor> = (0..num_filters)
            .map(|_| Tensor::xavier(&[kernel_size, kernel_size]))
            .collect();
        
        let filter_grads: Vec<Tensor> = (0..num_filters)
            .map(|_| Tensor::zeros(&[kernel_size, kernel_size]))
            .collect();
        
        Conv2D {
            filters,
            biases: vec![0.0; num_filters],
            stride,
            activation,
            input_cache: None,
            filter_grads,
            bias_grads: vec![0.0; num_filters],
        }
    }
}

impl Layer for Conv2D {
    fn forward(&mut self, input: &Tensor) -> Tensor {
        self.input_cache = Some(input.clone());
        
        // Assuming input is [height, width]
        let (h, w) = (input.shape[0], input.shape[1]);
        let k = self.filters[0].shape[0];
        
        let out_h = (h - k) / self.stride + 1;
        let out_w = (w - k) / self.stride + 1;
        let num_filters = self.filters.len();
        
        // Output is [num_filters, out_h, out_w]
        let mut output_data = Vec::with_capacity(num_filters * out_h * out_w);
        
        for (f_idx, filter) in self.filters.iter().enumerate() {
            let conv_out = input.conv2d(filter, self.stride);
            let bias = self.biases[f_idx];
            
            for &val in &conv_out.data {
                output_data.push(val + bias);
            }
        }
        
        let output = Tensor::from_data(output_data, &[num_filters, out_h, out_w]);
        self.activation.apply(&output)
    }

    fn backward(&mut self, grad_output: &Tensor) -> Tensor {
        // Simplified backward pass
        // In a full implementation, we'd compute gradients for filters and input
        
        let input = self.input_cache.as_ref().expect("Forward must be called first");
        
        // For now, return zeros (placeholder)
        Tensor::zeros(&input.shape)
    }

    fn update(&mut self, learning_rate: f32) {
        for (filter, grad) in self.filters.iter_mut().zip(&self.filter_grads) {
            let update = grad.scale(learning_rate);
            *filter = filter.sub(&update);
        }
        
        for (bias, &grad) in self.biases.iter_mut().zip(&self.bias_grads) {
            *bias -= learning_rate * grad;
        }
    }

    fn num_params(&self) -> usize {
        self.filters.iter().map(|f| f.size()).sum::<usize>() + self.biases.len()
    }

    fn name(&self) -> &str {
        "Conv2D"
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BATCH NORMALIZATION
// ═══════════════════════════════════════════════════════════════════════════

/// Batch normalization layer.
pub struct BatchNorm {
    /// Learnable scale parameter
    pub gamma: Tensor,
    /// Learnable shift parameter
    pub beta: Tensor,
    /// Running mean (for inference)
    pub running_mean: Tensor,
    /// Running variance (for inference)
    pub running_var: Tensor,
    /// Momentum for running stats
    pub momentum: f32,
    /// Small constant for numerical stability
    pub epsilon: f32,
    /// Whether in training mode
    pub training: bool,
    
    // Cached for backward
    input_normalized: Option<Tensor>,
    std_dev: Option<Tensor>,
    
    // Gradients
    gamma_grad: Tensor,
    beta_grad: Tensor,
}

impl BatchNorm {
    /// Create a new batch normalization layer.
    pub fn new(num_features: usize) -> Self {
        BatchNorm {
            gamma: Tensor::ones(&[num_features]),
            beta: Tensor::zeros(&[num_features]),
            running_mean: Tensor::zeros(&[num_features]),
            running_var: Tensor::ones(&[num_features]),
            momentum: 0.1,
            epsilon: 1e-5,
            training: true,
            input_normalized: None,
            std_dev: None,
            gamma_grad: Tensor::zeros(&[num_features]),
            beta_grad: Tensor::zeros(&[num_features]),
        }
    }

    /// Set training mode.
    pub fn train(&mut self, training: bool) {
        self.training = training;
    }
}

impl Layer for BatchNorm {
    fn forward(&mut self, input: &Tensor) -> Tensor {
        if input.ndim() == 1 {
            // Single sample: just normalize using running stats
            let normalized = input.sub(&self.running_mean);
            let var_sqrt = self.running_var.add_scalar(self.epsilon).sqrt();
            let normalized = normalized.div(&var_sqrt);
            normalized.mul(&self.gamma).add(&self.beta)
        } else {
            // Batch: compute batch statistics
            let batch_size = input.shape[0] as f32;
            let features = input.shape[1];
            
            // Compute mean over batch
            let mut mean_data = vec![0.0; features];
            for i in 0..input.shape[0] {
                for j in 0..features {
                    mean_data[j] += input.get(&[i, j]);
                }
            }
            for m in &mut mean_data {
                *m /= batch_size;
            }
            let mean = Tensor::vector(mean_data);
            
            // Compute variance over batch
            let mut var_data = vec![0.0; features];
            for i in 0..input.shape[0] {
                for j in 0..features {
                    let diff = input.get(&[i, j]) - mean.data[j];
                    var_data[j] += diff * diff;
                }
            }
            for v in &mut var_data {
                *v /= batch_size;
            }
            let var = Tensor::vector(var_data);
            
            // Update running statistics
            if self.training {
                for i in 0..features {
                    self.running_mean.data[i] = 
                        (1.0 - self.momentum) * self.running_mean.data[i] + 
                        self.momentum * mean.data[i];
                    self.running_var.data[i] = 
                        (1.0 - self.momentum) * self.running_var.data[i] + 
                        self.momentum * var.data[i];
                }
            }
            
            // Normalize
            let std = var.add_scalar(self.epsilon).sqrt();
            self.std_dev = Some(std.clone());
            
            let mut normalized = Tensor::zeros(&input.shape);
            for i in 0..input.shape[0] {
                for j in 0..features {
                    let norm = (input.get(&[i, j]) - mean.data[j]) / std.data[j];
                    normalized.set(&[i, j], norm);
                }
            }
            
            self.input_normalized = Some(normalized.clone());
            
            // Scale and shift
            let mut output = Tensor::zeros(&input.shape);
            for i in 0..input.shape[0] {
                for j in 0..features {
                    let val = self.gamma.data[j] * normalized.get(&[i, j]) + self.beta.data[j];
                    output.set(&[i, j], val);
                }
            }
            
            output
        }
    }

    fn backward(&mut self, grad_output: &Tensor) -> Tensor {
        // Simplified backward
        if let Some(ref normalized) = self.input_normalized {
            // Gradient w.r.t gamma
            self.gamma_grad = grad_output.mul(normalized).sum_axis(0);
            // Gradient w.r.t beta
            self.beta_grad = grad_output.sum_axis(0);
        }
        
        grad_output.clone()
    }

    fn update(&mut self, learning_rate: f32) {
        let g_update = self.gamma_grad.scale(learning_rate);
        self.gamma = self.gamma.sub(&g_update);
        
        let b_update = self.beta_grad.scale(learning_rate);
        self.beta = self.beta.sub(&b_update);
    }

    fn num_params(&self) -> usize {
        self.gamma.size() + self.beta.size()
    }

    fn name(&self) -> &str {
        "BatchNorm"
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DROPOUT LAYER
// ═══════════════════════════════════════════════════════════════════════════

/// Dropout layer for regularization.
pub struct Dropout {
    /// Dropout probability (fraction to drop)
    pub p: f32,
    /// Whether in training mode
    pub training: bool,
    /// Mask from last forward pass
    mask: Option<Tensor>,
    /// Random seed for reproducibility
    seed: u64,
}

impl Dropout {
    /// Create a new dropout layer.
    pub fn new(p: f32) -> Self {
        Dropout {
            p,
            training: true,
            mask: None,
            seed: 0xCAFE_BABE,
        }
    }

    /// Set training mode.
    pub fn train(&mut self, training: bool) {
        self.training = training;
    }
}

impl Layer for Dropout {
    fn forward(&mut self, input: &Tensor) -> Tensor {
        if !self.training || self.p == 0.0 {
            return input.clone();
        }
        
        // Generate mask
        let mut mask_data = Vec::with_capacity(input.size());
        let scale = 1.0 / (1.0 - self.p);
        
        for _i in 0..input.size() {
            self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let rand_val = ((self.seed >> 33) as f32) / (u32::MAX as f32);
            
            if rand_val > self.p {
                mask_data.push(scale);
            } else {
                mask_data.push(0.0);
            }
        }
        
        let mask = Tensor::from_data(mask_data, &input.shape);
        self.mask = Some(mask.clone());
        
        input.mul(&mask)
    }

    fn backward(&mut self, grad_output: &Tensor) -> Tensor {
        if let Some(ref mask) = self.mask {
            grad_output.mul(mask)
        } else {
            grad_output.clone()
        }
    }

    fn update(&mut self, _learning_rate: f32) {
        // No parameters to update
    }

    fn num_params(&self) -> usize {
        0
    }

    fn name(&self) -> &str {
        "Dropout"
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// FLATTEN LAYER
// ═══════════════════════════════════════════════════════════════════════════

/// Flatten layer (reshape to 1D).
pub struct Flatten {
    original_shape: Option<Vec<usize>>,
}

impl Flatten {
    pub fn new() -> Self {
        Flatten { original_shape: None }
    }
}

impl Default for Flatten {
    fn default() -> Self {
        Self::new()
    }
}

impl Layer for Flatten {
    fn forward(&mut self, input: &Tensor) -> Tensor {
        self.original_shape = Some(input.shape.clone());
        input.flatten()
    }

    fn backward(&mut self, grad_output: &Tensor) -> Tensor {
        if let Some(ref shape) = self.original_shape {
            grad_output.reshape(shape)
        } else {
            grad_output.clone()
        }
    }

    fn update(&mut self, _learning_rate: f32) {}
    
    fn num_params(&self) -> usize { 0 }
    
    fn name(&self) -> &str { "Flatten" }
}

// ═══════════════════════════════════════════════════════════════════════════
// EMBEDDING LAYER
// ═══════════════════════════════════════════════════════════════════════════

/// Embedding layer for looking up vectors from indices.
pub struct Embedding {
    /// Embedding matrix [vocab_size, embedding_dim]
    pub embeddings: Tensor,
    /// Gradient accumulator
    grad: Tensor,
    /// Last input indices
    last_input: Option<Vec<usize>>,
}

impl Embedding {
    /// Create a new embedding layer.
    pub fn new(vocab_size: usize, embedding_dim: usize) -> Self {
        Embedding {
            embeddings: Tensor::xavier(&[vocab_size, embedding_dim]),
            grad: Tensor::zeros(&[vocab_size, embedding_dim]),
            last_input: None,
        }
    }

    /// Look up embedding for a single index.
    pub fn lookup(&self, index: usize) -> Tensor {
        let dim = self.embeddings.shape[1];
        let start = index * dim;
        let data: Vec<f32> = self.embeddings.data[start..start + dim].to_vec();
        Tensor::vector(data)
    }

    /// Look up embeddings for multiple indices.
    pub fn lookup_batch(&mut self, indices: &[usize]) -> Tensor {
        self.last_input = Some(indices.to_vec());
        
        let dim = self.embeddings.shape[1];
        let batch_size = indices.len();
        let mut data = Vec::with_capacity(batch_size * dim);
        
        for &idx in indices {
            let start = idx * dim;
            data.extend_from_slice(&self.embeddings.data[start..start + dim]);
        }
        
        Tensor::from_data(data, &[batch_size, dim])
    }
}

impl Layer for Embedding {
    fn forward(&mut self, input: &Tensor) -> Tensor {
        // Assume input contains indices as integers
        let indices: Vec<usize> = input.data.iter().map(|&x| x as usize).collect();
        self.lookup_batch(&indices)
    }

    fn backward(&mut self, grad_output: &Tensor) -> Tensor {
        // Accumulate gradients for embeddings
        if let Some(ref indices) = self.last_input {
            let dim = self.embeddings.shape[1];
            
            for (i, &idx) in indices.iter().enumerate() {
                let start = idx * dim;
                for j in 0..dim {
                    self.grad.data[start + j] += grad_output.get(&[i, j]);
                }
            }
        }
        
        // No input gradient for embedding layer
        Tensor::zeros(&[1])
    }

    fn update(&mut self, learning_rate: f32) {
        let update = self.grad.scale(learning_rate);
        self.embeddings = self.embeddings.sub(&update);
        
        // Reset gradients
        for x in &mut self.grad.data {
            *x = 0.0;
        }
    }

    fn num_params(&self) -> usize {
        self.embeddings.size()
    }

    fn name(&self) -> &str {
        "Embedding"
    }
}
