//! Tensor operations for neural network computations.
//!
//! A minimal but complete tensor implementation optimized for
//! neural network operations. No external dependencies.

use std::fmt;
use std::ops::{Add, Sub, Mul, Index, IndexMut};

// ═══════════════════════════════════════════════════════════════════════════
// TENSOR STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════

/// A multi-dimensional array for neural network computations.
#[derive(Clone)]
pub struct Tensor {
    /// Flat storage of tensor data
    pub data: Vec<f32>,
    /// Shape of the tensor (e.g., [batch, channels, height, width])
    pub shape: Vec<usize>,
    /// Strides for indexing
    strides: Vec<usize>,
}

impl Tensor {
    /// Create a new tensor with given shape, initialized to zeros.
    pub fn zeros(shape: &[usize]) -> Self {
        let size: usize = shape.iter().product();
        let strides = Self::compute_strides(shape);
        Tensor {
            data: vec![0.0; size],
            shape: shape.to_vec(),
            strides,
        }
    }

    /// Create a new tensor with given shape, initialized to ones.
    pub fn ones(shape: &[usize]) -> Self {
        let size: usize = shape.iter().product();
        let strides = Self::compute_strides(shape);
        Tensor {
            data: vec![1.0; size],
            shape: shape.to_vec(),
            strides,
        }
    }

    /// Create a tensor from raw data and shape.
    pub fn from_data(data: Vec<f32>, shape: &[usize]) -> Self {
        let expected_size: usize = shape.iter().product();
        assert_eq!(data.len(), expected_size, "Data size must match shape");
        let strides = Self::compute_strides(shape);
        Tensor {
            data,
            shape: shape.to_vec(),
            strides,
        }
    }

    /// Create a 1D tensor (vector).
    pub fn vector(data: Vec<f32>) -> Self {
        let len = data.len();
        Self::from_data(data, &[len])
    }

    /// Create a 2D tensor (matrix).
    pub fn matrix(rows: usize, cols: usize, data: Vec<f32>) -> Self {
        Self::from_data(data, &[rows, cols])
    }

    /// Create a tensor with random values in [0, 1).
    pub fn rand(shape: &[usize]) -> Self {
        let size: usize = shape.iter().product();
        let strides = Self::compute_strides(shape);
        
        // Simple LCG random number generator (deterministic for reproducibility)
        let mut seed: u64 = 0xDEAD_BEEF_CAFE_BABE;
        let data: Vec<f32> = (0..size)
            .map(|_| {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                ((seed >> 33) as f32) / (u32::MAX as f32)
            })
            .collect();
        
        Tensor {
            data,
            shape: shape.to_vec(),
            strides,
        }
    }

    /// Create a tensor with random values using Xavier/Glorot initialization.
    pub fn xavier(shape: &[usize]) -> Self {
        let fan_in = if shape.len() >= 2 { shape[shape.len() - 2] } else { 1 };
        let fan_out = if !shape.is_empty() { shape[shape.len() - 1] } else { 1 };
        let scale = (6.0 / (fan_in + fan_out) as f32).sqrt();
        
        let mut tensor = Self::rand(shape);
        for x in &mut tensor.data {
            *x = (*x * 2.0 - 1.0) * scale;
        }
        tensor
    }

    /// Compute strides for row-major order.
    fn compute_strides(shape: &[usize]) -> Vec<usize> {
        let mut strides = vec![1; shape.len()];
        for i in (0..shape.len().saturating_sub(1)).rev() {
            strides[i] = strides[i + 1] * shape[i + 1];
        }
        strides
    }

    /// Total number of elements.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Number of dimensions.
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    /// Get element at multi-dimensional index.
    pub fn get(&self, indices: &[usize]) -> f32 {
        let flat_idx = self.flat_index(indices);
        self.data[flat_idx]
    }

    /// Set element at multi-dimensional index.
    pub fn set(&mut self, indices: &[usize], value: f32) {
        let flat_idx = self.flat_index(indices);
        self.data[flat_idx] = value;
    }

    /// Convert multi-dimensional index to flat index.
    fn flat_index(&self, indices: &[usize]) -> usize {
        assert_eq!(indices.len(), self.shape.len());
        indices
            .iter()
            .zip(&self.strides)
            .map(|(i, s)| i * s)
            .sum()
    }

    /// Reshape tensor (must have same total size).
    pub fn reshape(&self, new_shape: &[usize]) -> Self {
        let new_size: usize = new_shape.iter().product();
        assert_eq!(self.size(), new_size, "Reshape size mismatch");
        Self::from_data(self.data.clone(), new_shape)
    }

