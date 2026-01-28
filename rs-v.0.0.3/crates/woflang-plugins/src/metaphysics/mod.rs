//! Metaphysics module for Woflang.
//!
//! Philosophical and information-theoretic operations exploring
//! concepts like duality, entropy, learning, and (jokingly) over-unity.
//!
//! ## Submodules
//!
//! - `duality` - Logical and numeric duality operations
//! - `entropy` - Information entropy, chaos, and order
//! - `learning` - Interactive educational tools
//! - `over_unity` - Easter egg "free energy" operations
//!
//! ## Quick Reference
//!
//! ### Duality
//! ```text
//! duality_on / duality_off / duality_toggle
//! a b dual_add    # + when off, - when on
//! a b dual_and    # AND when off, OR when on
//! a b dual_or     # OR when off, AND when on
//! a dual_not      # NOT (self-dual)
//! "A and B" dual_logic  # → "A or B"
//! ```
//!
//! ### Entropy
//! ```text
//! entropy      # Shannon entropy of stack contents (bits)
//! chaos        # Randomly shuffle the stack
//! order        # Sort stack (numeric first, ascending)
//! unique_count # Count unique values
//! ```
//!
//! ### Learning
//! ```text
//! lesson    # Random learning tip
//! hint      # Context-aware hint
//! quiz      # Quiz question
//! examples  # Code snippets
//! tutorial  # Welcome message
//! ```
//!
//! ### Over Unity (Easter Eggs)
//! ```text
//! over_unity       # The mythical free energy device
//! perpetual_motion # Start the perpetual motion machine
//! thermodynamics   # Print laws of thermodynamics
//! maxwell_demon    # Sort molecules (but entropy still wins)
//! heat_death       # Fast-forward to the end of the universe
//! ```

mod duality;
mod entropy;
mod learning;
mod over_unity;

// use woflang_core::InterpreterContext;
use woflang_runtime::Interpreter;

/// Register all metaphysics operations.
pub fn register(interp: &mut Interpreter) {
    duality::register(interp);
    entropy::register(interp);
    learning::register(interp);
    over_unity::register(interp);

    // Help command
    interp.register("metaphysics_help", |_interp| {
        println!("Metaphysics Operations:");
        println!();
        println!("  Duality (☯️):");
        println!("    duality_on/off/toggle  # Control duality mode");
        println!("    duality?               # Check current mode");
        println!("    a b dual_add           # + when off, - when on");
        println!("    a b dual_and           # AND when off, OR when on");
        println!("    a b dual_or            # OR when off, AND when on");
        println!("    \"formula\" dual_logic   # Textual dualization");
        println!();
        println!("  Entropy:");
        println!("    entropy       # Shannon entropy of stack (bits)");
        println!("    entropy_max   # Maximum possible entropy");
        println!("    unique_count  # Count unique values");
        println!("    chaos         # Shuffle stack randomly");
        println!("    order         # Sort stack (numeric first)");
        println!("    sort_asc/desc # Simple numeric sort");
        println!();
        println!("  Learning:");
        println!("    lesson/lessons  # Learning tips");
        println!("    hint            # Context-aware hint");
        println!("    quiz/quizzes    # Quiz questions");
        println!("    examples        # Code snippets");
        println!("    tutorial        # Welcome message");
        println!("    quickstart      # Quick start guide");
        println!();
        println!("  Easter Eggs:");
        println!("    over_unity       # Free energy (doesn't work)");
        println!("    perpetual_motion # Perpetual motion (fails)");
        println!("    thermodynamics   # Laws of thermodynamics");
        println!("    maxwell_demon    # Sort molecules");
        println!("    heat_death       # End of the universe");
        Ok(())
    });
}
