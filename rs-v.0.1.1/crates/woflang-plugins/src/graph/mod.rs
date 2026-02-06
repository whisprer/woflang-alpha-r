//! Graph theory operations for Woflang.
//!
//! Provides graph creation, manipulation, search algorithms (BFS/DFS),
//! shortest path (Dijkstra), and graph coloring.

mod core;
mod search;
mod weighted;
mod coloring;

use woflang_runtime::Interpreter;

/// Register all graph operations with the interpreter.
pub fn register(interp: &mut Interpreter) {
    core::register(interp);
    search::register(interp);
    weighted::register(interp);
    coloring::register(interp);
}
