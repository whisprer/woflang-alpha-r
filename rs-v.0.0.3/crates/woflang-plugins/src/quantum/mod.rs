//! Quantum computing operations for Woflang.
//!
//! A simulated quantum computer with basic gates and measurements.
//! Uses probabilistic simulation (not actual quantum mechanics).
//!
//! ## Operations
//!
//! ### State Preparation
//! - `|ψ⟩`, `qubit` - Push a random qubit (superposition)
//! - `|0⟩` - Push qubit in |0⟩ state
//! - `|1⟩` - Push qubit in |1⟩ state
//!
//! ### Single-Qubit Gates
//! - `H` - Hadamard gate (creates superposition)
//! - `X` - Pauli-X gate (bit flip, NOT)
//! - `Y` - Pauli-Y gate
//! - `Z` - Pauli-Z gate (phase flip)
//! - `S` - S gate (π/2 phase)
//! - `T` - T gate (π/4 phase)
//!
//! ### Two-Qubit Gates
//! - `CNOT`, `CX` - Controlled NOT
//! - `SWAP` - Swap two qubits
//!
//! ### Measurement
//! - `measure` - Measure and collapse qubit

use std::sync::{Mutex, OnceLock};
use rand::Rng;
use woflang_core::WofValue;
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// QUBIT REPRESENTATION
// ═══════════════════════════════════════════════════════════════════════════

/// A qubit state represented as probability amplitudes.
/// |ψ⟩ = α|0⟩ + β|1⟩ where |α|² + |β|² = 1
#[derive(Clone, Debug)]
pub struct Qubit {
    /// Probability amplitude for |0⟩
    pub alpha_real: f64,
    pub alpha_imag: f64,
    /// Probability amplitude for |1⟩
    pub beta_real: f64,
    pub beta_imag: f64,
}

impl Qubit {
    /// Create |0⟩ state
    pub fn zero() -> Self {
        Qubit {
            alpha_real: 1.0,
            alpha_imag: 0.0,
            beta_real: 0.0,
            beta_imag: 0.0,
        }
    }

    /// Create |1⟩ state
    pub fn one() -> Self {
        Qubit {
            alpha_real: 0.0,
            alpha_imag: 0.0,
            beta_real: 1.0,
            beta_imag: 0.0,
        }
    }

    /// Create equal superposition: (|0⟩ + |1⟩) / √2
    pub fn superposition() -> Self {
        let s = 1.0 / 2.0_f64.sqrt();
        Qubit {
            alpha_real: s,
            alpha_imag: 0.0,
            beta_real: s,
            beta_imag: 0.0,
        }
    }

    /// Probability of measuring |0⟩
    pub fn prob_zero(&self) -> f64 {
        self.alpha_real * self.alpha_real + self.alpha_imag * self.alpha_imag
    }

    /// Probability of measuring |1⟩
    pub fn prob_one(&self) -> f64 {
        self.beta_real * self.beta_real + self.beta_imag * self.beta_imag
    }

    /// Measure the qubit, collapsing to classical bit
    pub fn measure(&mut self) -> i64 {
        let mut rng = rand::thread_rng();
        let r: f64 = rng.gen();

        if r < self.prob_zero() {
            // Collapse to |0⟩
            *self = Qubit::zero();
            0
        } else {
            // Collapse to |1⟩
            *self = Qubit::one();
            1
        }
    }

    /// Apply Hadamard gate: H|0⟩ = (|0⟩+|1⟩)/√2, H|1⟩ = (|0⟩-|1⟩)/√2
    pub fn hadamard(&mut self) {
        let s = 1.0 / 2.0_f64.sqrt();
        let new_alpha_r = s * (self.alpha_real + self.beta_real);
        let new_alpha_i = s * (self.alpha_imag + self.beta_imag);
        let new_beta_r = s * (self.alpha_real - self.beta_real);
        let new_beta_i = s * (self.alpha_imag - self.beta_imag);

        self.alpha_real = new_alpha_r;
        self.alpha_imag = new_alpha_i;
        self.beta_real = new_beta_r;
        self.beta_imag = new_beta_i;
    }

