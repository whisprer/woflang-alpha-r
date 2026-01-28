//! Markov-based suggestion system for Woflang.
//!
//! Provides context-aware suggestions and autocomplete hints
//! based on common patterns and best practices.
//!
//! ## Operations
//!
//! - `markov_suggest` - Random math expression suggestion
//! - `suggest_math` - Math pattern suggestion
//! - `suggest_next` - Suggest next operation based on stack
//! - `suggest_complete` - Autocomplete suggestions for partial input

use rand::seq::SliceRandom;
use rand::thread_rng;
use woflang_core::{InterpreterContext, WofValue};
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// SUGGESTION DATABASES
// ═══════════════════════════════════════════════════════════════════════════

/// Math expression suggestions
const MATH_SUGGESTIONS: &[&str] = &[
    "Try: X X +          # Double a value",
    "Try: pi * r r *     # Area of circle (πr²)",
    "Try: X X *          # Square a value",
    "Try: a b + c +      # Add three numbers",
    "Try: sqrt Y         # Square root",
    "Try: a b / 100 *    # Percentage",
    "Try: 2 3 ^ 4 5 ^ +  # Sum of powers",
    "Try: n fact         # Factorial",
    "Try: a b c solve_quadratic  # Solve ax²+bx+c=0",
    "Try: degrees radians  # Convert to radians",
];

/// Stack operation suggestions
const STACK_SUGGESTIONS: &[&str] = &[
    "Try: dup            # Duplicate top value",
    "Try: swap           # Swap top two values",
    "Try: over           # Copy second value to top",
    "Try: rot            # Rotate top three values",
    "Try: drop           # Remove top value",
    "Try: nip            # Remove second value",
    "Try: tuck           # Copy top under second",
];

/// Control flow suggestions
const CONTROL_SUGGESTIONS: &[&str] = &[
    "Try: cond { true } ? { false } !  # Conditional",
    "Try: n { body } loop              # Loop n times",
    "Try: { cond } { body } while      # While loop",
    "Try: func name { body }           # Define function",
];

/// Greek symbol suggestions
const GREEK_SUGGESTIONS: &[&str] = &[
    "Try: π             # Pi constant",
    "Try: τ             # Tau (2π)",
    "Try: φ             # Golden ratio",
    "Try: ∞             # Infinity",
    "Try: √ X           # Square root of X",
    "Try: Σ n values    # Sum of n values",
    "Try: Π n values    # Product of n values",
    "Try: Δ a b         # Difference (b - a)",
];

/// Plugin suggestions based on stack state
const EMPTY_STACK_SUGGESTIONS: &[&str] = &[
    "Stack is empty. Try: 42",
    "Push a value: 3.14159",
    "Try: π (push pi)",
    "Load some values: 1 2 3",
];

const ONE_VALUE_SUGGESTIONS: &[&str] = &[
    "One value on stack. Try: dup (duplicate)",
    "Try: sqrt (square root)",
    "Try: print (display value)",
    "Try: X X * (square the value)",
];

const TWO_VALUE_SUGGESTIONS: &[&str] = &[
    "Two values on stack. Try: + (add)",
    "Try: * (multiply)",
    "Try: swap (exchange)",
    "Try: / (divide)",
];

/// Transition probabilities for Markov-like suggestions
/// (operation, likely_next_operations)
const TRANSITIONS: &[(&str, &[&str])] = &[
    ("+", &["print", "dup", "*", "-"]),
    ("*", &["print", "+", "sqrt", "/"]),
    ("dup", &["*", "+", "swap", "over"]),
    ("swap", &["drop", "-", "/", "over"]),
    ("print", &["drop", "clear", "+"]),
    ("sqrt", &["print", "round", "*"]),
    ("sin", &["cos", "print", "*"]),
    ("cos", &["sin", "print", "+"]),
];

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Get a random suggestion from a list.
fn random_suggestion<'a>(suggestions: &[&'a str]) -> Option<&'a str> {
    suggestions.choose(&mut thread_rng()).copied()
}

