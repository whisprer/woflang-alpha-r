//! Mathematical operations for Woflang.
//!
//! Includes trigonometry, exponentials, logarithms, calculus operations,
//! geometry, fractals, and general mathematical functions.

mod trig;
mod expo_log;
mod basic;
mod calculus;
mod discrete;
mod geometry;
mod gradient;
mod fractal;
mod greek;

use woflang_runtime::Interpreter;

/// Register all math operations with the interpreter.
pub fn register(interp: &mut Interpreter) {
    trig::register(interp);
    expo_log::register(interp);
    basic::register(interp);
    calculus::register(interp);
    discrete::register(interp);
    geometry::register(interp);
    gradient::register(interp);
    fractal::register(interp);
    greek::register(interp);
}
