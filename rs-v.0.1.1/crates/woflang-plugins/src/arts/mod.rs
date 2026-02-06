//! Arts operations for Woflang.
//!
//! Creative and artistic tools:
//! - **music** - Music theory (scales, chords, intervals, rhythms)

pub mod music;

use woflang_runtime::Interpreter;

/// Register all arts operations.
pub fn register(interp: &mut Interpreter) {
    music::register(interp);
}
