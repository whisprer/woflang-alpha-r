//! Quantum computing simulation operations.
//!
//! Provides basic quantum state manipulation and gates:
//!
//! | Operation  | Stack Effect | Description |
//! |------------|--------------|-------------|
//! | `\|0⟩`     | ( -- q)      | Create \|0⟩ qubit state |
//! | `\|1⟩`     | ( -- q)      | Create \|1⟩ qubit state |
//! | `H`        | (q -- q')    | Hadamard gate |
//! | `X`        | (q -- q')    | Pauli-X (NOT) gate |
//! | `Z`        | (q -- q')    | Pauli-Z gate |
//! | `measure`  | (q -- n)     | Collapse and measure |
//! | `bell`     | ( -- q q)    | Create Bell state |
//!
//! ## Representation
//!
//! Quantum states are represented as complex amplitudes stored in
//! a two-element array [α, β] where the state is α|0⟩ + β|1⟩.
//! For simplicity, we use real amplitudes only in this simulation.

use woflang_core::{InterpreterContext, Result, WofError, WofValue};
use woflang_runtime::Interpreter;

/// Register all quantum operations.
pub fn register(interp: &mut Interpreter) {
    // State preparation
    interp.register("|0⟩", op_ket_zero);
    interp.register("|1⟩", op_ket_one);
    interp.register("|+⟩", op_ket_plus);
    interp.register("|-⟩", op_ket_minus);

    // Gates
    interp.register("H", op_hadamard);
    interp.register("X", op_pauli_x);
    interp.register("Y", op_pauli_y);
    interp.register("Z", op_pauli_z);

    // Measurement
    interp.register("measure", op_measure);

    // Entanglement
    interp.register("bell", op_bell_state);

    // Display
    interp.register("qshow", op_qshow);
}

/// Internal representation of a qubit state.
/// Stores [amplitude_0, amplitude_1] as real numbers.
#[derive(Clone, Copy, Debug)]
struct QubitState {
    alpha: f64, // Amplitude of |0⟩
    beta: f64,  // Amplitude of |1⟩
}

impl QubitState {
    fn ket_zero() -> Self {
        Self {
            alpha: 1.0,
            beta: 0.0,
        }
    }

    fn ket_one() -> Self {
        Self {
            alpha: 0.0,
            beta: 1.0,
        }
    }

    fn ket_plus() -> Self {
        let s = std::f64::consts::FRAC_1_SQRT_2;
        Self { alpha: s, beta: s }
    }

    fn ket_minus() -> Self {
        let s = std::f64::consts::FRAC_1_SQRT_2;
        Self { alpha: s, beta: -s }
    }

    fn hadamard(self) -> Self {
        let s = std::f64::consts::FRAC_1_SQRT_2;
        Self {
            alpha: s * (self.alpha + self.beta),
            beta: s * (self.alpha - self.beta),
        }
    }

    fn pauli_x(self) -> Self {
        Self {
            alpha: self.beta,
            beta: self.alpha,
        }
    }

    fn pauli_y(self) -> Self {
        // Y = i * [[0, -i], [i, 0]] simplified for real amplitudes
        Self {
            alpha: -self.beta,
            beta: self.alpha,
        }
    }

    fn pauli_z(self) -> Self {
        Self {
            alpha: self.alpha,
            beta: -self.beta,
        }
    }

    fn measure(self) -> bool {
        let prob_one = self.beta * self.beta;
        rand::random::<f64>() < prob_one
    }

    fn to_value(self) -> WofValue {
        // Encode as a string representation for stack storage
        WofValue::string(format!(
            "({:.4})|0⟩ + ({:.4})|1⟩",
            self.alpha, self.beta
        ))
    }

    fn from_value(val: &WofValue) -> Result<Self> {
        let s = val.as_str()?;

        // Parse simple format: "(a)|0⟩ + (b)|1⟩"
        if let Some(rest) = s.strip_prefix('(') {
            let parts: Vec<&str> = rest.split(")|0⟩ + (").collect();
            if parts.len() == 2 {
                if let Some(beta_str) = parts[1].strip_suffix(")|1⟩") {
                    let alpha: f64 = parts[0]
                        .parse()
                        .map_err(|_| WofError::parse_simple("invalid qubit alpha"))?;
                    let beta: f64 = beta_str
                        .parse()
                        .map_err(|_| WofError::parse_simple("invalid qubit beta"))?;
                    return Ok(Self { alpha, beta });
                }
            }
        }

        // Handle simple ket notation
        match s {
            "|0⟩" => Ok(Self::ket_zero()),
            "|1⟩" => Ok(Self::ket_one()),
            "|+⟩" => Ok(Self::ket_plus()),
            "|-⟩" => Ok(Self::ket_minus()),
            _ => Err(WofError::parse_simple(format!("invalid qubit state: {s}"))),
        }
    }
}

