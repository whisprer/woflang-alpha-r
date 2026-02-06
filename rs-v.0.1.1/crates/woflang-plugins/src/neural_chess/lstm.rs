//! Long Short-Term Memory (LSTM) implementation.
//!
//! LSTM networks are crucial for capturing long-term dependencies
//! in chess games - remembering opening theory, tracking piece exchanges,
//! and planning multi-move strategies.

use super::tensor::Tensor;
use super::activation::{sigmoid, tanh};

// ═══════════════════════════════════════════════════════════════════════════
// LSTM CELL
// ═══════════════════════════════════════════════════════════════════════════

/// LSTM Cell with forget, input, and output gates.
///
/// Equations:
/// f_t = σ(W_xf @ x_t + W_hf @ h_{t-1} + b_f)  # forget gate
/// i_t = σ(W_xi @ x_t + W_hi @ h_{t-1} + b_i)  # input gate
/// o_t = σ(W_xo @ x_t + W_ho @ h_{t-1} + b_o)  # output gate
/// c̃_t = tanh(W_xc @ x_t + W_hc @ h_{t-1} + b_c)  # candidate cell
/// c_t = f_t * c_{t-1} + i_t * c̃_t  # cell state
/// h_t = o_t * tanh(c_t)  # hidden state
pub struct LSTMCell {
    // Forget gate
    pub w_xf: Tensor,
    pub w_hf: Tensor,
    pub b_f: Tensor,
    
    // Input gate
    pub w_xi: Tensor,
    pub w_hi: Tensor,
    pub b_i: Tensor,
    
    // Output gate
    pub w_xo: Tensor,
    pub w_ho: Tensor,
    pub b_o: Tensor,
    
    // Cell candidate
    pub w_xc: Tensor,
    pub w_hc: Tensor,
    pub b_c: Tensor,
    
    pub hidden_size: usize,
    pub input_size: usize,
    
    // Gradients
    grad_w_xf: Tensor, grad_w_hf: Tensor, grad_b_f: Tensor,
    grad_w_xi: Tensor, grad_w_hi: Tensor, grad_b_i: Tensor,
    grad_w_xo: Tensor, grad_w_ho: Tensor, grad_b_o: Tensor,
    grad_w_xc: Tensor, grad_w_hc: Tensor, grad_b_c: Tensor,
    
    // Cache for backward pass
    cache: Vec<LSTMStepCache>,
}

/// Cached values for a single LSTM step (for backward pass).
#[derive(Clone)]
struct LSTMStepCache {
    x: Tensor,
    h_prev: Tensor,
    c_prev: Tensor,
    f: Tensor,
    i: Tensor,
    o: Tensor,
    c_candidate: Tensor,
    c: Tensor,
    h: Tensor,
}

impl LSTMCell {
    /// Create a new LSTM cell.
    pub fn new(input_size: usize, hidden_size: usize) -> Self {
        LSTMCell {
            // Forget gate
            w_xf: Tensor::xavier(&[input_size, hidden_size]),
            w_hf: Tensor::xavier(&[hidden_size, hidden_size]),
            b_f: Tensor::ones(&[hidden_size]),  // Initialize forget bias to 1 (remember by default)
            
            // Input gate
            w_xi: Tensor::xavier(&[input_size, hidden_size]),
            w_hi: Tensor::xavier(&[hidden_size, hidden_size]),
            b_i: Tensor::zeros(&[hidden_size]),
            
            // Output gate
            w_xo: Tensor::xavier(&[input_size, hidden_size]),
            w_ho: Tensor::xavier(&[hidden_size, hidden_size]),
            b_o: Tensor::zeros(&[hidden_size]),
            
            // Cell candidate
            w_xc: Tensor::xavier(&[input_size, hidden_size]),
            w_hc: Tensor::xavier(&[hidden_size, hidden_size]),
            b_c: Tensor::zeros(&[hidden_size]),
            
            hidden_size,
            input_size,
            
            // Initialize gradients
            grad_w_xf: Tensor::zeros(&[input_size, hidden_size]),
            grad_w_hf: Tensor::zeros(&[hidden_size, hidden_size]),
            grad_b_f: Tensor::zeros(&[hidden_size]),
            grad_w_xi: Tensor::zeros(&[input_size, hidden_size]),
            grad_w_hi: Tensor::zeros(&[hidden_size, hidden_size]),
            grad_b_i: Tensor::zeros(&[hidden_size]),
            grad_w_xo: Tensor::zeros(&[input_size, hidden_size]),
            grad_w_ho: Tensor::zeros(&[hidden_size, hidden_size]),
            grad_b_o: Tensor::zeros(&[hidden_size]),
            grad_w_xc: Tensor::zeros(&[input_size, hidden_size]),
            grad_w_hc: Tensor::zeros(&[hidden_size, hidden_size]),
            grad_b_c: Tensor::zeros(&[hidden_size]),
            
            cache: Vec::new(),
        }
    }

