//! Entropy operations for Woflang.
//!
//! Information-theoretic and thermodynamic-inspired stack operations.
//!
//! ## Operations
//!
//! - `entropy` - Calculate Shannon entropy of stack contents
//! - `chaos` - Randomly shuffle the stack
//! - `order` - Sort the stack (numeric values first, ascending)
//! - `entropy_bits` - Entropy in bits
//! - `unique_count` - Count unique values on stack

use std::collections::HashMap;
use woflang_core::WofValue;
use woflang_runtime::Interpreter;
use rand::seq::SliceRandom;
use rand::thread_rng;

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Create a canonical string key for a WofValue (for counting purposes).
fn make_entropy_key(v: &WofValue) -> String {
    match v {
        WofValue::Integer(n) => format!("i:{}", n),
        WofValue::Float(f) => format!("f:{:.10}", f),
        WofValue::String(s) => format!("s:{}", s),
        WofValue::Bool(b) => format!("b:{}", b),
        WofValue::Nil => "nil".to_string(),
    }
}

/// Check if a value is numeric.
fn is_numeric(v: &WofValue) -> bool {
    matches!(v, WofValue::Integer(_) | WofValue::Float(_))
}

/// Extract numeric value (for sorting).
fn to_numeric(v: &WofValue) -> f64 {
    match v {
        WofValue::Integer(n) => *n as f64,
        WofValue::Float(f) => *f,
        _ => 0.0,
    }
}

/// Calculate Shannon entropy in bits.
fn shannon_entropy(counts: &HashMap<String, usize>, total: usize) -> f64 {
    if total == 0 {
        return 0.0;
    }
    
    let total_f = total as f64;
    let mut h = 0.0;
    
    for &count in counts.values() {
        if count > 0 {
            let p = count as f64 / total_f;
            h -= p * p.log2();
        }
    }
    
    h
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register entropy operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // ENTROPY CALCULATION
    // ─────────────────────────────────────────────────────────────────────

    // Calculate Shannon entropy of stack contents
    // Stack: ... → ... H
    interp.register("entropy", |interp| {
        let stack = interp.stack();
        let n = stack.len();
        
        if n == 0 {
            println!("[entropy] Empty stack => H = 0 bits");
            interp.stack_mut().push(WofValue::Float(0.0));
            return Ok(());
        }
        
        // Count occurrences
        let mut counts: HashMap<String, usize> = HashMap::new();
        
        for value in stack.iter() {
            let key = make_entropy_key(value);
            *counts.entry(key).or_insert(0) += 1;
        }
        
        let h = shannon_entropy(&counts, n);
        
        println!(
            "[entropy] {} values, {} unique symbols => H = {:.4} bits",
            n,
            counts.len(),
            h
        );
        
        interp.stack_mut().push(WofValue::Float(h));
        Ok(())
    });

    // Same as entropy but with different name
    interp.register("entropy_bits", |interp| {
        let stack = interp.stack();
        let n = stack.len();
        
        if n == 0 {
            interp.stack_mut().push(WofValue::Float(0.0));
            return Ok(());
        }
        
        let mut counts: HashMap<String, usize> = HashMap::new();
        for value in stack.iter() {
            let key = make_entropy_key(value);
            *counts.entry(key).or_insert(0) += 1;
        }
        
        let h = shannon_entropy(&counts, n);
        interp.stack_mut().push(WofValue::Float(h));
        Ok(())
    });

    // Maximum possible entropy for current stack
    // Stack: → H_max
    interp.register("entropy_max", |interp| {
        let n = interp.stack().len();
        if n == 0 {
            interp.stack_mut().push(WofValue::Float(0.0));
        } else {
            // Max entropy when all symbols are unique
            let h_max = (n as f64).log2();
            interp.stack_mut().push(WofValue::Float(h_max));
        }
        Ok(())
    });

    // Count unique values on stack
    // Stack: → count
    interp.register("unique_count", |interp| {
        let mut seen: HashMap<String, bool> = HashMap::new();
        
        for value in interp.stack().iter() {
            let key = make_entropy_key(value);
            seen.insert(key, true);
        }
        
        let count = seen.len() as i64;
        interp.stack_mut().push(WofValue::integer(count));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // CHAOS (SHUFFLE)
    // ─────────────────────────────────────────────────────────────────────

    // Randomly shuffle the stack
    // Stack: a b c ... → (randomly permuted)
    interp.register("chaos", |interp| {
        let stack = interp.stack_mut();
        
        if stack.is_empty() {
            println!("[chaos] Stack already empty, nothing to shuffle");
            return Ok(());
        }
        
        let len = stack.len();
        
        // Get mutable slice of stack contents
        let values: &mut [WofValue] = stack.as_mut_slice();
        values.shuffle(&mut thread_rng());
        
        println!("[chaos] Stack has been randomly permuted (size = {})", len);
        Ok(())
    });

    // Alias for chaos
    interp.register("shuffle", |interp| {
        let stack = interp.stack_mut();
        
        if stack.is_empty() {
            return Ok(());
        }
        
        let values: &mut [WofValue] = stack.as_mut_slice();
        values.shuffle(&mut thread_rng());
        
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // ORDER (SORT)
    // ─────────────────────────────────────────────────────────────────────

    // Sort the stack: numeric values first (ascending), then others
    // Stack: ... → (sorted)
    interp.register("order", |interp| {
        let stack = interp.stack_mut();
        
        if stack.is_empty() {
            println!("[order] Stack already empty, nothing to sort");
            return Ok(());
        }
        
        let len = stack.len();
        
        // Sort with custom comparator
        let values: &mut [WofValue] = stack.as_mut_slice();
        values.sort_by(|a, b| {
            let a_num = is_numeric(a);
            let b_num = is_numeric(b);
            
            match (a_num, b_num) {
                (true, true) => {
                    // Both numeric: compare values
                    let av = to_numeric(a);
                    let bv = to_numeric(b);
                    av.partial_cmp(&bv).unwrap_or(std::cmp::Ordering::Equal)
                }
                (true, false) => std::cmp::Ordering::Less,   // Numeric first
                (false, true) => std::cmp::Ordering::Greater,
                (false, false) => std::cmp::Ordering::Equal, // Preserve order
            }
        });
        
        println!("[order] Stack sorted; numeric values promoted (size = {})", len);
        Ok(())
    });

    // Sort ascending (simple numeric sort)
    interp.register("sort_asc", |interp| {
        let stack = interp.stack_mut();
        let values: &mut [WofValue] = stack.as_mut_slice();
        
        values.sort_by(|a, b| {
            let av = to_numeric(a);
            let bv = to_numeric(b);
            av.partial_cmp(&bv).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(())
    });

    // Sort descending
    interp.register("sort_desc", |interp| {
        let stack = interp.stack_mut();
        let values: &mut [WofValue] = stack.as_mut_slice();
        
        values.sort_by(|a, b| {
            let av = to_numeric(a);
            let bv = to_numeric(b);
            bv.partial_cmp(&av).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(())
    });

    // Reverse the stack
    interp.register("reverse_stack", |interp| {
        let stack = interp.stack_mut();
        let values: &mut [WofValue] = stack.as_mut_slice();
        values.reverse();
        Ok(())
    });
}
