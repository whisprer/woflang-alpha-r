//! Forbidden operations for Woflang.
//!
//! Dangerous and mystical stack-destroying operations:
//! - `void_division` - Divide by void (clears stack, leaves infinity)
//! - `/0` - Quick divide by zero
//! - `stack_slayer` - Destroy the entire stack
//! - `resurrect` - Bring back sacred constants from the void
//! - `glyph_prophecy` - The encrypted glyph prophecy
//! - `forbidden_echo` - Echo the last forbidden message

use std::sync::{Mutex, OnceLock};
use std::f64::consts::PI;
use woflang_core::{WofValue, InterpreterContext, WofType};
use woflang_runtime::Interpreter;

/// The last forbidden message.
fn last_message() -> &'static Mutex<String> {
    static MSG: OnceLock<Mutex<String>> = OnceLock::new();
    MSG.get_or_init(|| Mutex::new(String::new()))
}

/// Register forbidden operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // VOID DIVISION
    // ═══════════════════════════════════════════════════════════════
    
    // Divide by the void (forbidden operation)
    // Stack: a b → ∞ (clears everything, leaves infinity)
    interp.register("void_division", |interp| {
        // Store the forbidden message
        if let Ok(mut msg) = last_message().lock() {
            *msg = "You have peered into the void.".to_string();
        }
        
        println!("⚠️  FORBIDDEN OPERATION DETECTED ⚠️");
        println!("Attempting to divide by the void...");
        
        if interp.stack().len() < 2 {
            println!("The void requires a sacrifice.");
            return Ok(());
        }
        
        // Get the values
        let _divisor = interp.stack_mut().pop()?;
        let dividend = interp.stack_mut().pop()?;
        
        let dividend_val = dividend.as_double().unwrap_or(0.0);
        
        println!("Dividing {} by the essence of nothingness...", dividend_val);
        
        // The void consumes all
        interp.stack_mut().clear();
        
        // But leaves behind infinity
        interp.stack_mut().push(WofValue::double(f64::INFINITY));
        
        println!("The operation succeeds. Infinity remains.");
        println!("You have gazed into the abyss.");
        
        Ok(())
    });

    // Quick divide by zero
    // Stack: a → ∞
    interp.register("/0", |interp| {
        if interp.stack().is_empty() {
            println!("Even the void requires something to consume.");
            return Ok(());
        }
        
        let value = interp.stack_mut().pop()?;
        let numeric_val = value.as_double().unwrap_or(0.0);
        
        println!("÷0: {} → ∞", numeric_val);
        
        interp.stack_mut().push(WofValue::double(f64::INFINITY));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // STACK SLAYER
    // ═══════════════════════════════════════════════════════════════
    
    // Destroy the entire stack (dramatic version)
    // Stack: ... →
    interp.register("stack_slayer", |interp| {
        if let Ok(mut msg) = last_message().lock() {
            *msg = "The stack has been slain.".to_string();
        }
        
        if interp.stack().is_empty() {
            println!("⚔️  The Stack Slayer finds nothing to slay.");
            return Ok(());
        }
        
        println!("⚔️  THE STACK SLAYER AWAKENS! ⚔️");
        
        let victims = interp.stack().len();
        
        // Print dramatic effect
        for _ in 0..victims.min(10) {
            print!("💀 ");
        }
        println!();
        
        interp.stack_mut().clear();
        
        println!("⚰️  The Stack Slayer has claimed {} victims. The stack lies empty.", victims);
        
        Ok(())
    });

    // Alternative name with skull emoji
    interp.register("☠", |interp| {
        if let Ok(mut msg) = last_message().lock() {
            *msg = "The skull claims all.".to_string();
        }
        
        println!("☠ The stack perishes.");
        interp.stack_mut().clear();
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // RESURRECT
    // ═══════════════════════════════════════════════════════════════
    
    // Bring back sacred constants from the void
    // Stack: → π e φ
    interp.register("resurrect", |interp| {
        println!("✨ Attempting resurrection ritual...");
        
        // Resurrect with mystical constants
        let e = std::f64::consts::E;
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;  // Golden ratio
        
        interp.stack_mut().push(WofValue::double(PI));   // π
        interp.stack_mut().push(WofValue::double(e));    // e
        interp.stack_mut().push(WofValue::double(phi));  // φ
        
        println!("✨ Three sacred constants have risen from the void.");
        println!("   π ≈ {:.5}", PI);
        println!("   e ≈ {:.5}", e);
        println!("   φ ≈ {:.5}", phi);
        
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // GLYPH PROPHECY
    // ═══════════════════════════════════════════════════════════════
    
    // The encrypted glyph prophecy (void division warning)
    interp.register("glyph_prophecy", |_interp| {
        println!("[Forbidden] The encrypted glyph prophecy divides the stack void.");
        println!("            Beware division by zero!");
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // FORBIDDEN ECHO
    // ═══════════════════════════════════════════════════════════════
    
    // Echo the last forbidden message
    // Stack: →
    interp.register("forbidden_echo", |_interp| {
        if let Ok(msg) = last_message().lock() {
            if msg.is_empty() {
                println!("∅∅  No forbidden op to echo.");
            } else {
                println!("∅∅  Forbidden echo (inverted): {} (now returned to you)", *msg);
            }
        }
        Ok(())
    });

    // Void echo - requires zero on stack
    // Stack: 0 → (cleared)
    interp.register("void_echo", |interp| {
        if let Ok(mut msg) = last_message().lock() {
            *msg = "You have peered into the void.".to_string();
        }
        
        if interp.stack().is_empty() {
            println!("∅  You have peered into the void. (stack erased)");
            interp.stack_mut().clear();
            return Ok(());
        }
        
        if let Ok(top) = interp.stack().peek() {
            let is_zero = if let Some(n) = top.try_integer() { n == 0 }
                else if let Some(f) = top.try_double() { f == 0.0 }
                else { false };
            
            if is_zero {
                println!("∅  You have peered into the void. (stack erased)");
                interp.stack_mut().clear();
            } else {
                println!("∅  Only the zero can echo the void.");
            }
        }
        
        Ok(())
    });
}
