//! # Woflang Runtime
//!
//! This crate provides the runtime interpreter for the Woflang stack-based
//! programming language. It handles:
//!
//! - **Tokenization**: Converting source text into tokens
//! - **Interpretation**: Executing tokens against the stack machine
//! - **Plugin System**: Extensible operation registration
//!
//! ## Architecture
//!
//! The runtime follows a straightforward execution model:
//!
//! ```text
//! Source → Tokenizer → Tokens → Interpreter → Stack
//! ```
//!
//! Operations are registered via the [`Registry`] and dispatched by name
//! during interpretation. The plugin system supports both compile-time
//! registration (preferred) and dynamic loading (feature-gated).
//!
//! ## Usage
//!
//! ```
//! use woflang_runtime::{Interpreter, Registry};
//! use woflang_core::WofValue;
//!
//! let mut registry = Registry::new();
//! registry.register("double", |interp| {
//!     let val = interp.stack_mut().pop_numeric()?;
//!     interp.stack_mut().push(WofValue::double(val * 2.0));
//!     Ok(())
//! });
//!
//! let mut interp = Interpreter::with_registry(registry);
//! interp.exec_line("21 double").unwrap();
//!
//! assert_eq!(interp.stack().peek().unwrap().as_double().unwrap(), 42.0);
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]

mod interpreter;
mod keybind;
#[cfg(feature = "dynamic-plugins")]
mod plugin;
mod registry;
mod tokenizer;

pub use interpreter::{FunctionDef, Interpreter, LoopType, OwnedToken};
pub use keybind::KeyBindings;
#[cfg(feature = "dynamic-plugins")]
pub use plugin::PluginLoader;
pub use registry::{OpFn, Registry};
pub use tokenizer::{Token, TokenKind, Tokenizer};

/// Re-export core types for convenience.
pub mod core {
    pub use woflang_core::*;
}
