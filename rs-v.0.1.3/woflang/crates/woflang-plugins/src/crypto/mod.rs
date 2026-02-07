//! Cryptographic and number theory operations for Woflang.
//!
//! Includes prime checking, modular arithmetic, and basic crypto primitives.

mod primes;
mod modular;

use woflang_runtime::Interpreter;

/// Register all crypto operations with the interpreter.
pub fn register(interp: &mut Interpreter) {
    primes::register(interp);
    modular::register(interp);
}