    /// Flatten to 1D.
    pub fn flatten(&self) -> Self {
        Self::from_data(self.data.clone(), &[self.size()])
    }

    /// Transpose (for 2D tensors).
    pub fn transpose(&self) -> Self {
        assert_eq!(self.ndim(), 2, "Transpose requires 2D tensor");
        let (rows, cols) = (self.shape[0], self.shape[1]);
        let mut result = Self::zeros(&[cols, rows]);
        
        for i in 0..rows {
            for j in 0..cols {
                result.set(&[j, i], self.get(&[i, j]));
            }
        }
        result
    }

    // ─────────────────────────────────────────────────────────────────────
    // ELEMENT-WISE OPERATIONS
    // ─────────────────────────────────────────────────────────────────────

    /// Element-wise addition.
    pub fn add(&self, other: &Tensor) -> Self {
        assert_eq!(self.shape, other.shape, "Shape mismatch for add");
        let data: Vec<f32> = self.data.iter()
            .zip(&other.data)
            .map(|(a, b)| a + b)
            .collect();
        Self::from_data(data, &self.shape)
    }

    /// Element-wise subtraction.
    pub fn sub(&self, other: &Tensor) -> Self {
        assert_eq!(self.shape, other.shape, "Shape mismatch for sub");
        let data: Vec<f32> = self.data.iter()
            .zip(&other.data)
            .map(|(a, b)| a - b)
            .collect();
        Self::from_data(data, &self.shape)
    }

    /// Element-wise multiplication (Hadamard product).
    pub fn mul(&self, other: &Tensor) -> Self {
        assert_eq!(self.shape, other.shape, "Shape mismatch for mul");
        let data: Vec<f32> = self.data.iter()
            .zip(&other.data)
            .map(|(a, b)| a * b)
            .collect();
        Self::from_data(data, &self.shape)
    }

    /// Element-wise division.
    pub fn div(&self, other: &Tensor) -> Self {
        assert_eq!(self.shape, other.shape, "Shape mismatch for div");
        let data: Vec<f32> = self.data.iter()
            .zip(&other.data)
            .map(|(a, b)| a / b)
            .collect();
        Self::from_data(data, &self.shape)
    }

    /// Scalar multiplication.
    pub fn scale(&self, scalar: f32) -> Self {
        let data: Vec<f32> = self.data.iter().map(|x| x * scalar).collect();
        Self::from_data(data, &self.shape)
    }

    /// Scalar addition.
    pub fn add_scalar(&self, scalar: f32) -> Self {
        let data: Vec<f32> = self.data.iter().map(|x| x + scalar).collect();
        Self::from_data(data, &self.shape)
    }

    /// Element-wise square.
    pub fn square(&self) -> Self {
        let data: Vec<f32> = self.data.iter().map(|x| x * x).collect();
        Self::from_data(data, &self.shape)
    }

    /// Element-wise square root.
    pub fn sqrt(&self) -> Self {
        let data: Vec<f32> = self.data.iter().map(|x| x.sqrt()).collect();
        Self::from_data(data, &self.shape)
    }

    /// Element-wise exponential.
    pub fn exp(&self) -> Self {
        let data: Vec<f32> = self.data.iter().map(|x| x.exp()).collect();
        Self::from_data(data, &self.shape)
    }

    /// Element-wise natural log.
    pub fn ln(&self) -> Self {
        let data: Vec<f32> = self.data.iter().map(|x| x.ln()).collect();
        Self::from_data(data, &self.shape)
    }

    /// Element-wise absolute value.
    pub fn abs(&self) -> Self {
        let data: Vec<f32> = self.data.iter().map(|x| x.abs()).collect();
        Self::from_data(data, &self.shape)
    }

    /// Clamp values to range.
    pub fn clamp(&self, min: f32, max: f32) -> Self {
        let data: Vec<f32> = self.data.iter().map(|x| x.clamp(min, max)).collect();
        Self::from_data(data, &self.shape)
    }

    // ─────────────────────────────────────────────────────────────────────
    // REDUCTION OPERATIONS
    // ─────────────────────────────────────────────────────────────────────

    /// Sum of all elements.
    pub fn sum(&self) -> f32 {
        self.data.iter().sum()
    }

    /// Mean of all elements.
    pub fn mean(&self) -> f32 {
        self.sum() / self.size() as f32
    }

    /// Maximum element.
    pub fn max(&self) -> f32 {
        self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    }

