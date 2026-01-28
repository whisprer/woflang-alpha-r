//! 3-Way GAN Brain Core for Chess AI.
//!
//! The Brain Core combines three neural network architectures:
//! - **CNN**: Spatial pattern recognition on the chess board
//! - **RNN**: Sequential move pattern recognition
//! - **LSTM**: Long-term strategic memory
//!
//! These are combined through a GAN-like architecture where:
//! - The Generator produces candidate moves
//! - The Discriminator evaluates position quality
//! - The three networks provide complementary information

use super::tensor::Tensor;
use super::activation::{softmax, Activation};
use super::layers::{Dense, Layer};
use super::rnn::RNN;
use super::lstm::LSTM;
use super::cnn::ChessCNN;
use super::ganglion::NeuralClockCoordinator;

// ═══════════════════════════════════════════════════════════════════════════
// FUSION LAYER
// ═══════════════════════════════════════════════════════════════════════════

/// Fuses outputs from CNN, RNN, and LSTM into a unified representation.
pub struct FusionLayer {
    /// CNN feature projection
    cnn_proj: Dense,
    /// RNN feature projection
    rnn_proj: Dense,
    /// LSTM feature projection
    lstm_proj: Dense,
    /// Attention weights
    attention: Dense,
    /// Final fusion projection
    fusion_proj: Dense,
    /// Output dimension
    pub output_dim: usize,
}

impl FusionLayer {
    /// Create a new fusion layer.
    pub fn new(cnn_dim: usize, rnn_dim: usize, lstm_dim: usize, output_dim: usize) -> Self {
        let hidden_dim = 256;
        
        FusionLayer {
            cnn_proj: Dense::new(cnn_dim, hidden_dim, Activation::ReLU),
            rnn_proj: Dense::new(rnn_dim, hidden_dim, Activation::ReLU),
            lstm_proj: Dense::new(lstm_dim, hidden_dim, Activation::ReLU),
            attention: Dense::new(hidden_dim * 3, 3, Activation::Softmax),
            fusion_proj: Dense::new(hidden_dim, output_dim, Activation::ReLU),
            output_dim,
        }
    }

    /// Fuse CNN, RNN, and LSTM features.
    pub fn forward(
        &mut self,
        cnn_features: &Tensor,
        rnn_features: &Tensor,
        lstm_features: &Tensor,
    ) -> Tensor {
        // Project each to common dimension
        let cnn_proj = self.cnn_proj.forward(cnn_features);
        let rnn_proj = self.rnn_proj.forward(rnn_features);
        let lstm_proj = self.lstm_proj.forward(lstm_features);
        
        // Concatenate for attention
        let concat_data: Vec<f32> = cnn_proj.data.iter()
            .chain(rnn_proj.data.iter())
            .chain(lstm_proj.data.iter())
            .copied()
            .collect();
        let concat = Tensor::from_data(concat_data, &[cnn_proj.size() * 3]);
        
        // Compute attention weights (3 weights summing to 1)
        let attn_weights = self.attention.forward(&concat);
        
        // Weighted combination
        let weighted_cnn = cnn_proj.scale(attn_weights.data[0]);
        let weighted_rnn = rnn_proj.scale(attn_weights.data[1]);
        let weighted_lstm = lstm_proj.scale(attn_weights.data[2]);
        
        let combined = weighted_cnn.add(&weighted_rnn).add(&weighted_lstm);
        
        // Final projection
        self.fusion_proj.forward(&combined)
    }

