//! Hebrew mode operations for Woflang.
//!
//! RTL (right-to-left) display mode and related fun:
//! - `hebrew_mode_on` / `hebrew_mode_off` - Toggle RTL mode
//! - `hebrew_echo` - Echo top-of-stack in pseudo-Hebrew (reversed)
//! - `hebrews_it` - Tell the classic Moses tea joke

use std::sync::atomic::{AtomicBool, Ordering};
use woflang_core::{WofValue, InterpreterContext, WofType};
use woflang_runtime::Interpreter;

/// Hebrew mode flag.
static HEBREW_MODE: AtomicBool = AtomicBool::new(false);

/// Check if Hebrew mode is active.
pub fn is_hebrew_mode() -> bool {
    HEBREW_MODE.load(Ordering::Relaxed)
}

/// Convert string to pseudo-Hebrew (reversed with RLM prefix).
fn to_pseudo_hebrew(s: &str) -> String {
    // U+200F = Right-to-Left Mark
    let rlm = '\u{200F}';
    let reversed: String = s.chars().rev().collect();
    format!("{}{}", rlm, reversed)
}

/// Format a WofValue as string.
fn value_to_string(v: &WofValue) -> String {
    format!("{}", v)
}

/// Register hebrew operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // HEBREW MODE TOGGLE
    // ═══════════════════════════════════════════════════════════════
    
    // Turn on Hebrew (RTL) mode
    interp.register("hebrew_mode_on", |_interp| {
        HEBREW_MODE.store(true, Ordering::Relaxed);
        println!("[hebrew_ops] Hebrew mode: ON (RTL mirroring enabled)");
        Ok(())
    });

    // Turn off Hebrew mode
    interp.register("hebrew_mode_off", |_interp| {
        HEBREW_MODE.store(false, Ordering::Relaxed);
        println!("[hebrew_ops] Hebrew mode: OFF");
        Ok(())
    });

    // Toggle Hebrew mode
    interp.register("hebrew_toggle", |_interp| {
        let now = !HEBREW_MODE.load(Ordering::Relaxed);
        HEBREW_MODE.store(now, Ordering::Relaxed);
        println!("[hebrew_ops] Hebrew mode: {}", if now { "ON" } else { "OFF" });
        Ok(())
    });

    // Query Hebrew mode
    interp.register("hebrew?", |interp| {
        let on = HEBREW_MODE.load(Ordering::Relaxed);
        interp.stack_mut().push(WofValue::integer(if on { 1 } else { 0 }));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // HEBREW ECHO
    // ═══════════════════════════════════════════════════════════════
    
    // Echo top-of-stack as pseudo-Hebrew (reversed if mode is on)
    // Stack: value → echoed_string
    interp.register("hebrew_echo", |interp| {
        if interp.stack().is_empty() {
            println!("[hebrew_ops] hebrew_echo: stack is empty.");
            return Ok(());
        }
        
        let value = interp.stack_mut().pop()?;
        let s = value_to_string(&value);
        
        let out = if HEBREW_MODE.load(Ordering::Relaxed) {
            to_pseudo_hebrew(&s)
        } else {
            s
        };
        
        println!("{}", out);
        
        interp.stack_mut().push(WofValue::string(out));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // THE MOSES TEA JOKE
    // ═══════════════════════════════════════════════════════════════
    
    // Tell the famous joke: "How does Moses take his tea? He brews it!"
    // Stack: → joke_string
    interp.register("hebrews_it", |interp| {
        let joke = "How does Moses take his tea? He brews it!";
        
        let out = if HEBREW_MODE.load(Ordering::Relaxed) {
            to_pseudo_hebrew(joke)
        } else {
            joke.to_string()
        };
        
        println!("{}", out);
        
        interp.stack_mut().push(WofValue::string(out));
        Ok(())
    });

    // Alternative name for the joke
    interp.register("moses_tea", |interp| {
        let joke = "☕ He brews it! 🌊";
        println!("{}", joke);
        interp.stack_mut().push(WofValue::string(joke.to_string()));
        Ok(())
    });
}