    /// Minimum element.
    pub fn min(&self) -> f32 {
        self.data.iter().cloned().fold(f32::INFINITY, f32::min)
    }

    /// Index of maximum element.
    pub fn argmax(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Sum along an axis.
    pub fn sum_axis(&self, axis: usize) -> Self {
        assert!(axis < self.ndim(), "Axis out of bounds");
        
        let mut new_shape = self.shape.clone();
        new_shape.remove(axis);
        if new_shape.is_empty() {
            new_shape.push(1);
        }
        
        let mut result = Self::zeros(&new_shape);
        
        // Iterate through all elements and accumulate
        let axis_size = self.shape[axis];
        let axis_stride = self.strides[axis];
        
        for (i, &val) in self.data.iter().enumerate() {
            // Calculate index without the summed axis
            let mut new_idx = 0;
            let mut remaining = i;
            let mut new_stride_idx = 0;
            
            for (dim, (&size, &stride)) in self.shape.iter().zip(&self.strides).enumerate() {
                if dim != axis {
                    let idx = remaining / stride;
                    if new_stride_idx < result.strides.len() {
                        new_idx += idx * result.strides[new_stride_idx];
                        new_stride_idx += 1;
                    }
                }
                remaining %= stride;
            }
            
            result.data[new_idx] += val;
        }
        
        result
    }

    // ─────────────────────────────────────────────────────────────────────
    // MATRIX OPERATIONS
    // ─────────────────────────────────────────────────────────────────────

    /// Matrix multiplication (2D @ 2D).
    pub fn matmul(&self, other: &Tensor) -> Self {
        assert_eq!(self.ndim(), 2, "matmul requires 2D tensor");
        assert_eq!(other.ndim(), 2, "matmul requires 2D tensor");
        assert_eq!(self.shape[1], other.shape[0], "Inner dimensions must match");

        let (m, k) = (self.shape[0], self.shape[1]);
        let n = other.shape[1];
        
        let mut result = Self::zeros(&[m, n]);
        
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for l in 0..k {
                    sum += self.get(&[i, l]) * other.get(&[l, j]);
                }
                result.set(&[i, j], sum);
            }
        }
        