    /// Initialize hidden state (h, c) to zeros.
    pub fn init_state(&self) -> (Tensor, Tensor) {
        (
            Tensor::zeros(&[self.hidden_size]),  // h
            Tensor::zeros(&[self.hidden_size]),  // c
        )
    }

    /// Forward pass for single timestep.
    pub fn forward_step(&mut self, x: &Tensor, h_prev: &Tensor, c_prev: &Tensor) -> (Tensor, Tensor) {
        let x_row = x.reshape(&[1, self.input_size]);
        let h_row = h_prev.reshape(&[1, self.hidden_size]);
        
        // Forget gate: f = σ(W_xf @ x + W_hf @ h + b_f)
        let f_x = x_row.matmul(&self.w_xf).flatten();
        let f_h = h_row.matmul(&self.w_hf).flatten();
        let f_pre = f_x.add(&f_h).add(&self.b_f);
        let f = sigmoid(&f_pre);
        
        // Input gate: i = σ(W_xi @ x + W_hi @ h + b_i)
        let i_x = x_row.matmul(&self.w_xi).flatten();
        let i_h = h_row.matmul(&self.w_hi).flatten();
        let i_pre = i_x.add(&i_h).add(&self.b_i);
        let i = sigmoid(&i_pre);
        
        // Output gate: o = σ(W_xo @ x + W_ho @ h + b_o)
        let o_x = x_row.matmul(&self.w_xo).flatten();
        let o_h = h_row.matmul(&self.w_ho).flatten();
        let o_pre = o_x.add(&o_h).add(&self.b_o);
        let o = sigmoid(&o_pre);
        
        // Cell candidate: c̃ = tanh(W_xc @ x + W_hc @ h + b_c)
        let c_x = x_row.matmul(&self.w_xc).flatten();
        let c_h = h_row.matmul(&self.w_hc).flatten();
        let c_cand_pre = c_x.add(&c_h).add(&self.b_c);
        let c_candidate = tanh(&c_cand_pre);
        
        // Cell state: c = f * c_prev + i * c̃
        let forget_part = f.mul(c_prev);
        let input_part = i.mul(&c_candidate);
        let c = forget_part.add(&input_part);
        
        // Hidden state: h = o * tanh(c)
        let c_tanh = tanh(&c);
        let h = o.mul(&c_tanh);
        
        // Cache for backward pass
        self.cache.push(LSTMStepCache {
            x: x.clone(),
            h_prev: h_prev.clone(),
            c_prev: c_prev.clone(),
            f,
            i,
            o,
            c_candidate,
            c: c.clone(),
            h: h.clone(),
        });
        
        (h, c)
    }

    /// Forward pass over entire sequence.
    pub fn forward(&mut self, sequence: &[Tensor]) -> Vec<Tensor> {
        self.clear_cache();
        
        let (mut h, mut c) = self.init_state();
        let mut outputs = Vec::with_capacity(sequence.len());
        
        for x in sequence {
            let (h_new, c_new) = self.forward_step(x, &h, &c);
            outputs.push(h_new.clone());
            h = h_new;
            c = c_new;
        }
        
        outputs
    }

