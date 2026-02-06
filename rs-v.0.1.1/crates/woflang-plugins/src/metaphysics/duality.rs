//! Duality operations for Woflang.
//!
//! Implements logical and numeric dualities based on mathematical duality theory:
//! - In duality mode, AND ↔ OR, TRUE ↔ FALSE, + ↔ -
//!
//! ## Operations
//!
//! - `duality`, `duality_on`, `duality_off`, `duality_toggle` - Mode control
//! - `dual_add` - Addition (subtraction when duality on)
//! - `dual_and` - AND (OR when duality on)
//! - `dual_or` - OR (AND when duality on)
//! - `dual_not` - NOT (same in both modes, but logs state)
//! - `dual_logic` - Textual formula dualization

use std::sync::atomic::{AtomicBool, Ordering};
use woflang_core::{WofValue, InterpreterContext, WofType};
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// GLOBAL STATE
// ═══════════════════════════════════════════════════════════════════════════

/// Global duality mode flag.
static DUALITY_ENABLED: AtomicBool = AtomicBool::new(false);

fn duality_on() -> bool {
    DUALITY_ENABLED.load(Ordering::Relaxed)
}

fn set_duality(val: bool) {
    DUALITY_ENABLED.store(val, Ordering::Relaxed);
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Convert a WofValue to boolean (truthiness).
fn to_bool(v: &WofValue) -> bool {
    v.is_truthy()
}

/// Convert string to lowercase.
fn to_lower(s: &str) -> String {
    s.to_lowercase()
}

/// Dualize a logical formula (textual).
///
/// Swaps:
/// - "and" ↔ "or"
/// - "true" ↔ "false"
fn dualize_formula(formula: &str) -> String {
    let mut result = formula.to_string();
    let lower = to_lower(formula);
    
    // Use placeholders to avoid re-rewrites
    let replacements = [
        ("true", "##DUAL_TRUE##"),
        ("false", "##DUAL_FALSE##"),
        ("and", "##DUAL_AND##"),
        ("or", "##DUAL_OR##"),
    ];
    
    // First pass: replace with placeholders
    for (token, placeholder) in &replacements {
        result = replace_token_case_insensitive(&result, token, placeholder);
    }
    
    // Second pass: replace placeholders with duals
    result = result.replace("##DUAL_TRUE##", "false");
    result = result.replace("##DUAL_FALSE##", "true");
    result = result.replace("##DUAL_AND##", "or");
    result = result.replace("##DUAL_OR##", "and");
    
    result
}

/// Replace a token in a string (case-insensitive, word boundaries).
fn replace_token_case_insensitive(text: &str, token: &str, replacement: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let lower_text = to_lower(text);
    let lower_token = to_lower(token);
    
    let mut i = 0;
    let chars: Vec<char> = text.chars().collect();
    let lower_chars: Vec<char> = lower_text.chars().collect();
    
    while i < chars.len() {
        // Check if we might have a match
        if i + token.len() <= chars.len() {
            let slice: String = lower_chars[i..i + token.len()].iter().collect();
            
            if slice == lower_token {
                // Check word boundaries
                let left_ok = i == 0 || !lower_chars[i - 1].is_alphanumeric();
                let right_ok = i + token.len() >= chars.len() 
                    || !lower_chars[i + token.len()].is_alphanumeric();
                
                if left_ok && right_ok {
                    result.push_str(replacement);
                    i += token.len();
                    continue;
                }
            }
        }
        
        result.push(chars[i]);
        i += 1;
    }
    
    result
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register duality operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // MODE CONTROL
    // ─────────────────────────────────────────────────────────────────────

    // Turn duality mode on
    interp.register("duality_on", |_interp| {
        set_duality(true);
        println!("☯️  Duality mode: ON");
        Ok(())
    });

    // Turn duality mode off
    interp.register("duality_off", |_interp| {
        set_duality(false);
        println!("☯️  Duality mode: OFF");
        Ok(())
    });

    // Toggle duality mode
    interp.register("duality_toggle", |_interp| {
        let new_state = !duality_on();
        set_duality(new_state);
        println!("☯️  Duality mode toggled to: {}", if new_state { "ON" } else { "OFF" });
        Ok(())
    });

    // Legacy toggle name
    interp.register("duality", |_interp| {
        let new_state = !duality_on();
        set_duality(new_state);
        println!("☯️  duality: mode is now {}", if new_state { "ON" } else { "OFF" });
        Ok(())
    });

    // Check current mode
    interp.register("duality?", |interp| {
        let state = duality_on();
        println!("☯️  Duality mode is: {}", if state { "ON" } else { "OFF" });
        interp.stack_mut().push(WofValue::double(if state { 1.0 } else { 0.0 }));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // DUAL ARITHMETIC
    // ─────────────────────────────────────────────────────────────────────

    // Dual addition: + when off, - when on
    // Stack: a b → (a+b) or (a-b)
    interp.register("dual_add", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        
        let result = if duality_on() { a - b } else { a + b };
        
        interp.stack_mut().push(WofValue::double(result));
        Ok(())
    });

    // Dual subtraction: - when off, + when on
    // Stack: a b → (a-b) or (a+b)
    interp.register("dual_sub", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        
        let result = if duality_on() { a + b } else { a - b };
        
        interp.stack_mut().push(WofValue::double(result));
        Ok(())
    });

    // Dual multiplication: * when off, / when on
    // Stack: a b → (a*b) or (a/b)
    interp.register("dual_mul", |interp| {
        let b = interp.stack_mut().pop()?.as_double()?;
        let a = interp.stack_mut().pop()?.as_double()?;
        
        let result = if duality_on() {
            if b == 0.0 { f64::INFINITY } else { a / b }
        } else {
            a * b
        };
        
        interp.stack_mut().push(WofValue::double(result));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // DUAL LOGIC
    // ─────────────────────────────────────────────────────────────────────

    // Dual AND: AND when off, OR when on
    // Stack: a b → (a AND b) or (a OR b)
    interp.register("dual_and", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        
        let a_bool = to_bool(&a);
        let b_bool = to_bool(&b);
        
        let result = if duality_on() {
            a_bool || b_bool  // Dual: OR
        } else {
            a_bool && b_bool  // Normal: AND
        };
        
        interp.stack_mut().push(WofValue::integer(if result { 1 } else { 0 }));
        Ok(())
    });

    // Dual OR: OR when off, AND when on
    // Stack: a b → (a OR b) or (a AND b)
    interp.register("dual_or", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        
        let a_bool = to_bool(&a);
        let b_bool = to_bool(&b);
        
        let result = if duality_on() {
            a_bool && b_bool  // Dual: AND
        } else {
            a_bool || b_bool  // Normal: OR
        };
        
        interp.stack_mut().push(WofValue::integer(if result { 1 } else { 0 }));
        Ok(())
    });

    // Dual NOT: NOT in both modes (self-dual)
    // Stack: a → (NOT a)
    interp.register("dual_not", |interp| {
        let a = interp.stack_mut().pop()?;
        let a_bool = to_bool(&a);
        
        let result = !a_bool;
        
        println!(
            "☯️  dual_not (duality {}): {} -> {}",
            if duality_on() { "ON" } else { "OFF" },
            if a_bool { "true" } else { "false" },
            if result { "true" } else { "false" }
        );
        
        interp.stack_mut().push(WofValue::integer(if result { 1 } else { 0 }));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // TEXTUAL DUALIZATION
    // ─────────────────────────────────────────────────────────────────────

    // Dualize a formula string
    // Stack: "formula" → "dual_formula"
    interp.register("dual_logic", |interp| {
        let formula = interp.stack_mut().pop()?.as_string()?;
        let dual = dualize_formula(&formula);
        
        println!("☯️  dual_logic: \"{}\" -> \"{}\"", formula, dual);
        
        interp.stack_mut().push(WofValue::string(dual));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // DUAL CONSTANTS
    // ─────────────────────────────────────────────────────────────────────

    // Dual zero: 0 when off, infinity when on
    interp.register("dual_zero", |interp| {
        let result = if duality_on() { f64::INFINITY } else { 0.0 };
        interp.stack_mut().push(WofValue::double(result));
        Ok(())
    });

    // Dual one: 1 when off, 1 when on (multiplicative identity is self-dual)
    interp.register("dual_one", |interp| {
        interp.stack_mut().push(WofValue::double(1.0));
        Ok(())
    });

    // Dual infinity: infinity when off, 0 when on
    interp.register("dual_inf", |interp| {
        let result = if duality_on() { 0.0 } else { f64::INFINITY };
        interp.stack_mut().push(WofValue::double(result));
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dualize_formula() {
        assert_eq!(dualize_formula("A and B"), "A or B");
        assert_eq!(dualize_formula("true or false"), "false and true");
        assert_eq!(dualize_formula("AND OR"), "or and");
    }
}