fn op_ket_zero(interp: &mut Interpreter) -> Result<()> {
    interp.push(QubitState::ket_zero().to_value());
    Ok(())
}

fn op_ket_one(interp: &mut Interpreter) -> Result<()> {
    interp.push(QubitState::ket_one().to_value());
    Ok(())
}

fn op_ket_plus(interp: &mut Interpreter) -> Result<()> {
    interp.push(QubitState::ket_plus().to_value());
    Ok(())
}

fn op_ket_minus(interp: &mut Interpreter) -> Result<()> {
    interp.push(QubitState::ket_minus().to_value());
    Ok(())
}

fn op_hadamard(interp: &mut Interpreter) -> Result<()> {
    let val = interp.stack_mut().pop()?;
    let state = QubitState::from_value(&val)?;
    interp.push(state.hadamard().to_value());
    Ok(())
}

fn op_pauli_x(interp: &mut Interpreter) -> Result<()> {
    let val = interp.stack_mut().pop()?;
    let state = QubitState::from_value(&val)?;
    interp.push(state.pauli_x().to_value());
    Ok(())
}

fn op_pauli_y(interp: &mut Interpreter) -> Result<()> {
    let val = interp.stack_mut().pop()?;
    let state = QubitState::from_value(&val)?;
    interp.push(state.pauli_y().to_value());
    Ok(())
}

fn op_pauli_z(interp: &mut Interpreter) -> Result<()> {
    let val = interp.stack_mut().pop()?;
    let state = QubitState::from_value(&val)?;
    interp.push(state.pauli_z().to_value());
    Ok(())
}

fn op_measure(interp: &mut Interpreter) -> Result<()> {
    let val = interp.stack_mut().pop()?;
    let state = QubitState::from_value(&val)?;
    let result = if state.measure() { 1 } else { 0 };
    interp.push(WofValue::integer(result));
    Ok(())
}

fn op_bell_state(interp: &mut Interpreter) -> Result<()> {
    // Create |Φ+⟩ = (|00⟩ + |11⟩)/√2
    // Represented as two entangled qubits in superposition
    let s = std::f64::consts::FRAC_1_SQRT_2;
    interp.push(WofValue::string(format!(
        "Bell: ({s:.4})|00⟩ + ({s:.4})|11⟩"
    )));
    Ok(())
}

fn op_qshow(interp: &mut Interpreter) -> Result<()> {
    let val = interp.stack().peek()?;
    println!("Quantum state: {val}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_interp() -> Interpreter {
        let mut interp = Interpreter::new();
        register(&mut interp);
        interp
    }

    #[test]
    fn test_ket_zero() {
        let mut interp = make_interp();
        interp.exec_line("|0⟩").unwrap();
        let val = interp.stack().peek().unwrap();
        assert!(val.as_str().unwrap().contains("|0⟩"));
    }

    #[test]
    fn test_hadamard_on_zero() {
        let mut interp = make_interp();
        interp.exec_line("|0⟩ H").unwrap();
        let val = interp.stack().peek().unwrap();
        let s = val.as_str().unwrap();
        // After H on |0⟩, both amplitudes should be ~0.7071
        assert!(s.contains("0.7071"));
    }

    #[test]
    fn test_pauli_x_flip() {
        let mut interp = make_interp();
        interp.exec_line("|0⟩ X").unwrap();
        let val = interp.stack().peek().unwrap();
        let state = QubitState::from_value(val).unwrap();
        // X|0⟩ = |1⟩
        assert!(state.alpha.abs() < 0.001);
        assert!((state.beta - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_bell_state() {
        let mut interp = make_interp();
        interp.exec_line("bell").unwrap();
        let val = interp.stack().peek().unwrap();
        assert!(val.as_str().unwrap().contains("Bell"));
    }
}
