//! Utility operations for Woflang.
//!
//! Includes stack manipulation, I/O, debugging, and assertion helpers.

mod stack;
mod io;
mod assert;

use woflang_runtime::Interpreter;

/// Register all utility operations with the interpreter.
pub fn register(interp: &mut Interpreter) {
    stack::register(interp);
    io::register(interp);
    assert::register(interp);
}