    /// Backward pass through time (BPTT).
    pub fn backward(&mut self, grad_outputs: &[Tensor]) -> Vec<Tensor> {
        let seq_len = grad_outputs.len();
        let mut grad_inputs = Vec::with_capacity(seq_len);
        
        // Initialize gradients for h and c
        let mut grad_h_next = Tensor::zeros(&[self.hidden_size]);
        let mut grad_c_next = Tensor::zeros(&[self.hidden_size]);
        
        // Backward through time
        for t in (0..seq_len).rev() {
            let grad_output = &grad_outputs[t];
            let cache = &self.cache[t];
            
            // Total gradient at this timestep
            let grad_h = grad_output.add(&grad_h_next);
            
            // Gradient through h = o * tanh(c)
            let c_tanh = tanh(&cache.c);
            let grad_o = grad_h.mul(&c_tanh);
            
            // d(tanh(c))/dc = 1 - tanh²(c)
            let tanh_deriv = c_tanh.square().scale(-1.0).add_scalar(1.0);
            let grad_c_from_h = grad_h.mul(&cache.o).mul(&tanh_deriv);
            let grad_c = grad_c_from_h.add(&grad_c_next);
            
            // Gradient through c = f * c_prev + i * c̃
            let grad_f = grad_c.mul(&cache.c_prev);
            let grad_i = grad_c.mul(&cache.c_candidate);
            let grad_c_candidate = grad_c.mul(&cache.i);
            grad_c_next = grad_c.mul(&cache.f);
            
            // Gradient through gates (sigmoid derivative: σ(x)(1-σ(x)))
            let sig_deriv_f = cache.f.mul(&cache.f.scale(-1.0).add_scalar(1.0));
            let sig_deriv_i = cache.i.mul(&cache.i.scale(-1.0).add_scalar(1.0));
            let sig_deriv_o = cache.o.mul(&cache.o.scale(-1.0).add_scalar(1.0));
            
            let grad_f_pre = grad_f.mul(&sig_deriv_f);
            let grad_i_pre = grad_i.mul(&sig_deriv_i);
            let grad_o_pre = grad_o.mul(&sig_deriv_o);
            
            // Gradient through c_candidate (tanh derivative)
            let tanh_deriv_c = cache.c_candidate.square().scale(-1.0).add_scalar(1.0);
            let grad_c_cand_pre = grad_c_candidate.mul(&tanh_deriv_c);
            
            // Accumulate weight gradients
            // Forget gate
            self.grad_w_xf = self.grad_w_xf.add(&cache.x.outer(&grad_f_pre));
            self.grad_w_hf = self.grad_w_hf.add(&cache.h_prev.outer(&grad_f_pre));
            self.grad_b_f = self.grad_b_f.add(&grad_f_pre);
            
            // Input gate
            self.grad_w_xi = self.grad_w_xi.add(&cache.x.outer(&grad_i_pre));
            self.grad_w_hi = self.grad_w_hi.add(&cache.h_prev.outer(&grad_i_pre));
            self.grad_b_i = self.grad_b_i.add(&grad_i_pre);
            
            // Output gate
            self.grad_w_xo = self.grad_w_xo.add(&cache.x.outer(&grad_o_pre));
            self.grad_w_ho = self.grad_w_ho.add(&cache.h_prev.outer(&grad_o_pre));
            self.grad_b_o = self.grad_b_o.add(&grad_o_pre);
            
            // Cell candidate
            self.grad_w_xc = self.grad_w_xc.add(&cache.x.outer(&grad_c_cand_pre));
            self.grad_w_hc = self.grad_w_hc.add(&cache.h_prev.outer(&grad_c_cand_pre));
            self.grad_b_c = self.grad_b_c.add(&grad_c_cand_pre);
            
            // Gradient w.r.t. input
            let mut grad_x = Tensor::zeros(&[self.input_size]);
            
            // Sum contributions from all gates
            let gates = [
                (&self.w_xf, &grad_f_pre),
                (&self.w_xi, &grad_i_pre),
                (&self.w_xo, &grad_o_pre),
                (&self.w_xc, &grad_c_cand_pre),
            ];
            
            for (w, g) in &gates {
                let w_t = w.transpose();
                let g_row = g.reshape(&[1, self.hidden_size]);
                let contrib = g_row.matmul(&w_t).flatten();
                grad_x = grad_x.add(&contrib);
            }
            
            grad_inputs.push(grad_x);
            
            // Gradient w.r.t. previous hidden state
            grad_h_next = Tensor::zeros(&[self.hidden_size]);
            let h_gates = [
                (&self.w_hf, &grad_f_pre),
                (&self.w_hi, &grad_i_pre),
                (&self.w_ho, &grad_o_pre),
                (&self.w_hc, &grad_c_cand_pre),
            ];
            
            for (w, g) in &h_gates {
                let w_t = w.transpose();
                let g_row = g.reshape(&[1, self.hidden_size]);
                let contrib = g_row.matmul(&w_t).flatten();
                grad_h_next = grad_h_next.add(&contrib);
            }
        }
        
        grad_inputs.reverse();
        grad_inputs
    }

