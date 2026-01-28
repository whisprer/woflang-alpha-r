//! Plugin system for Woflang.
//!
//! This module provides the infrastructure for registering operations
//! with the Woflang interpreter. Plugins are organized by category
//! and can be enabled/disabled via Cargo features.
//!
//! # Example
//!
//! ```ignore
//! use woflang_plugins::register_all;
//! use woflang_runtime::Interpreter;
//!
//! let mut interp = Interpreter::new();
//! register_all(&mut interp);
//! ```

#![allow(clippy::module_name_repetitions)]

#[cfg(feature = "math")]
pub mod math;

#[cfg(feature = "util")]
pub mod util;

#[cfg(feature = "crypto")]
pub mod crypto;

#[cfg(feature = "logic")]
pub mod logic;

#[cfg(feature = "graph")]
pub mod graph;

#[cfg(feature = "sigils")]
pub mod sigils;

#[cfg(feature = "language")]
pub mod language;

#[cfg(feature = "arts")]
pub mod arts;

#[cfg(feature = "science")]
pub mod science;

#[cfg(feature = "games")]
pub mod games;

#[cfg(feature = "solver")]
pub mod solver;

#[cfg(feature = "metaphysics")]
pub mod metaphysics;

#[cfg(feature = "quantum")]
pub mod quantum;

#[cfg(feature = "markov")]
pub mod markov;

#[cfg(feature = "neural_chess")]
pub mod neural_chess;

pub mod data;

use woflang_runtime::Interpreter;
use woflang_core::InterpreterContext;

/// Register all enabled plugins with the interpreter.
pub fn register_all(interp: &mut Interpreter) {
    #[cfg(feature = "math")]
    math::register(interp);

    #[cfg(feature = "util")]
    util::register(interp);

    #[cfg(feature = "crypto")]
    crypto::register(interp);

    #[cfg(feature = "logic")]
    logic::register(interp);

    #[cfg(feature = "graph")]
    graph::register(interp);

    #[cfg(feature = "sigils")]
    sigils::register(interp);

    #[cfg(feature = "language")]
    language::register(interp);

    #[cfg(feature = "arts")]
    arts::register(interp);

    #[cfg(feature = "science")]
    science::register(interp);

    #[cfg(feature = "games")]
    games::register(interp);

    #[cfg(feature = "solver")]
    solver::register(interp);

    #[cfg(feature = "metaphysics")]
    metaphysics::register(interp);

    #[cfg(feature = "quantum")]
    quantum::register(interp);

    #[cfg(feature = "markov")]
    markov::register(interp);

    #[cfg(feature = "neural_chess")]
    neural_chess::register(interp);
}

/// Helper macro for registering a unary numeric operation.
///
/// Takes a value from the stack, applies the function, pushes the result.
#[macro_export]
macro_rules! register_unary {
    ($interp:expr, $name:expr, $func:expr) => {
        $interp.register($name, |interp| {
            let a = interp.stack_mut().pop()?.as_float()?;
            let result = $func(a);
            interp.stack_mut().push(woflang_core::WofValue::double(result));
            Ok(())
        });
    };
}

/// Helper macro for registering a binary numeric operation.
///
/// Takes two values from the stack (b then a), applies the function, pushes the result.
#[macro_export]
macro_rules! register_binary {
    ($interp:expr, $name:expr, $func:expr) => {
        $interp.register($name, |interp| {
            let b = interp.stack_mut().pop()?.as_float()?;
            let a = interp.stack_mut().pop()?.as_float()?;
            let result = $func(a, b);
            interp.stack_mut().push(woflang_core::WofValue::double(result));
            Ok(())
        });
    };
}

/// Helper macro for registering a constant.
///
/// Pushes a constant value onto the stack.
#[macro_export]
macro_rules! register_const {
    ($interp:expr, $name:expr, $value:expr) => {
        $interp.register($name, |interp| {
            interp.stack_mut().push(woflang_core::WofValue::double($value));
            Ok(())
        });
    };
}

/// Helper macro for registering a string constant.
#[macro_export]
macro_rules! register_str_const {
    ($interp:expr, $name:expr, $value:expr) => {
        $interp.register($name, |interp| {
            interp.stack_mut().push(woflang_core::WofValue::string($value));
            Ok(())
        });
    };
}