        result
    }

    /// Matrix-vector multiplication.
    pub fn matvec(&self, vec: &Tensor) -> Self {
        assert_eq!(self.ndim(), 2, "matvec requires 2D matrix");
        assert_eq!(vec.ndim(), 1, "matvec requires 1D vector");
        assert_eq!(self.shape[1], vec.shape[0], "Dimensions must match");

        let m = self.shape[0];
        let k = self.shape[1];
        
        let mut result = Self::zeros(&[m]);
        
        for i in 0..m {
            let mut sum = 0.0;
            for j in 0..k {
                sum += self.get(&[i, j]) * vec.data[j];
            }
            result.data[i] = sum;
        }
        
        result
    }

    /// Outer product of two vectors.
    pub fn outer(&self, other: &Tensor) -> Self {
        assert_eq!(self.ndim(), 1, "outer requires 1D tensor");
        assert_eq!(other.ndim(), 1, "outer requires 1D tensor");

        let m = self.shape[0];
        let n = other.shape[0];
        
        let mut result = Self::zeros(&[m, n]);
        
        for i in 0..m {
            for j in 0..n {
                result.set(&[i, j], self.data[i] * other.data[j]);
            }
        }
        
        result
    }

    /// Dot product of two vectors.
    pub fn dot(&self, other: &Tensor) -> f32 {
        assert_eq!(self.shape, other.shape, "Shapes must match for dot product");
        self.data.iter().zip(&other.data).map(|(a, b)| a * b).sum()
    }

    // ─────────────────────────────────────────────────────────────────────
    // CONVOLUTION OPERATIONS
    // ─────────────────────────────────────────────────────────────────────

    /// 2D convolution (for CNN).
    /// Input: [height, width], Kernel: [kh, kw]
    /// Returns: [out_h, out_w] where out_h = h - kh + 1
    pub fn conv2d(&self, kernel: &Tensor, stride: usize) -> Self {
        assert_eq!(self.ndim(), 2, "conv2d input must be 2D");
        assert_eq!(kernel.ndim(), 2, "conv2d kernel must be 2D");

        let (h, w) = (self.shape[0], self.shape[1]);
        let (kh, kw) = (kernel.shape[0], kernel.shape[1]);
        
        let out_h = (h - kh) / stride + 1;
        let out_w = (w - kw) / stride + 1;
        
        let mut result = Self::zeros(&[out_h, out_w]);
        
        for i in 0..out_h {
            for j in 0..out_w {
                let mut sum = 0.0;
                for ki in 0..kh {
                    for kj in 0..kw {
                        let ii = i * stride + ki;
                        let jj = j * stride + kj;
                        sum += self.get(&[ii, jj]) * kernel.get(&[ki, kj]);
                    }
                }
                result.set(&[i, j], sum);
            }
        }
        
        result
    }

    /// Max pooling 2D.
    pub fn max_pool2d(&self, pool_size: usize) -> Self {
        assert_eq!(self.ndim(), 2, "max_pool2d requires 2D tensor");

        let (h, w) = (self.shape[0], self.shape[1]);
        let out_h = h / pool_size;
        let out_w = w / pool_size;
        
        let mut result = Self::zeros(&[out_h, out_w]);
        
        for i in 0..out_h {
            for j in 0..out_w {
                let mut max_val = f32::NEG_INFINITY;
                for pi in 0..pool_size {
                    for pj in 0..pool_size {
                        let ii = i * pool_size + pi;
                        let jj = j * pool_size + pj;
                        max_val = max_val.max(self.get(&[ii, jj]));
                    }
                }
                result.set(&[i, j], max_val);
            }
        }
        
        result
    }

    /// Average pooling 2D.
    pub fn avg_pool2d(&self, pool_size: usize) -> Self {
        assert_eq!(self.ndim(), 2, "avg_pool2d requires 2D tensor");

        let (h, w) = (self.shape[0], self.shape[1]);
        let out_h = h / pool_size;
        let out_w = w / pool_size;
        let pool_area = (pool_size * pool_size) as f32;
        
        let mut result = Self::zeros(&[out_h, out_w]);
        
        for i in 0..out_h {
            for j in 0..out_w {
                let mut sum = 0.0;
                for pi in 0..pool_size {
                    for pj in 0..pool_size {
                        let ii = i * pool_size + pi;
                        let jj = j * pool_size + pj;
                        sum += self.get(&[ii, jj]);
                    }
                }
                result.set(&[i, j], sum / pool_area);
            }
        }
        
        result
    }

    // ─────────────────────────────────────────────────────────────────────
    // BROADCASTING
    // ─────────────────────────────────────────────────────────────────────

    /// Add a vector to each row of a matrix (broadcasting).
    pub fn add_broadcast_row(&self, vec: &Tensor) -> Self {
        assert_eq!(self.ndim(), 2, "Requires 2D tensor");
        assert_eq!(vec.ndim(), 1, "Requires 1D vector");
        assert_eq!(self.shape[1], vec.shape[0], "Vector size must match columns");

        let mut result = self.clone();
        let (rows, cols) = (self.shape[0], self.shape[1]);
        
        for i in 0..rows {
            for j in 0..cols {
                let idx = i * cols + j;
                result.data[idx] += vec.data[j];
            }
        }
        
        result
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════

impl fmt::Debug for Tensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tensor(shape={:?}, data=[", self.shape)?;
        
        let max_display = 10;
        for (i, &val) in self.data.iter().take(max_display).enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:.4}", val)?;
        }
        
        if self.data.len() > max_display {
            write!(f, ", ...")?;
        }
        
        write!(f, "])")
    }
}

impl Index<usize> for Tensor {
    type Output = f32;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Tensor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Add for &Tensor {
    type Output = Tensor;
    
    fn add(self, other: Self) -> Self::Output {
        self.add(other)
    }
}

impl Sub for &Tensor {
    type Output = Tensor;
    
    fn sub(self, other: Self) -> Self::Output {
        self.sub(other)
    }
}

impl Mul for &Tensor {
    type Output = Tensor;
    
    fn mul(self, other: Self) -> Self::Output {
        self.mul(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matmul() {
        let a = Tensor::matrix(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = Tensor::matrix(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
        let c = a.matmul(&b);
        
        assert_eq!(c.shape, vec![2, 2]);
        assert_eq!(c.get(&[0, 0]), 58.0);  // 1*7 + 2*9 + 3*11
        assert_eq!(c.get(&[0, 1]), 64.0);  // 1*8 + 2*10 + 3*12
    }

    #[test]
    fn test_conv2d() {
        let input = Tensor::matrix(4, 4, (1..=16).map(|x| x as f32).collect());
        let kernel = Tensor::ones(&[2, 2]);
        let output = input.conv2d(&kernel, 1);
        
        assert_eq!(output.shape, vec![3, 3]);
    }
}
