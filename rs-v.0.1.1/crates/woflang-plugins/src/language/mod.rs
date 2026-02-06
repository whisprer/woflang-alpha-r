//! Language and unicode operations for Woflang.
//!
//! Provides kanji lookup/learning tools and cyrillic alphabet tools.

mod kanji;
mod cyrillic;

use woflang_runtime::Interpreter;

/// Register all language operations with the interpreter.
pub fn register(interp: &mut Interpreter) {
    kanji::register(interp);
    cyrillic::register(interp);
}
