//! Science operations for Woflang.
//!
//! Scientific tools and calculations:
//! - **chemistry** - Elements, molecular weights, temperature conversion

pub mod chemistry;

use woflang_runtime::Interpreter;

/// Register all science operations.
pub fn register(interp: &mut Interpreter) {
    chemistry::register(interp);
}
