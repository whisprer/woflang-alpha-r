//! Recurrent Neural Network (RNN) implementation.
//!
//! Implements vanilla RNN cells and networks for sequence processing.
//! Used for capturing move sequences and game history patterns.

use super::tensor::Tensor;
use super::activation::{tanh, tanh_derivative};

// ═══════════════════════════════════════════════════════════════════════════
// RNN CELL
// ═══════════════════════════════════════════════════════════════════════════

/// A single RNN cell.
/// h_t = tanh(W_xh @ x_t + W_hh @ h_{t-1} + b_h)
pub struct RNNCell {
    /// Input-to-hidden weights [input_size, hidden_size]
    pub w_xh: Tensor,
    /// Hidden-to-hidden weights [hidden_size, hidden_size]
    pub w_hh: Tensor,
    /// Hidden bias [hidden_size]
    pub b_h: Tensor,
    
    /// Hidden size
    pub hidden_size: usize,
    /// Input size
    pub input_size: usize,
    
    // Gradients
    w_xh_grad: Tensor,
    w_hh_grad: Tensor,
    b_h_grad: Tensor,
    
    // Cache for backward pass
    inputs: Vec<Tensor>,
    hiddens: Vec<Tensor>,
    pre_activations: Vec<Tensor>,
}

impl RNNCell {
    /// Create a new RNN cell.
    pub fn new(input_size: usize, hidden_size: usize) -> Self {
        RNNCell {
            w_xh: Tensor::xavier(&[input_size, hidden_size]),
            w_hh: Tensor::xavier(&[hidden_size, hidden_size]),
            b_h: Tensor::zeros(&[hidden_size]),
            hidden_size,
            input_size,
            w_xh_grad: Tensor::zeros(&[input_size, hidden_size]),
            w_hh_grad: Tensor::zeros(&[hidden_size, hidden_size]),
            b_h_grad: Tensor::zeros(&[hidden_size]),
            inputs: Vec::new(),
            hiddens: Vec::new(),
            pre_activations: Vec::new(),
        }
    }

    /// Initialize hidden state to zeros.
    pub fn init_hidden(&self) -> Tensor {
        Tensor::zeros(&[self.hidden_size])
    }

    /// Forward pass for single timestep.
    pub fn forward_step(&mut self, x: &Tensor, h_prev: &Tensor) -> Tensor {
        // Store for backward
        self.inputs.push(x.clone());
        self.hiddens.push(h_prev.clone());
        
        // x @ W_xh
        let x_row = x.reshape(&[1, self.input_size]);
        let xh = x_row.matmul(&self.w_xh).flatten();
        
        // h_prev @ W_hh
        let h_row = h_prev.reshape(&[1, self.hidden_size]);
        let hh = h_row.matmul(&self.w_hh).flatten();
        
        // xh + hh + b_h
        let pre_act = xh.add(&hh).add(&self.b_h);
        self.pre_activations.push(pre_act.clone());
        
        // tanh activation
        tanh(&pre_act)
    }

    /// Forward pass over entire sequence.
    pub fn forward(&mut self, sequence: &[Tensor]) -> Vec<Tensor> {
        self.clear_cache();
        
        let mut h = self.init_hidden();
        let mut outputs = Vec::with_capacity(sequence.len());
        
        for x in sequence {
            h = self.forward_step(x, &h);
            outputs.push(h.clone());
        }
        
        outputs
    }

    /// Backward pass through time (BPTT).
    pub fn backward(&mut self, grad_outputs: &[Tensor]) -> Vec<Tensor> {
        let seq_len = grad_outputs.len();
        let mut grad_inputs = Vec::with_capacity(seq_len);
        
        // Initialize gradient of hidden state
        let mut grad_h_next = Tensor::zeros(&[self.hidden_size]);
        
        // Backward through time
        for t in (0..seq_len).rev() {
            let grad_output = &grad_outputs[t];
            let pre_act = &self.pre_activations[t];
            let h_prev = &self.hiddens[t];
            let x = &self.inputs[t];
            
            // Total gradient at this timestep
            let grad_h = grad_output.add(&grad_h_next);
            
            // Gradient through tanh
            let tanh_deriv = tanh_derivative(pre_act);
            let grad_pre_act = grad_h.mul(&tanh_deriv);
            
            // Gradient w.r.t. bias
            self.b_h_grad = self.b_h_grad.add(&grad_pre_act);
            
            // Gradient w.r.t. W_xh
            let grad_w_xh = x.outer(&grad_pre_act);
            self.w_xh_grad = self.w_xh_grad.add(&grad_w_xh);
            
            // Gradient w.r.t. W_hh
            let grad_w_hh = h_prev.outer(&grad_pre_act);
            self.w_hh_grad = self.w_hh_grad.add(&grad_w_hh);
            
            // Gradient w.r.t. input
            let w_xh_t = self.w_xh.transpose();
            let grad_pre_row = grad_pre_act.reshape(&[1, self.hidden_size]);
            let grad_x = grad_pre_row.matmul(&w_xh_t).flatten();
            grad_inputs.push(grad_x);
            
            // Gradient w.r.t. previous hidden state
            let w_hh_t = self.w_hh.transpose();
            grad_h_next = grad_pre_row.matmul(&w_hh_t).flatten();
        }
        
        grad_inputs.reverse();
        grad_inputs
    }

