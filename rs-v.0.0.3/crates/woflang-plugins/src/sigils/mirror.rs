//! Mirror operations for Woflang.
//!
//! Reverse stack view mode:
//! - `:mirror` - Toggle mirror mode and reverse the stack

use std::sync::atomic::{AtomicBool, Ordering};
use woflang_core::WofValue;
use woflang_runtime::Interpreter;

/// Mirror mode flag.
static MIRROR_MODE: AtomicBool = AtomicBool::new(false);

/// Check if mirror mode is active.
pub fn is_mirror_mode() -> bool {
    MIRROR_MODE.load(Ordering::Relaxed)
}

/// Register mirror operations.
pub fn register(interp: &mut Interpreter) {
    // Toggle mirror mode and reverse the stack
    // Stack: a b c → c b a + 0|1
    interp.register(":mirror", |interp| {
        let now = !MIRROR_MODE.load(Ordering::Relaxed);
        MIRROR_MODE.store(now, Ordering::Relaxed);
        
        // Collect all values
        let mut values = Vec::new();
        while !interp.stack().is_empty() {
            values.push(interp.stack_mut().pop().unwrap());
        }
        // values is now in reverse order (top first)
        // To reverse the stack, we push them back in the same order
        // (which puts the old top at the bottom)
        for v in values {
            interp.stack_mut().push(v);
        }
        
        println!();
        println!("🪞 Reverse-stack mode {}.", if now { "enabled" } else { "disabled" });
        println!("   Top and bottom have swapped stories.");
        println!();
        
        interp.stack_mut().push(WofValue::integer(if now { 1 } else { 0 }));
        Ok(())
    });

    // Just reverse the stack without toggling mode
    // Stack: a b c → c b a
    interp.register("reverse", |interp| {
        let mut values = Vec::new();
        while !interp.stack().is_empty() {
            values.push(interp.stack_mut().pop().unwrap());
        }
        for v in values {
            interp.stack_mut().push(v);
        }
        Ok(())
    });

    // Query mirror mode
    interp.register("mirror?", |interp| {
        let on = MIRROR_MODE.load(Ordering::Relaxed);
        interp.stack_mut().push(WofValue::integer(if on { 1 } else { 0 }));
        Ok(())
    });

    // Palindrome check for strings
    // Stack: string → 0|1
    interp.register("palindrome?", |interp| {
        let val = interp.stack_mut().pop()?;
        let s = match val {
            WofValue::String(s) => s,
            WofValue::Integer(n) => n.to_string(),
            _ => return Ok(()),
        };
        
        let cleaned: String = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase().next().unwrap_or(c))
            .collect();
        
        let reversed: String = cleaned.chars().rev().collect();
        let is_palindrome = cleaned == reversed;
        
        interp.stack_mut().push(WofValue::integer(if is_palindrome { 1 } else { 0 }));
        Ok(())
    });
}
