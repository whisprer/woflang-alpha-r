//! # Woflang Core
//!
//! Core types and traits for the Woflang stack-based programming language.
//!
//! This crate provides:
//! - [`WofValue`]: The primary value type with SIMD-aligned memory layout
//! - [`WofStack`]: A type-safe stack abstraction
//! - [`WofError`]: Comprehensive error types via `thiserror`
//! - [`OpHandler`]: The trait for operation implementations
//! - [`Opcode`]: The complete set of language operations
//! - [`BlockRegistry`]: Block tracking for structured control flow
//! - [`ScopeStack`]: Lexical scoping with variable bindings
//!
//! ## Memory Layout
//!
//! All value types are aligned to 16 bytes for SIMD compatibility. The
//! discriminated union uses a compact representation optimized for cache
//! locality when processing batches of values.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]

mod block;
mod error;
mod instruction;
mod opcode;
mod scope;
mod span;
mod stack;
mod unit;
mod value;

pub use block::{BlockId, BlockInfo, BlockRegistry, BlockStack, BlockType};
pub use error::{Result, WofError};
pub use instruction::{Instruction, Operand, Program};
pub use opcode::{Opcode, OpcodeCategory};
pub use scope::{Scope, ScopeId, ScopeStack};
pub use span::{Span, Spanned};
pub use stack::WofStack;
pub use unit::UnitInfo;
pub use value::{WofType, WofValue};

/// Version information for the Woflang runtime.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Operation handler function signature.
///
/// Operations receive a mutable reference to the interpreter context
/// and may return an error if execution fails.
pub type OpHandler<Ctx> = fn(&mut Ctx) -> Result<()>;

/// Trait for types that can act as an interpreter context.
///
/// This abstraction allows operations to be defined generically over
/// any interpreter implementation that provides stack access.
pub trait InterpreterContext {
    /// Push a value onto the stack.
    fn push(&mut self, value: WofValue);

    /// Pop a value from the stack, returning an error if empty.
    fn pop(&mut self) -> Result<WofValue>;

    /// Peek at the top value without removing it.
    fn peek(&self) -> Result<&WofValue>;

    /// Check if the stack has at least `n` values.
    fn has(&self, n: usize) -> bool;

    /// Get immutable access to the entire stack.
    fn stack(&self) -> &WofStack;

    /// Get mutable access to the entire stack.
    fn stack_mut(&mut self) -> &mut WofStack;

    /// Clear the stack.
    fn clear(&mut self);

    /// Signal an error with a message.
    fn error(&self, msg: impl Into<String>) -> WofError {
        WofError::Runtime(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_semver() {
        let parts: Vec<&str> = VERSION.split('.').collect();
        assert_eq!(parts.len(), 3);
        for part in parts {
            assert!(part.parse::<u32>().is_ok());
        }
    }
}
