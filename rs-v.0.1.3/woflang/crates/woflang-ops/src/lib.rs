//! # Woflang Standard Library Operations
//!
//! This crate provides the standard library of operations for the Woflang
//! interpreter. Operations are organized into modules by category:
//!
//! - [`arithmetic`]: Basic math operations (+, -, *, /, pow, sqrt, etc.)
//! - [`stack`]: Stack manipulation (dup, drop, swap, rot, etc.)
//! - [`math`]: Extended math (trig, constants, etc.)
//! - [`logic`]: Boolean and propositional logic
//! - [`quantum`]: Quantum computing simulation
//! - [`crypto`]: Cryptographic primitives
//! - [`io`]: Input/output operations
//!
//! ## Usage
//!
//! Use [`register_all`] to register all standard operations with an
//! interpreter, or register individual modules for a minimal footprint.
//!
//! ```
//! use woflang_runtime::Interpreter;
//! use woflang_ops::register_all;
//!
//! let mut interp = Interpreter::new();
//! register_all(&mut interp);
//!
//! interp.exec_line("2 3 +").unwrap();
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]

pub mod arithmetic;
pub mod constants;
#[cfg(feature = "crypto-ops")]
pub mod crypto;
pub mod io;
pub mod logic;
pub mod math;
#[cfg(feature = "quantum-ops")]
pub mod quantum;
pub mod stack;

use woflang_runtime::Interpreter;

/// Register all standard library operations with an interpreter.
///
/// This is the recommended way to initialize a fully-featured Woflang
/// environment. For minimal builds, register individual modules instead.
pub fn register_all(interp: &mut Interpreter) {
    arithmetic::register(interp);
    stack::register(interp);
    constants::register(interp);
    math::register(interp);
    logic::register(interp);
    io::register(interp);

    #[cfg(feature = "quantum-ops")]
    quantum::register(interp);

    #[cfg(feature = "crypto-ops")]
    crypto::register(interp);
}

/// Register only the minimal core operations.
///
/// Includes arithmetic, stack operations, and basic I/O.
pub fn register_core(interp: &mut Interpreter) {
    arithmetic::register(interp);
    stack::register(interp);
    io::register(interp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_all_works() {
        let mut interp = Interpreter::new();
        register_all(&mut interp);

        // Basic arithmetic should work
        interp.exec_line("2 3 +").unwrap();
        let result = interp.stack().peek().unwrap().as_integer().unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn register_core_works() {
        let mut interp = Interpreter::new();
        register_core(&mut interp);

        interp.exec_line("42 dup +").unwrap();
        let result = interp.stack().peek().unwrap().as_integer().unwrap();
        assert_eq!(result, 84);
    }
}
