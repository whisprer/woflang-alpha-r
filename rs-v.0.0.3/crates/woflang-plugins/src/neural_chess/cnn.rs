//! Convolutional Neural Network (CNN) for chess board pattern recognition.
//!
//! The CNN processes the 8x8 chess board as a spatial input, recognizing:
//! - Piece patterns (pawn chains, knight outposts)
//! - Positional features (king safety, open files)
//! - Tactical motifs (forks, pins, skewers)

use super::tensor::Tensor;
use super::activation::{relu, Activation};
use super::layers::{Dense, Layer};

// ═══════════════════════════════════════════════════════════════════════════
// CNN BUILDING BLOCKS
// ═══════════════════════════════════════════════════════════════════════════

/// 2D Convolution layer optimized for chess board analysis.
pub struct ChessConv2D {
    /// Kernels: [num_filters, in_channels, kernel_h, kernel_w]
    pub kernels: Vec<Vec<Tensor>>,
    /// Bias for each filter
    pub biases: Vec<f32>,
    /// Kernel size
    pub kernel_size: usize,
    /// Number of input channels
    pub in_channels: usize,
    /// Number of output channels (filters)
    pub out_channels: usize,
    /// Padding size
    pub padding: usize,
    /// Stride
    pub stride: usize,
    
    // Cached for backward
    input_cache: Option<Vec<Tensor>>,
    
    // Gradients
    kernel_grads: Vec<Vec<Tensor>>,
    bias_grads: Vec<f32>,
}

impl ChessConv2D {
    /// Create a new convolutional layer.
    pub fn new(
        in_channels: usize,
        out_channels: usize,
        kernel_size: usize,
        padding: usize,
        stride: usize,
    ) -> Self {
        // Initialize kernels with Xavier initialization
        let kernels: Vec<Vec<Tensor>> = (0..out_channels)
            .map(|_| {
                (0..in_channels)
                    .map(|_| Tensor::xavier(&[kernel_size, kernel_size]))
                    .collect()
            })
            .collect();
        
        let kernel_grads: Vec<Vec<Tensor>> = (0..out_channels)
            .map(|_| {
                (0..in_channels)
                    .map(|_| Tensor::zeros(&[kernel_size, kernel_size]))
                    .collect()
            })
            .collect();
        
        ChessConv2D {
            kernels,
            biases: vec![0.0; out_channels],
            kernel_size,
            in_channels,
            out_channels,
            padding,
            stride,
            input_cache: None,
            kernel_grads,
            bias_grads: vec![0.0; out_channels],
        }
    }

    /// Apply padding to input.
    fn pad_input(&self, input: &Tensor) -> Tensor {
        if self.padding == 0 {
            return input.clone();
        }
        
        let (h, w) = (input.shape[0], input.shape[1]);
        let new_h = h + 2 * self.padding;
        let new_w = w + 2 * self.padding;
        
        let mut padded = Tensor::zeros(&[new_h, new_w]);
        
        for i in 0..h {
            for j in 0..w {
                padded.set(&[i + self.padding, j + self.padding], input.get(&[i, j]));
            }
        }
        
        padded
    }

    /// Forward pass for multi-channel input.
    /// Input: Vec<Tensor> where each tensor is [height, width]
    /// Output: Vec<Tensor> of [out_height, out_width]
    pub fn forward(&mut self, input: &[Tensor]) -> Vec<Tensor> {
        assert_eq!(input.len(), self.in_channels, "Input channels mismatch");
        
        // Cache input for backward
        self.input_cache = Some(input.to_vec());
        
        // Pad inputs
        let padded: Vec<Tensor> = input.iter().map(|t| self.pad_input(t)).collect();
        
        let h = padded[0].shape[0];
        let w = padded[0].shape[1];
        let out_h = (h - self.kernel_size) / self.stride + 1;
        let out_w = (w - self.kernel_size) / self.stride + 1;
        
        // Compute output for each filter
        let mut outputs = Vec::with_capacity(self.out_channels);
        
        for f in 0..self.out_channels {
            let mut output = Tensor::zeros(&[out_h, out_w]);
            
            // Sum convolutions over all input channels
            for c in 0..self.in_channels {
                let conv = padded[c].conv2d(&self.kernels[f][c], self.stride);
                output = output.add(&conv);
            }
            
            // Add bias and apply ReLU
            let biased = output.add_scalar(self.biases[f]);
            outputs.push(relu(&biased));
        }
        
        outputs
    }

