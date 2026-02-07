//! Games for Woflang.
//!
//! Interactive games and puzzles:
//! - **chess** - Simple chess engine with 3-ply AI

pub mod chess;

use woflang_runtime::Interpreter;

/// Register all games.
pub fn register(interp: &mut Interpreter) {
    chess::register(interp);
}