    /// Update weights using gradients.
    pub fn update(&mut self, learning_rate: f32) {
        // SGD update
        let w_xh_update = self.w_xh_grad.scale(learning_rate);
        self.w_xh = self.w_xh.sub(&w_xh_update);
        
        let w_hh_update = self.w_hh_grad.scale(learning_rate);
        self.w_hh = self.w_hh.sub(&w_hh_update);
        
        let b_h_update = self.b_h_grad.scale(learning_rate);
        self.b_h = self.b_h.sub(&b_h_update);
        
        // Reset gradients
        self.w_xh_grad = Tensor::zeros(&[self.input_size, self.hidden_size]);
        self.w_hh_grad = Tensor::zeros(&[self.hidden_size, self.hidden_size]);
        self.b_h_grad = Tensor::zeros(&[self.hidden_size]);
    }

    /// Clear cached values.
    pub fn clear_cache(&mut self) {
        self.inputs.clear();
        self.hiddens.clear();
        self.pre_activations.clear();
    }

    /// Number of parameters.
    pub fn num_params(&self) -> usize {
        self.w_xh.size() + self.w_hh.size() + self.b_h.size()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// RNN LAYER
// ═══════════════════════════════════════════════════════════════════════════

/// Multi-layer RNN.
pub struct RNN {
    /// RNN cells for each layer
    pub cells: Vec<RNNCell>,
    /// Number of layers
    pub num_layers: usize,
    /// Hidden size
    pub hidden_size: usize,
    /// Whether bidirectional
    pub bidirectional: bool,
}

impl RNN {
    /// Create a new multi-layer RNN.
    pub fn new(input_size: usize, hidden_size: usize, num_layers: usize) -> Self {
        let mut cells = Vec::with_capacity(num_layers);
        
        // First layer takes input_size
        cells.push(RNNCell::new(input_size, hidden_size));
        
        // Subsequent layers take hidden_size
        for _ in 1..num_layers {
            cells.push(RNNCell::new(hidden_size, hidden_size));
        }
        
        RNN {
            cells,
            num_layers,
            hidden_size,
            bidirectional: false,
        }
    }

    /// Create bidirectional RNN.
    pub fn bidirectional(input_size: usize, hidden_size: usize, num_layers: usize) -> Self {
        let mut rnn = Self::new(input_size, hidden_size, num_layers);
        rnn.bidirectional = true;
        
        // Add backward cells
        rnn.cells.push(RNNCell::new(input_size, hidden_size));
        for _ in 1..num_layers {
            rnn.cells.push(RNNCell::new(hidden_size, hidden_size));
        }
        
        rnn
    }

    /// Forward pass through all layers.
    pub fn forward(&mut self, sequence: &[Tensor]) -> Vec<Tensor> {
        let mut current_sequence: Vec<Tensor> = sequence.to_vec();
        
        // Forward through each layer
        for cell in &mut self.cells[..self.num_layers] {
            current_sequence = cell.forward(&current_sequence);
        }
        
        // If bidirectional, also run backward
        if self.bidirectional {
            let mut reversed: Vec<Tensor> = sequence.iter().rev().cloned().collect();
            
            for cell in &mut self.cells[self.num_layers..] {
                reversed = cell.forward(&reversed);
            }
            
            // Concatenate forward and backward outputs
            for (fwd, bwd) in current_sequence.iter_mut().zip(reversed.iter().rev()) {
                let combined_data: Vec<f32> = fwd.data.iter()
                    .chain(bwd.data.iter())
                    .copied()
                    .collect();
                *fwd = Tensor::from_data(combined_data, &[self.hidden_size * 2]);
            }
        }
        
        current_sequence
    }

    /// Get the final hidden state.
    pub fn get_final_hidden(&mut self, sequence: &[Tensor]) -> Tensor {
        let outputs = self.forward(sequence);
        outputs.last().cloned().unwrap_or_else(|| Tensor::zeros(&[self.hidden_size]))
    }

    /// Update all cells.
    pub fn update(&mut self, learning_rate: f32) {
        for cell in &mut self.cells {
            cell.update(learning_rate);
        }
    }

    /// Clear all caches.
    pub fn clear_cache(&mut self) {
        for cell in &mut self.cells {
            cell.clear_cache();
        }
    }

    /// Total number of parameters.
    pub fn num_params(&self) -> usize {
        self.cells.iter().map(|c| c.num_params()).sum()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GRU CELL (GATED RECURRENT UNIT)
// ═══════════════════════════════════════════════════════════════════════════

/// GRU Cell - a more sophisticated RNN cell with gating.
/// z_t = σ(W_xz @ x_t + W_hz @ h_{t-1} + b_z)  # update gate
/// r_t = σ(W_xr @ x_t + W_hr @ h_{t-1} + b_r)  # reset gate
/// h_candidate = tanh(W_xh @ x_t + W_hh @ (r_t * h_{t-1}) + b_h)
/// h_t = (1 - z_t) * h_{t-1} + z_t * h_candidate
pub struct GRUCell {
    /// Update gate weights
    pub w_xz: Tensor,
    pub w_hz: Tensor,
    pub b_z: Tensor,
    
    /// Reset gate weights
    pub w_xr: Tensor,
    pub w_hr: Tensor,
    pub b_r: Tensor,
    
    /// Candidate hidden state weights
    pub w_xh: Tensor,
    pub w_hh: Tensor,
    pub b_h: Tensor,
    
    pub hidden_size: usize,
    pub input_size: usize,
}

impl GRUCell {
    /// Create a new GRU cell.
    pub fn new(input_size: usize, hidden_size: usize) -> Self {
        GRUCell {
            // Update gate
            w_xz: Tensor::xavier(&[input_size, hidden_size]),
            w_hz: Tensor::xavier(&[hidden_size, hidden_size]),
            b_z: Tensor::zeros(&[hidden_size]),
            
            // Reset gate
            w_xr: Tensor::xavier(&[input_size, hidden_size]),
            w_hr: Tensor::xavier(&[hidden_size, hidden_size]),
            b_r: Tensor::zeros(&[hidden_size]),
            
            // Candidate
            w_xh: Tensor::xavier(&[input_size, hidden_size]),
            w_hh: Tensor::xavier(&[hidden_size, hidden_size]),
            b_h: Tensor::zeros(&[hidden_size]),
            
            hidden_size,
            input_size,
        }
    }

    /// Initialize hidden state.
    pub fn init_hidden(&self) -> Tensor {
        Tensor::zeros(&[self.hidden_size])
    }

    /// Forward pass for single timestep.
    pub fn forward_step(&self, x: &Tensor, h_prev: &Tensor) -> Tensor {
        use super::activation::sigmoid;
        
        let x_row = x.reshape(&[1, self.input_size]);
        let h_row = h_prev.reshape(&[1, self.hidden_size]);
        
        // Update gate: z = σ(W_xz @ x + W_hz @ h + b_z)
        let z_x = x_row.matmul(&self.w_xz).flatten();
        let z_h = h_row.matmul(&self.w_hz).flatten();
        let z_pre = z_x.add(&z_h).add(&self.b_z);
        let z = sigmoid(&z_pre);
        
        // Reset gate: r = σ(W_xr @ x + W_hr @ h + b_r)
        let r_x = x_row.matmul(&self.w_xr).flatten();
        let r_h = h_row.matmul(&self.w_hr).flatten();
        let r_pre = r_x.add(&r_h).add(&self.b_r);
        let r = sigmoid(&r_pre);
        
        // Candidate: h_cand = tanh(W_xh @ x + W_hh @ (r * h) + b_h)
        let h_x = x_row.matmul(&self.w_xh).flatten();
        let r_h_mul = r.mul(h_prev);
        let r_h_row = r_h_mul.reshape(&[1, self.hidden_size]);
        let h_h = r_h_row.matmul(&self.w_hh).flatten();
        let h_cand_pre = h_x.add(&h_h).add(&self.b_h);
        let h_cand = tanh(&h_cand_pre);
        
        // New hidden: h = (1 - z) * h_prev + z * h_cand
        let one_minus_z = z.scale(-1.0).add_scalar(1.0);
        let h_keep = one_minus_z.mul(h_prev);
        let h_update = z.mul(&h_cand);
        h_keep.add(&h_update)
    }

    /// Forward over sequence.
    pub fn forward(&self, sequence: &[Tensor]) -> Vec<Tensor> {
        let mut h = self.init_hidden();
        let mut outputs = Vec::with_capacity(sequence.len());
        
        for x in sequence {
            h = self.forward_step(x, &h);
            outputs.push(h.clone());
        }
        
        outputs
    }

    /// Number of parameters.
    pub fn num_params(&self) -> usize {
        // 3 gates, each with w_x, w_h, and b
        3 * (self.w_xz.size() + self.w_hz.size() + self.b_z.size())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rnn_cell() {
        let mut cell = RNNCell::new(4, 8);
        let x = Tensor::rand(&[4]);
        let h = cell.init_hidden();
        
        let h_new = cell.forward_step(&x, &h);
        assert_eq!(h_new.shape, vec![8]);
    }

    #[test]
    fn test_rnn_sequence() {
        let mut rnn = RNN::new(4, 8, 2);
        let sequence: Vec<Tensor> = (0..5).map(|_| Tensor::rand(&[4])).collect();
        
        let outputs = rnn.forward(&sequence);
        assert_eq!(outputs.len(), 5);
        assert_eq!(outputs[0].shape, vec![8]);
    }

    #[test]
    fn test_gru_cell() {
        let cell = GRUCell::new(4, 8);
        let x = Tensor::rand(&[4]);
        let h = cell.init_hidden();
        
        let h_new = cell.forward_step(&x, &h);
        assert_eq!(h_new.shape, vec![8]);
    }
}