    /// Apply Pauli-X gate (bit flip): X|0⟩ = |1⟩, X|1⟩ = |0⟩
    pub fn pauli_x(&mut self) {
        std::mem::swap(&mut self.alpha_real, &mut self.beta_real);
        std::mem::swap(&mut self.alpha_imag, &mut self.beta_imag);
    }

    /// Apply Pauli-Y gate
    pub fn pauli_y(&mut self) {
        // Y = [[0, -i], [i, 0]]
        let new_alpha_r = self.beta_imag;
        let new_alpha_i = -self.beta_real;
        let new_beta_r = -self.alpha_imag;
        let new_beta_i = self.alpha_real;

        self.alpha_real = new_alpha_r;
        self.alpha_imag = new_alpha_i;
        self.beta_real = new_beta_r;
        self.beta_imag = new_beta_i;
    }

    /// Apply Pauli-Z gate (phase flip): Z|0⟩ = |0⟩, Z|1⟩ = -|1⟩
    pub fn pauli_z(&mut self) {
        self.beta_real = -self.beta_real;
        self.beta_imag = -self.beta_imag;
    }

    /// Apply S gate (π/2 phase): S|0⟩ = |0⟩, S|1⟩ = i|1⟩
    pub fn s_gate(&mut self) {
        let new_beta_r = -self.beta_imag;
        let new_beta_i = self.beta_real;
        self.beta_real = new_beta_r;
        self.beta_imag = new_beta_i;
    }