    /// Update weights.
    pub fn update(&mut self, learning_rate: f32) {
        for f in 0..self.out_channels {
            for c in 0..self.in_channels {
                let update = self.kernel_grads[f][c].scale(learning_rate);
                self.kernels[f][c] = self.kernels[f][c].sub(&update);
                
                // Reset gradient
                self.kernel_grads[f][c] = Tensor::zeros(&[self.kernel_size, self.kernel_size]);
            }
            self.biases[f] -= learning_rate * self.bias_grads[f];
            self.bias_grads[f] = 0.0;
        }
    }

    /// Number of parameters.
    pub fn num_params(&self) -> usize {
        self.out_channels * self.in_channels * self.kernel_size * self.kernel_size + self.out_channels
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BATCH NORMALIZATION FOR CNN
// ═══════════════════════════════════════════════════════════════════════════

/// Batch normalization for convolutional layers.
pub struct ConvBatchNorm {
    /// Scale parameter per channel
    pub gamma: Vec<f32>,
    /// Shift parameter per channel
    pub beta: Vec<f32>,
    /// Running mean per channel
    pub running_mean: Vec<f32>,
    /// Running variance per channel
    pub running_var: Vec<f32>,
    /// Number of channels
    pub num_channels: usize,
    /// Momentum
    pub momentum: f32,
    /// Epsilon
    pub epsilon: f32,
}

impl ConvBatchNorm {
    pub fn new(num_channels: usize) -> Self {
        ConvBatchNorm {
            gamma: vec![1.0; num_channels],
            beta: vec![0.0; num_channels],
            running_mean: vec![0.0; num_channels],
            running_var: vec![1.0; num_channels],
            num_channels,
            momentum: 0.1,
            epsilon: 1e-5,
        }
    }

    pub fn forward(&mut self, inputs: &[Tensor], training: bool) -> Vec<Tensor> {
        let mut outputs = Vec::with_capacity(inputs.len());
        
        for (c, input) in inputs.iter().enumerate() {
            let mean = if training { input.mean() } else { self.running_mean[c] };
            let var = if training {
                let diff = input.add_scalar(-mean);
                diff.square().mean()
            } else {
                self.running_var[c]
            };
            
            // Update running stats
            if training {
                self.running_mean[c] = (1.0 - self.momentum) * self.running_mean[c] + self.momentum * mean;
                self.running_var[c] = (1.0 - self.momentum) * self.running_var[c] + self.momentum * var;
            }
            
            // Normalize
            let std = (var + self.epsilon).sqrt();
            let normalized = input.add_scalar(-mean).scale(1.0 / std);
            
            // Scale and shift
            let output = normalized.scale(self.gamma[c]).add_scalar(self.beta[c]);
            outputs.push(output);
        }
        
        outputs
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// POOLING LAYERS
// ═══════════════════════════════════════════════════════════════════════════

/// Max pooling for multiple channels.
pub fn max_pool2d_multi(inputs: &[Tensor], pool_size: usize) -> Vec<Tensor> {
    inputs.iter().map(|t| t.max_pool2d(pool_size)).collect()
}

/// Global average pooling - reduces each channel to a single value.
pub fn global_avg_pool(inputs: &[Tensor]) -> Tensor {
    let data: Vec<f32> = inputs.iter().map(|t| t.mean()).collect();
    Tensor::vector(data)
}

// ═══════════════════════════════════════════════════════════════════════════
// CHESS CNN ARCHITECTURE
// ═══════════════════════════════════════════════════════════════════════════

/// Complete CNN for chess board evaluation.
/// 
/// Architecture:
/// Input: 12 channels (6 piece types × 2 colors) × 8 × 8
/// Conv1: 64 filters, 3×3, padding=1 → 64 × 8 × 8
/// Conv2: 128 filters, 3×3, padding=1 → 128 × 8 × 8
/// Conv3: 256 filters, 3×3, padding=1 → 256 × 8 × 8
/// Global Average Pool → 256
/// Dense: 256 → 512
/// Dense: 512 → 256
/// Output: 256 (feature vector for combining with RNN/LSTM)
pub struct ChessCNN {
    /// Convolutional layers
    pub conv1: ChessConv2D,
    pub conv2: ChessConv2D,
    pub conv3: ChessConv2D,
    
    /// Batch normalization
    pub bn1: ConvBatchNorm,
    pub bn2: ConvBatchNorm,
    pub bn3: ConvBatchNorm,
    
    /// Dense layers
    pub fc1: Dense,
    pub fc2: Dense,
    
    /// Training mode
    pub training: bool,
}

impl ChessCNN {
    /// Create a new chess CNN.
    pub fn new() -> Self {
        ChessCNN {
            // 12 input channels (pieces), 64 output
            conv1: ChessConv2D::new(12, 64, 3, 1, 1),
            // 64 input, 128 output
            conv2: ChessConv2D::new(64, 128, 3, 1, 1),
            // 128 input, 256 output
            conv3: ChessConv2D::new(128, 256, 3, 1, 1),
            
            bn1: ConvBatchNorm::new(64),
            bn2: ConvBatchNorm::new(128),
            bn3: ConvBatchNorm::new(256),
            
            // Dense layers after global pooling
            fc1: Dense::new(256, 512, Activation::ReLU),
            fc2: Dense::new(512, 256, Activation::ReLU),
            
            training: true,
        }
    }

    /// Set training mode.
    pub fn train(&mut self, training: bool) {
        self.training = training;
    }

    /// Forward pass.
    /// Input: 12 channels × 8 × 8 (piece planes)
    /// Output: 256-dimensional feature vector
    pub fn forward(&mut self, input: &[Tensor]) -> Tensor {
        // Conv block 1
        let x = self.conv1.forward(input);
        let x = self.bn1.forward(&x, self.training);
        
        // Conv block 2
        let x = self.conv2.forward(&x);
        let x = self.bn2.forward(&x, self.training);
        
        // Conv block 3
        let x = self.conv3.forward(&x);
        let x = self.bn3.forward(&x, self.training);
        
        // Global average pooling
        let x = global_avg_pool(&x);
        
        // Dense layers
        let x = self.fc1.forward(&x);
        self.fc2.forward(&x)
    }

    /// Update all weights.
    pub fn update(&mut self, learning_rate: f32) {
        self.conv1.update(learning_rate);
        self.conv2.update(learning_rate);
        self.conv3.update(learning_rate);
        self.fc1.update(learning_rate);
        self.fc2.update(learning_rate);
    }

    /// Total number of parameters.
    pub fn num_params(&self) -> usize {
        self.conv1.num_params() +
        self.conv2.num_params() +
        self.conv3.num_params() +
        self.fc1.num_params() +
        self.fc2.num_params()
    }
}

impl Default for ChessCNN {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// RESIDUAL BLOCKS (FOR DEEPER NETWORKS)
// ═══════════════════════════════════════════════════════════════════════════

/// Residual block for deeper networks.
/// output = relu(input + conv2(relu(conv1(input))))
pub struct ResidualBlock {
    pub conv1: ChessConv2D,
    pub conv2: ChessConv2D,
    pub bn1: ConvBatchNorm,
    pub bn2: ConvBatchNorm,
}

impl ResidualBlock {
    pub fn new(channels: usize) -> Self {
        ResidualBlock {
            conv1: ChessConv2D::new(channels, channels, 3, 1, 1),
            conv2: ChessConv2D::new(channels, channels, 3, 1, 1),
            bn1: ConvBatchNorm::new(channels),
            bn2: ConvBatchNorm::new(channels),
        }
    }

    pub fn forward(&mut self, input: &[Tensor], training: bool) -> Vec<Tensor> {
        // First conv + bn + relu
        let x = self.conv1.forward(input);
        let x = self.bn1.forward(&x, training);
        let x: Vec<Tensor> = x.iter().map(|t| relu(t)).collect();
        
        // Second conv + bn
        let x = self.conv2.forward(&x);
        let x = self.bn2.forward(&x, training);
        
        // Skip connection
        let output: Vec<Tensor> = x.iter()
            .zip(input.iter())
            .map(|(conv_out, skip)| relu(&conv_out.add(skip)))
            .collect();
        
        output
    }

    pub fn update(&mut self, learning_rate: f32) {
        self.conv1.update(learning_rate);
        self.conv2.update(learning_rate);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BOARD ENCODING
// ═══════════════════════════════════════════════════════════════════════════

/// Convert a chess board to CNN input planes.
/// Returns 12 planes: 6 piece types × 2 colors
/// Each plane is 8×8 with 1.0 where piece exists, 0.0 elsewhere
pub fn board_to_planes(board: &[i8; 64]) -> Vec<Tensor> {
    let mut planes = vec![Tensor::zeros(&[8, 8]); 12];
    
    // Piece encoding: 
    // 1-6: White pieces (Pawn, Knight, Bishop, Rook, Queen, King)
    // -1 to -6: Black pieces
    
    for (sq, &piece) in board.iter().enumerate() {
        if piece == 0 {
            continue;
        }
        
        let row = sq / 8;
        let col = sq % 8;
        
        let plane_idx = if piece > 0 {
            (piece - 1) as usize  // White: 0-5
        } else {
            ((-piece - 1) + 6) as usize  // Black: 6-11
        };
        
        planes[plane_idx].set(&[row, col], 1.0);
    }
    
    planes
}

/// Additional feature planes for enhanced position understanding.
pub fn additional_planes(
    board: &[i8; 64],
    castling_rights: u8,
    en_passant: Option<usize>,
    halfmove: u32,
    side_to_move: bool,  // true = white
) -> Vec<Tensor> {
    let mut planes = Vec::new();
    
    // Side to move plane (all 1s if white to move)
    let stm_plane = if side_to_move {
        Tensor::ones(&[8, 8])
    } else {
        Tensor::zeros(&[8, 8])
    };
    planes.push(stm_plane);
    
    // Castling rights (4 planes)
    for i in 0..4 {
        let can_castle = (castling_rights >> i) & 1 == 1;
        planes.push(if can_castle {
            Tensor::ones(&[8, 8])
        } else {
            Tensor::zeros(&[8, 8])
        });
    }
    
    // En passant square
    let mut ep_plane = Tensor::zeros(&[8, 8]);
    if let Some(ep_sq) = en_passant {
        let row = ep_sq / 8;
        let col = ep_sq % 8;
        ep_plane.set(&[row, col], 1.0);
    }
    planes.push(ep_plane);
    
    // Halfmove clock (normalized)
    let hm_normalized = (halfmove as f32 / 100.0).min(1.0);
    planes.push(Tensor::from_data(vec![hm_normalized; 64], &[8, 8]));
    
    planes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv2d() {
        let mut conv = ChessConv2D::new(1, 4, 3, 1, 1);
        let input = vec![Tensor::rand(&[8, 8])];
        let output = conv.forward(&input);
        
        assert_eq!(output.len(), 4);
        assert_eq!(output[0].shape, vec![8, 8]);
    }

    #[test]
    fn test_chess_cnn() {
        let mut cnn = ChessCNN::new();
        let input: Vec<Tensor> = (0..12).map(|_| Tensor::rand(&[8, 8])).collect();
        let output = cnn.forward(&input);
        
        assert_eq!(output.shape, vec![256]);
    }

    #[test]
    fn test_board_to_planes() {
        let mut board = [0i8; 64];
        board[0] = 4;  // White rook on a1
        board[63] = -4;  // Black rook on h8
        
        let planes = board_to_planes(&board);
        assert_eq!(planes.len(), 12);
        assert_eq!(planes[3].get(&[0, 0]), 1.0);  // White rook plane
        assert_eq!(planes[9].get(&[7, 7]), 1.0);  // Black rook plane
    }
}