/// Get suggestions based on stack size.
fn stack_based_suggestions(stack_len: usize) -> &'static [&'static str] {
    match stack_len {
        0 => EMPTY_STACK_SUGGESTIONS,
        1 => ONE_VALUE_SUGGESTIONS,
        2 => TWO_VALUE_SUGGESTIONS,
        _ => MATH_SUGGESTIONS,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register markov suggestion operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // BASIC SUGGESTIONS
    // ─────────────────────────────────────────────────────────────────────

    // Random math suggestion
    interp.register("markov_suggest", |_interp| {
        if let Some(suggestion) = random_suggestion(MATH_SUGGESTIONS) {
            println!("[Markov Suggestion] {}", suggestion);
        }
        Ok(())
    });

    // Math pattern suggestion (alias)
    interp.register("suggest_math", |_interp| {
        if let Some(suggestion) = random_suggestion(MATH_SUGGESTIONS) {
            println!("[Suggest] {}", suggestion);
        }
        Ok(())
    });

    // Stack operation suggestion
    interp.register("suggest_stack", |_interp| {
        if let Some(suggestion) = random_suggestion(STACK_SUGGESTIONS) {
            println!("[Suggest] {}", suggestion);
        }
        Ok(())
    });

    // Control flow suggestion
    interp.register("suggest_control", |_interp| {
        if let Some(suggestion) = random_suggestion(CONTROL_SUGGESTIONS) {
            println!("[Suggest] {}", suggestion);
        }
        Ok(())
    });

    // Greek symbol suggestion
    interp.register("suggest_greek", |_interp| {
        if let Some(suggestion) = random_suggestion(GREEK_SUGGESTIONS) {
            println!("[Suggest] {}", suggestion);
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // CONTEXT-AWARE SUGGESTIONS
    // ─────────────────────────────────────────────────────────────────────

    // Suggest based on current stack state
    interp.register("suggest_next", |interp| {
        let suggestions = stack_based_suggestions(interp.stack().len());
        if let Some(suggestion) = random_suggestion(suggestions) {
            println!("[Suggest] {}", suggestion);
        }
        Ok(())
    });

    // Suggest based on stack contents
    interp.register("suggest_smart", |interp| {
        let stack_len = interp.stack().len();

        let suggestion = match stack_len {
            0 => "Try pushing some values: 1 2 3",
            1 => {
                // Check if it's a number we can do something with
                if let Ok(top) = interp.stack().peek() {
                    match top {
                        WofValue::Integer(n) if *n > 0 => "Try: fact (factorial), sqrt, dup *",
                        WofValue::Float(f) if *f >= 0.0 => "Try: sqrt, sin, cos, round",
                        WofValue::String(_) => "Try: print, strlen, upper",
                        _ => "Try: dup, print",
                    }
                } else {
                    "Try: dup, print"
                }
            }
            2 => "Try: + (add), - (subtract), * (multiply), / (divide), swap",
            3..=10 => "Try: + (add all), sort_asc, entropy, chaos",
            _ => "Many values! Try: clear, entropy, sum",
        };

        println!("[Smart Suggest] {}", suggestion);
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // AUTOCOMPLETE SUGGESTIONS
    // ─────────────────────────────────────────────────────────────────────

    // Get autocomplete suggestions for a prefix
    // Stack: "prefix" → (prints suggestions)
    interp.register("suggest_complete", |interp| {
        let prefix = interp.stack_mut().pop()?.as_string()?;
        let prefix_lower = prefix.to_lowercase();

        // Common operations to suggest
        let operations = [
            "add", "sub", "mul", "div", "mod",
            "dup", "drop", "swap", "over", "rot",
            "print", "println", "stack",
            "sin", "cos", "tan", "sqrt", "pow",
            "pi", "tau", "phi", "e",
            "entropy", "chaos", "order",
            "duality", "quantum", "measure",
        ];

        let matches: Vec<&str> = operations
            .iter()
            .filter(|op| op.starts_with(&prefix_lower))
            .copied()
            .collect();

        if matches.is_empty() {
            println!("[Autocomplete] No matches for '{}'", prefix);
        } else {
            println!("[Autocomplete] Matches for '{}':", prefix);
            for m in &matches {
                println!("  {}", m);
            }
        }

        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // MARKOV CHAIN SUGGESTIONS
    // ─────────────────────────────────────────────────────────────────────

    // Given last operation, suggest next
    // Stack: "last_op" → (prints suggestions)
    interp.register("suggest_after", |interp| {
        let last_op = interp.stack_mut().pop()?.as_string()?;
        let last_op_lower = last_op.to_lowercase();

        // Find transition
        for (op, nexts) in TRANSITIONS {
            if *op == last_op_lower {
                if let Some(next) = random_suggestion(nexts) {
                    println!("[Markov] After '{}', try: {}", last_op, next);
                    return Ok(());
                }
            }
        }

        // Default suggestion
        println!("[Markov] After '{}', try: print, dup, or +", last_op);
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // ALL SUGGESTIONS
    // ─────────────────────────────────────────────────────────────────────

    // Show all suggestion categories
    interp.register("suggest_all", |_interp| {
        println!("Math Suggestions:");
        for s in MATH_SUGGESTIONS {
            println!("  {}", s);
        }
        println!();
        println!("Stack Suggestions:");
        for s in STACK_SUGGESTIONS {
            println!("  {}", s);
        }
        println!();
        println!("Greek Suggestions:");
        for s in GREEK_SUGGESTIONS {
            println!("  {}", s);
        }
        Ok(())
    });

    // Random suggestion from any category
    interp.register("suggest", |_interp| {
        let all_suggestions: Vec<&str> = MATH_SUGGESTIONS
            .iter()
            .chain(STACK_SUGGESTIONS.iter())
            .chain(GREEK_SUGGESTIONS.iter())
            .copied()
            .collect();

        if let Some(suggestion) = random_suggestion(&all_suggestions) {
            println!("[Suggest] {}", suggestion);
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // HELP
    // ─────────────────────────────────────────────────────────────────────

    interp.register("markov_help", |_interp| {
        println!("Markov Suggestion Operations:");
        println!();
        println!("  Basic Suggestions:");
        println!("    markov_suggest   # Random math suggestion");
        println!("    suggest_math     # Math pattern");
        println!("    suggest_stack    # Stack operations");
        println!("    suggest_control  # Control flow");
        println!("    suggest_greek    # Greek symbols");
        println!();
        println!("  Context-Aware:");
        println!("    suggest_next     # Based on stack size");
        println!("    suggest_smart    # Based on stack contents");
        println!();
        println!("  Autocomplete:");
        println!("    \"pr\" suggest_complete  # Find matches");
        println!("    \"+\" suggest_after       # Next operation");
        println!();
        println!("  Other:");
        println!("    suggest          # Random from all");
        println!("    suggest_all      # Show all suggestions");
        Ok(())
    });
}