    /// Apply T gate (π/4 phase)
    pub fn t_gate(&mut self) {
        let s = 1.0 / 2.0_f64.sqrt();
        let new_beta_r = s * (self.beta_real - self.beta_imag);
        let new_beta_i = s * (self.beta_real + self.beta_imag);
        self.beta_real = new_beta_r;
        self.beta_imag = new_beta_i;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// QUANTUM REGISTER
// ═══════════════════════════════════════════════════════════════════════════

/// Global quantum register for multi-qubit operations.
fn quantum_register() -> &'static Mutex<Vec<Qubit>> {
    static REGISTER: OnceLock<Mutex<Vec<Qubit>>> = OnceLock::new();
    REGISTER.get_or_init(|| Mutex::new(Vec::new()))
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

fn random_bit() -> i64 {
    let mut rng = rand::thread_rng();
    if rng.gen::<bool>() { 1 } else { 0 }
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register quantum operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // STATE PREPARATION
    // ─────────────────────────────────────────────────────────────────────

    // Push a random qubit (superposition that immediately measures)
    // Stack: → 0|1
    interp.register("|ψ⟩", |interp| {
        let bit = random_bit();
        println!("[quantum] |ψ⟩ superposition → pushed qubit {}", bit);
        interp.stack_mut().push(WofValue::integer(bit));
        Ok(())
    });

    // Alternative name
    interp.register("qubit", |interp| {
        let bit = random_bit();
        println!("[quantum] qubit superposition → {}", bit);
        interp.stack_mut().push(WofValue::integer(bit));
        Ok(())
    });

    // Push |0⟩
    interp.register("|0⟩", |interp| {
        println!("[quantum] |0⟩ → pushed 0");
        interp.stack_mut().push(WofValue::integer(0));
        Ok(())
    });

    // Push |1⟩
    interp.register("|1⟩", |interp| {
        println!("[quantum] |1⟩ → pushed 1");
        interp.stack_mut().push(WofValue::integer(1));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // HADAMARD GATE
    // ─────────────────────────────────────────────────────────────────────

    // H gate: creates superposition, then measures
    // Stack: qubit → new_qubit
    interp.register("H", |interp| {
        if interp.stack().is_empty() {
            println!("[quantum] H gate: empty stack");
            return Ok(());
        }

        let _ = interp.stack_mut().pop()?;
        let bit = random_bit();
        println!("[quantum] H gate → new qubit {}", bit);
        interp.stack_mut().push(WofValue::integer(bit));
        Ok(())
    });

    // Hadamard alias
    interp.register("hadamard", |interp| {
        if interp.stack().is_empty() {
            return Ok(());
        }
        let _ = interp.stack_mut().pop()?;
        let bit = random_bit();
        interp.stack_mut().push(WofValue::integer(bit));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // PAULI GATES
    // ─────────────────────────────────────────────────────────────────────

    // X gate (bit flip): 0 ↔ 1
    // Stack: qubit → flipped_qubit
    interp.register("X", |interp| {
        if interp.stack().is_empty() {
            println!("[quantum] X gate: empty stack");
            return Ok(());
        }

        let q = interp.stack_mut().pop()?;
        let v = match &q {
            WofValue::Integer(n) => *n,
            _ => 0,
        };

        let flipped = if v == 0 { 1 } else { 0 };
        println!("[quantum] X gate: {} → {}", v, flipped);
        interp.stack_mut().push(WofValue::integer(flipped));
        Ok(())
    });

    // Pauli-X alias
    interp.register("pauli_x", |interp| {
        if interp.stack().is_empty() {
            return Ok(());
        }
        let q = interp.stack_mut().pop()?;
        let v = match &q {
            WofValue::Integer(n) => *n,
            _ => 0,
        };
        let flipped = if v == 0 { 1 } else { 0 };
        interp.stack_mut().push(WofValue::integer(flipped));
        Ok(())
    });

    // Y gate (simplified: bit flip with phase)
    interp.register("Y", |interp| {
        if interp.stack().is_empty() {
            println!("[quantum] Y gate: empty stack");
            return Ok(());
        }

        let q = interp.stack_mut().pop()?;
        let v = match &q {
            WofValue::Integer(n) => *n,
            _ => 0,
        };

        let flipped = if v == 0 { 1 } else { 0 };
        println!("[quantum] Y gate: {} → {} (with phase)", v, flipped);
        interp.stack_mut().push(WofValue::integer(flipped));
        Ok(())
    });

    // Z gate (phase flip): |0⟩ → |0⟩, |1⟩ → -|1⟩
    // In classical simulation, this is identity
    interp.register("Z", |interp| {
        if interp.stack().is_empty() {
            println!("[quantum] Z gate: empty stack");
            return Ok(());
        }

        let q = interp.stack_mut().pop()?;
        let v = match &q {
            WofValue::Integer(n) => *n,
            _ => 0,
        };

        println!("[quantum] Z gate: {} → {} (phase flip)", v, v);
        interp.stack_mut().push(WofValue::integer(v));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // TWO-QUBIT GATES
    // ─────────────────────────────────────────────────────────────────────

    // CNOT (Controlled NOT): flips target if control is 1
    // Stack: control target → control target'
    interp.register("CNOT", |interp| {
        if interp.stack().len() < 2 {
            println!("[quantum] CNOT: need 2 qubits");
            return Ok(());
        }

        let target = interp.stack_mut().pop()?;
        let control = interp.stack_mut().pop()?;

        let c = match &control {
            WofValue::Integer(n) => *n,
            _ => 0,
        };
        let t = match &target {
            WofValue::Integer(n) => *n,
            _ => 0,
        };

        let new_target = if c != 0 { if t == 0 { 1 } else { 0 } } else { t };

        println!("[quantum] CNOT: control={}, target={} → target'={}", c, t, new_target);
        interp.stack_mut().push(control);
        interp.stack_mut().push(WofValue::integer(new_target));
        Ok(())
    });

    // CX alias for CNOT
    interp.register("CX", |interp| {
        if interp.stack().len() < 2 {
            return Ok(());
        }
        let target = interp.stack_mut().pop()?;
        let control = interp.stack_mut().pop()?;
        let c = match &control { WofValue::Integer(n) => *n, _ => 0 };
        let t = match &target { WofValue::Integer(n) => *n, _ => 0 };
        let new_target = if c != 0 { if t == 0 { 1 } else { 0 } } else { t };
        interp.stack_mut().push(control);
        interp.stack_mut().push(WofValue::integer(new_target));
        Ok(())
    });

    // SWAP gate
    // Stack: a b → b a
    interp.register("SWAP", |interp| {
        if interp.stack().len() < 2 {
            println!("[quantum] SWAP: need 2 qubits");
            return Ok(());
        }

        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;

        interp.stack_mut().push(b);
        interp.stack_mut().push(a);
        println!("[quantum] SWAP: qubits swapped");
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // MEASUREMENT
    // ─────────────────────────────────────────────────────────────────────

    // Measure qubit (already classical in our simulation)
    // Stack: qubit → classical_bit
    interp.register("measure", |interp| {
        if interp.stack().is_empty() {
            println!("[quantum] measure: empty stack");
            return Ok(());
        }

        let q = interp.stack_mut().pop()?;
        let v = match &q {
            WofValue::Integer(n) => *n,
            WofValue::Float(f) => if *f >= 0.5 { 1 } else { 0 },
            _ => 0,
        };

        println!("[quantum] measured: {}", v);
        interp.stack_mut().push(WofValue::integer(v));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // QUANTUM REGISTER OPERATIONS
    // ─────────────────────────────────────────────────────────────────────

    // Initialize quantum register with n qubits
    interp.register("qreg_init", |interp| {
        let n = interp.stack_mut().pop()?.as_int()? as usize;

        if let Ok(mut reg) = quantum_register().lock() {
            reg.clear();
            for _ in 0..n {
                reg.push(Qubit::zero());
            }
            println!("[quantum] Initialized register with {} qubits", n);
        }
        Ok(())
    });

    // Show quantum register state
    interp.register("qreg_show", |_interp| {
        if let Ok(reg) = quantum_register().lock() {
            println!("[quantum] Register state ({} qubits):", reg.len());
            for (i, q) in reg.iter().enumerate() {
                println!(
                    "  q{}: P(0)={:.3}, P(1)={:.3}",
                    i,
                    q.prob_zero(),
                    q.prob_one()
                );
            }
        }
        Ok(())
    });

    // Measure all qubits in register
    interp.register("qreg_measure", |interp| {
        if let Ok(mut reg) = quantum_register().lock() {
            let mut results = Vec::new();
            for q in reg.iter_mut() {
                results.push(q.measure());
            }
            println!("[quantum] Measured register: {:?}", results);

            // Push results as integers
            for r in results {
                interp.stack_mut().push(WofValue::integer(r));
            }
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // HELP
    // ─────────────────────────────────────────────────────────────────────

    interp.register("quantum_help", |_interp| {
        println!("Quantum Computing Operations:");
        println!();
        println!("  State Preparation:");
        println!("    |ψ⟩, qubit    # Push random qubit (superposition)");
        println!("    |0⟩, |1⟩      # Push specific basis state");
        println!();
        println!("  Single-Qubit Gates:");
        println!("    H             # Hadamard (superposition)");
        println!("    X, pauli_x    # Pauli-X (bit flip, NOT)");
        println!("    Y, Z          # Pauli-Y, Pauli-Z");
        println!();
        println!("  Two-Qubit Gates:");
        println!("    CNOT, CX      # Controlled NOT");
        println!("    SWAP          # Swap two qubits");
        println!();
        println!("  Measurement:");
        println!("    measure       # Measure and collapse");
        println!();
        println!("  Register Operations:");
        println!("    n qreg_init   # Initialize n-qubit register");
        println!("    qreg_show     # Show register state");
        println!("    qreg_measure  # Measure all qubits");
        Ok(())
    });
}