    /// Update weights using gradients.
    pub fn update(&mut self, learning_rate: f32) {
        // Gradient clipping to prevent exploding gradients
        let clip_value = 5.0;
        
        let grads = [
            (&mut self.w_xf, &mut self.grad_w_xf),
            (&mut self.w_hf, &mut self.grad_w_hf),
            (&mut self.b_f, &mut self.grad_b_f),
            (&mut self.w_xi, &mut self.grad_w_xi),
            (&mut self.w_hi, &mut self.grad_w_hi),
            (&mut self.b_i, &mut self.grad_b_i),
            (&mut self.w_xo, &mut self.grad_w_xo),
            (&mut self.w_ho, &mut self.grad_w_ho),
            (&mut self.b_o, &mut self.grad_b_o),
            (&mut self.w_xc, &mut self.grad_w_xc),
            (&mut self.w_hc, &mut self.grad_w_hc),
            (&mut self.b_c, &mut self.grad_b_c),
        ];
        
        for (weight, grad) in grads {
            // Clip gradient
            let clipped = grad.clamp(-clip_value, clip_value);
            let update = clipped.scale(learning_rate);
            *weight = weight.sub(&update);
            
            // Reset gradient
            for x in &mut grad.data {
                *x = 0.0;
            }
        }
    }