    /// Update weights.
    pub fn update(&mut self, learning_rate: f32) {
        self.cnn_proj.update(learning_rate);
        self.rnn_proj.update(learning_rate);
        self.lstm_proj.update(learning_rate);
        self.attention.update(learning_rate);
        self.fusion_proj.update(learning_rate);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MOVE GENERATOR (GAN Generator)
// ═══════════════════════════════════════════════════════════════════════════

/// Generates candidate chess moves from fused neural features.
pub struct MoveGenerator {
    /// Input projection
    input_proj: Dense,
    /// Hidden layers
    hidden1: Dense,
    hidden2: Dense,
    /// Output layer (4096 = 64 * 64 possible from-to combinations)
    output: Dense,
}

impl MoveGenerator {
    /// Create a new move generator.
    pub fn new(input_dim: usize) -> Self {
        MoveGenerator {
            input_proj: Dense::new(input_dim, 512, Activation::ReLU),
            hidden1: Dense::new(512, 1024, Activation::ReLU),
            hidden2: Dense::new(1024, 1024, Activation::ReLU),
            output: Dense::new(1024, 4096, Activation::None), // Raw logits for move selection
        }
    }

    /// Generate move probabilities.
    /// Output: 4096-dimensional vector (64 from-squares × 64 to-squares)
    pub fn forward(&mut self, features: &Tensor) -> Tensor {
        let x = self.input_proj.forward(features);
        let x = self.hidden1.forward(&x);
        let x = self.hidden2.forward(&x);
        let logits = self.output.forward(&x);
        
        // Softmax over all possible moves
        softmax(&logits)
    }

    /// Get move index from from-square and to-square.
    pub fn move_to_index(from: usize, to: usize) -> usize {
        from * 64 + to
    }

    /// Get from-square and to-square from move index.
    pub fn index_to_move(index: usize) -> (usize, usize) {
        (index / 64, index % 64)
    }

    /// Update weights.
    pub fn update(&mut self, learning_rate: f32) {
        self.input_proj.update(learning_rate);
        self.hidden1.update(learning_rate);
        self.hidden2.update(learning_rate);
        self.output.update(learning_rate);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// POSITION EVALUATOR (GAN Discriminator)
// ═══════════════════════════════════════════════════════════════════════════

/// Evaluates chess positions (like GAN discriminator).
pub struct PositionEvaluator {
    /// Input projection
    input_proj: Dense,
    /// Hidden layers
    hidden1: Dense,
    hidden2: Dense,
    /// Value head (single scalar output)
    value_head: Dense,
    /// Policy head (move preferences)
    policy_head: Dense,
}

impl PositionEvaluator {
    /// Create a new position evaluator.
    pub fn new(input_dim: usize) -> Self {
        PositionEvaluator {
            input_proj: Dense::new(input_dim, 512, Activation::ReLU),
            hidden1: Dense::new(512, 512, Activation::ReLU),
            hidden2: Dense::new(512, 256, Activation::ReLU),
            value_head: Dense::new(256, 1, Activation::Tanh), // Value in [-1, 1]
            policy_head: Dense::new(256, 4096, Activation::None),
        }
    }

    /// Evaluate position.
    /// Returns (value, policy) where:
    /// - value: position evaluation in [-1, 1] (positive = good for side to move)
    /// - policy: move probabilities
    pub fn forward(&mut self, features: &Tensor) -> (f32, Tensor) {
        let x = self.input_proj.forward(features);
        let x = self.hidden1.forward(&x);
        let x = self.hidden2.forward(&x);
        
        let value = self.value_head.forward(&x);
        let policy_logits = self.policy_head.forward(&x);
        let policy = softmax(&policy_logits);
        
        (value.data[0], policy)
    }

    /// Update weights.
    pub fn update(&mut self, learning_rate: f32) {
        self.input_proj.update(learning_rate);
        self.hidden1.update(learning_rate);
        self.hidden2.update(learning_rate);
        self.value_head.update(learning_rate);
        self.policy_head.update(learning_rate);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 3-WAY GAN BRAIN CORE
// ═══════════════════════════════════════════════════════════════════════════

/// The complete 3-way GAN Brain Core combining CNN + RNN + LSTM.
pub struct BrainCore {
    /// CNN for spatial pattern recognition
    pub cnn: ChessCNN,
    /// RNN for move sequence patterns
    pub rnn: RNN,
    /// LSTM for long-term strategic memory
    pub lstm: LSTM,
    /// Fusion layer
    pub fusion: FusionLayer,
    /// Move generator (GAN Generator)
    pub generator: MoveGenerator,
    /// Position evaluator (GAN Discriminator)
    pub evaluator: PositionEvaluator,
    /// Synchronization clock
    pub ganglion: NeuralClockCoordinator,
    /// Training mode
    pub training: bool,
    /// Learning rate
    pub learning_rate: f32,
    /// Move history for RNN/LSTM input
    move_history: Vec<Tensor>,
}

impl BrainCore {
    /// Create a new brain core.
    pub fn new() -> Self {
        // Dimensions
        let cnn_output_dim = 256;
        let rnn_hidden_dim = 128;
        let lstm_hidden_dim = 128;
        let fusion_output_dim = 512;
        
        BrainCore {
            cnn: ChessCNN::new(),
            rnn: RNN::new(64, rnn_hidden_dim, 2),  // 64 = move encoding dimension
            lstm: LSTM::new(64, lstm_hidden_dim, 2),
            fusion: FusionLayer::new(cnn_output_dim, rnn_hidden_dim, lstm_hidden_dim, fusion_output_dim),
            generator: MoveGenerator::new(fusion_output_dim),
            evaluator: PositionEvaluator::new(fusion_output_dim),
            ganglion: NeuralClockCoordinator::new(),
            training: true,
            learning_rate: 0.001,
            move_history: Vec::new(),
        }
    }

    /// Set training mode.
    pub fn train(&mut self, training: bool) {
        self.training = training;
        self.cnn.train(training);
    }

    /// Clear move history (start new game).
    pub fn reset_history(&mut self) {
        self.move_history.clear();
        self.rnn.clear_cache();
        self.lstm.clear_cache();
    }

    /// Record a move (for RNN/LSTM context).
    pub fn record_move(&mut self, move_encoding: Tensor) {
        self.move_history.push(move_encoding);
        
        // Keep last 50 moves
        if self.move_history.len() > 50 {
            self.move_history.remove(0);
        }
    }

    /// Encode a move as a tensor.
    pub fn encode_move(from: usize, to: usize) -> Tensor {
        let mut data = vec![0.0; 64];
        data[from] = 1.0;
        data[to] = -1.0;  // Negative to distinguish from-square
        Tensor::vector(data)
    }

    /// Forward pass through the brain.
    /// Returns (value, move_probabilities)
    pub fn forward(&mut self, board_planes: &[Tensor]) -> (f32, Tensor) {
        let timing = self.ganglion.begin_inference();
        
        // CNN: Process current board state
        let cnn_features = self.cnn.forward(board_planes);
        
        // RNN: Process move sequence
        let rnn_features = if self.move_history.is_empty() {
            Tensor::zeros(&[128])
        } else {
            self.rnn.get_final_hidden(&self.move_history)
        };
        
        // LSTM: Long-term context
        let lstm_features = if self.move_history.is_empty() {
            Tensor::zeros(&[128])
        } else {
            self.lstm.get_final_hidden(&self.move_history)
        };
        
        // Fuse all features
        let fused = self.fusion.forward(&cnn_features, &rnn_features, &lstm_features);
        
        // Evaluate position
        let (value, policy) = self.evaluator.forward(&fused);
        
        self.ganglion.end_inference(timing);
        
        (value, policy)
    }

    /// Get best move (greedy).
    pub fn get_best_move(&mut self, board_planes: &[Tensor]) -> (usize, usize, f32) {
        let (value, policy) = self.forward(board_planes);
        
        let best_idx = policy.argmax();
        let (from, to) = MoveGenerator::index_to_move(best_idx);
        
        (from, to, value)
    }

    /// Get move with exploration (for training).
    pub fn get_move_with_exploration(&mut self, board_planes: &[Tensor], temperature: f32) -> (usize, usize, f32) {
        let (value, policy) = self.forward(board_planes);
        
        // Apply temperature
        let scaled: Vec<f32> = policy.data.iter()
            .map(|&p| (p.ln() / temperature).exp())
            .collect();
        let sum: f32 = scaled.iter().sum();
        let probs: Vec<f32> = scaled.iter().map(|&p| p / sum).collect();
        
        // Sample from distribution
        let mut seed: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let r = ((seed >> 33) as f32) / (u32::MAX as f32);
        
        let mut cumsum = 0.0;
        let mut selected_idx = 0;
        for (i, &p) in probs.iter().enumerate() {
            cumsum += p;
            if r < cumsum {
                selected_idx = i;
                break;
            }
        }
        
        let (from, to) = MoveGenerator::index_to_move(selected_idx);
        (from, to, value)
    }

    /// Training step: update based on game outcome.
    pub fn train_step(&mut self, board_planes: &[Tensor], target_value: f32, target_policy: &Tensor) {
        let timing = self.ganglion.begin_forward();
        
        // Forward pass
        let (value, policy) = self.forward(board_planes);
        
        // Compute losses (simplified)
        let value_loss = (value - target_value).powi(2);
        let policy_loss = target_policy.mul(&policy.ln().scale(-1.0)).sum();
        let _total_loss = value_loss + policy_loss;
        
        self.ganglion.end_forward(timing);
        
        // Update all components
        let timing = self.ganglion.begin_backward();
        
        self.cnn.update(self.learning_rate);
        self.rnn.update(self.learning_rate);
        self.lstm.update(self.learning_rate);
        self.fusion.update(self.learning_rate);
        self.generator.update(self.learning_rate);
        self.evaluator.update(self.learning_rate);
        
        self.ganglion.end_backward(timing);
    }

    /// Get diagnostics.
    pub fn diagnostics(&self) -> String {
        format!(
            "Brain Core Status:\n\
             - CNN params: {}\n\
             - RNN params: {}\n\
             - LSTM params: {}\n\
             - Move history: {} moves\n\
             - Training: {}\n\
             - LR: {}\n\
             {}",
            self.cnn.num_params(),
            self.rnn.num_params(),
            self.lstm.num_params(),
            self.move_history.len(),
            self.training,
            self.learning_rate,
            self.ganglion.timing_report(),
        )
    }
}

impl Default for BrainCore {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ADVERSARIAL TRAINING
// ═══════════════════════════════════════════════════════════════════════════

/// Adversarial training loop for the GAN-like architecture.
pub struct AdversarialTrainer {
    /// Generator learning rate
    pub gen_lr: f32,
    /// Discriminator (evaluator) learning rate
    pub disc_lr: f32,
    /// Number of discriminator updates per generator update
    pub disc_steps: usize,
}

impl AdversarialTrainer {
    pub fn new() -> Self {
        AdversarialTrainer {
            gen_lr: 0.0002,
            disc_lr: 0.0001,
            disc_steps: 5,
        }
    }

    /// Single training iteration.
    pub fn train_iteration(
        &self,
        brain: &mut BrainCore,
        real_positions: &[Vec<Tensor>],
        real_values: &[f32],
    ) {
        // Train discriminator (evaluator) on real positions
        for (i, (position, &value)) in real_positions.iter().zip(real_values.iter()).enumerate() {
            if i >= self.disc_steps {
                break;
            }
            
            let (predicted_value, _) = brain.evaluator.forward(
                &brain.cnn.forward(position)
            );
            
            // Update evaluator to predict real values
            let _error = (predicted_value - value).powi(2);
            brain.evaluator.update(self.disc_lr);
        }
        
        // Train generator to fool evaluator
        // The generator tries to produce moves that the evaluator rates highly
        brain.generator.update(self.gen_lr);
    }
}

impl Default for AdversarialTrainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brain_core_creation() {
        let brain = BrainCore::new();
        assert!(brain.cnn.num_params() > 0);
        assert!(brain.rnn.num_params() > 0);
        assert!(brain.lstm.num_params() > 0);
    }

    #[test]
    fn test_brain_forward() {
        let mut brain = BrainCore::new();
        let board_planes: Vec<Tensor> = (0..12).map(|_| Tensor::rand(&[8, 8])).collect();
        
        let (value, policy) = brain.forward(&board_planes);
        
        assert!(value >= -1.0 && value <= 1.0);
        assert_eq!(policy.shape, vec![4096]);
        assert!((policy.sum() - 1.0).abs() < 0.01);  // Probabilities sum to 1
    }

    #[test]
    fn test_move_encoding() {
        let encoded = BrainCore::encode_move(4, 28);  // e2-e4
        assert_eq!(encoded.shape, vec![64]);
        assert_eq!(encoded.data[4], 1.0);
        assert_eq!(encoded.data[28], -1.0);
    }
}