    /// Clear cached values.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Number of parameters.
    pub fn num_params(&self) -> usize {
        // 4 gates, each with w_x, w_h, and b
        4 * (self.w_xf.size() + self.w_hf.size() + self.b_f.size())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LSTM LAYER (MULTI-LAYER)
// ═══════════════════════════════════════════════════════════════════════════

/// Multi-layer LSTM network.
pub struct LSTM {
    /// LSTM cells for each layer
    pub cells: Vec<LSTMCell>,
    /// Number of layers
    pub num_layers: usize,
    /// Hidden size
    pub hidden_size: usize,
}

impl LSTM {
    /// Create a new multi-layer LSTM.
    pub fn new(input_size: usize, hidden_size: usize, num_layers: usize) -> Self {
        let mut cells = Vec::with_capacity(num_layers);
        
        // First layer takes input_size
        cells.push(LSTMCell::new(input_size, hidden_size));
        
        // Subsequent layers take hidden_size
        for _ in 1..num_layers {
            cells.push(LSTMCell::new(hidden_size, hidden_size));
        }
        
        LSTM {
            cells,
            num_layers,
            hidden_size,
        }
    }

    /// Forward pass through all layers.
    pub fn forward(&mut self, sequence: &[Tensor]) -> Vec<Tensor> {
        let mut current_sequence: Vec<Tensor> = sequence.to_vec();
        
        for cell in &mut self.cells {
            current_sequence = cell.forward(&current_sequence);
        }
        
        current_sequence
    }

    /// Get final hidden state.
    pub fn get_final_hidden(&mut self, sequence: &[Tensor]) -> Tensor {
        let outputs = self.forward(sequence);
        outputs.last().cloned().unwrap_or_else(|| Tensor::zeros(&[self.hidden_size]))
    }

    /// Get all layer states (for more complex architectures).
    pub fn get_all_states(&mut self, sequence: &[Tensor]) -> Vec<Vec<Tensor>> {
        let mut all_outputs = Vec::with_capacity(self.num_layers);
        let mut current_sequence: Vec<Tensor> = sequence.to_vec();
        
        for cell in &mut self.cells {
            current_sequence = cell.forward(&current_sequence);
            all_outputs.push(current_sequence.clone());
        }
        
        all_outputs
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
// PEEPHOLE LSTM (ADVANCED VARIANT)
// ═══════════════════════════════════════════════════════════════════════════

/// Peephole LSTM - gates can look at the cell state.
/// This variant often performs better for precise timing tasks.
pub struct PeepholeLSTMCell {
    base: LSTMCell,
    
    /// Peephole connections: c -> gates
    pub p_f: Tensor,  // cell -> forget
    pub p_i: Tensor,  // cell -> input
    pub p_o: Tensor,  // cell -> output
}

impl PeepholeLSTMCell {
    /// Create a new peephole LSTM cell.
    pub fn new(input_size: usize, hidden_size: usize) -> Self {
        PeepholeLSTMCell {
            base: LSTMCell::new(input_size, hidden_size),
            p_f: Tensor::xavier(&[hidden_size]),
            p_i: Tensor::xavier(&[hidden_size]),
            p_o: Tensor::xavier(&[hidden_size]),
        }
    }

    /// Initialize state.
    pub fn init_state(&self) -> (Tensor, Tensor) {
        self.base.init_state()
    }

    /// Forward step with peephole connections.
    pub fn forward_step(&self, x: &Tensor, h_prev: &Tensor, c_prev: &Tensor) -> (Tensor, Tensor) {
        let x_row = x.reshape(&[1, self.base.input_size]);
        let h_row = h_prev.reshape(&[1, self.base.hidden_size]);
        
        // Forget gate with peephole: f = σ(W_xf @ x + W_hf @ h + p_f * c_prev + b_f)
        let f_x = x_row.matmul(&self.base.w_xf).flatten();
        let f_h = h_row.matmul(&self.base.w_hf).flatten();
        let f_c = self.p_f.mul(c_prev);
        let f_pre = f_x.add(&f_h).add(&f_c).add(&self.base.b_f);
        let f = sigmoid(&f_pre);
        
        // Input gate with peephole
        let i_x = x_row.matmul(&self.base.w_xi).flatten();
        let i_h = h_row.matmul(&self.base.w_hi).flatten();
        let i_c = self.p_i.mul(c_prev);
        let i_pre = i_x.add(&i_h).add(&i_c).add(&self.base.b_i);
        let i = sigmoid(&i_pre);
        
        // Cell candidate (no peephole)
        let c_x = x_row.matmul(&self.base.w_xc).flatten();
        let c_h = h_row.matmul(&self.base.w_hc).flatten();
        let c_cand_pre = c_x.add(&c_h).add(&self.base.b_c);
        let c_candidate = tanh(&c_cand_pre);
        
        // Cell state
        let forget_part = f.mul(c_prev);
        let input_part = i.mul(&c_candidate);
        let c = forget_part.add(&input_part);
        
        // Output gate with peephole (uses new cell state)
        let o_x = x_row.matmul(&self.base.w_xo).flatten();
        let o_h = h_row.matmul(&self.base.w_ho).flatten();
        let o_c = self.p_o.mul(&c);
        let o_pre = o_x.add(&o_h).add(&o_c).add(&self.base.b_o);
        let o = sigmoid(&o_pre);
        
        // Hidden state
        let c_tanh = tanh(&c);
        let h = o.mul(&c_tanh);
        
        (h, c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lstm_cell() {
        let mut cell = LSTMCell::new(4, 8);
        let x = Tensor::rand(&[4]);
        let (h, c) = cell.init_state();
        
        let (h_new, c_new) = cell.forward_step(&x, &h, &c);
        assert_eq!(h_new.shape, vec![8]);
        assert_eq!(c_new.shape, vec![8]);
    }

    #[test]
    fn test_lstm_sequence() {
        let mut lstm = LSTM::new(4, 8, 2);
        let sequence: Vec<Tensor> = (0..10).map(|_| Tensor::rand(&[4])).collect();
        
        let outputs = lstm.forward(&sequence);
        assert_eq!(outputs.len(), 10);
        assert_eq!(outputs[0].shape, vec![8]);
    }

    #[test]
    fn test_lstm_backward() {
        let mut cell = LSTMCell::new(4, 8);
        let sequence: Vec<Tensor> = (0..5).map(|_| Tensor::rand(&[4])).collect();
        
        let outputs = cell.forward(&sequence);
        let grads: Vec<Tensor> = (0..5).map(|_| Tensor::rand(&[8])).collect();
        
        let input_grads = cell.backward(&grads);
        assert_eq!(input_grads.len(), 5);
        assert_eq!(input_grads[0].shape, vec![4]);
    }
}
