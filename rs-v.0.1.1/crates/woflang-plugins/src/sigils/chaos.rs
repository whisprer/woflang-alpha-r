//! Chaos mode operations for Woflang.
//!
//! Session state toggles for forbidden and experimental modes:
//! - `:unlock` - Unlock forbidden glyphs
//! - `:chaos?` - Query chaos state
//! - `:glitchmode` - Toggle random glyph glitching
//! - `:glitch-echo` - Echo with random substitutions
//! - `:deity` - Toggle divine recursion mode

use std::sync::atomic::{AtomicBool, Ordering};
use woflang_core::{WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

// Global state flags
static CHAOS_UNLOCKED: AtomicBool = AtomicBool::new(false);
static GLITCH_MODE: AtomicBool = AtomicBool::new(false);
static DEITY_MODE: AtomicBool = AtomicBool::new(false);

/// Get a random glyph character for glitching.
fn random_glyph_char() -> char {
    use std::time::{SystemTime, UNIX_EPOCH};
    static GLYPHS: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*', '+', '=', '?', '/', '\\', '|', '~'];
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    GLYPHS[(nanos as usize) % GLYPHS.len()]
}

/// Query if chaos is unlocked.
pub fn is_chaos_unlocked() -> bool {
    CHAOS_UNLOCKED.load(Ordering::Relaxed)
}

/// Query if glitch mode is on.
pub fn is_glitch_mode() -> bool {
    GLITCH_MODE.load(Ordering::Relaxed)
}

/// Query if deity mode is on.
pub fn is_deity_mode() -> bool {
    DEITY_MODE.load(Ordering::Relaxed)
}

/// Register chaos operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // CHAOS UNLOCK
    // ═══════════════════════════════════════════════════════════════
    
    // Unlock forbidden glyphs for this session
    // Stack: → 1
    interp.register(":unlock", |interp| {
        CHAOS_UNLOCKED.store(true, Ordering::Relaxed);
        
        println!();
        println!("⚡ Forbidden glyphs unlocked for this session.");
        println!("   Use with reverence; the stack remembers.");
        println!();
        
        interp.stack_mut().push(WofValue::integer(1));
        Ok(())
    });

    // Query chaos state
    // Stack: → 0|1
    interp.register(":chaos?", |interp| {
        let on = CHAOS_UNLOCKED.load(Ordering::Relaxed);
        println!("[chaos] {}", if on { "unleashed" } else { "sleeping" });
        interp.stack_mut().push(WofValue::integer(if on { 1 } else { 0 }));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // GLITCH MODE
    // ═══════════════════════════════════════════════════════════════
    
    // Toggle glitch mode
    // Stack: →
    interp.register(":glitchmode", |_interp| {
        let now = !GLITCH_MODE.load(Ordering::Relaxed);
        GLITCH_MODE.store(now, Ordering::Relaxed);
        
        println!();
        println!("⚠ Glitch mode {}.", if now { "ONLINE" } else { "OFFLINE" });
        println!("Random glyph substitutions {}.", if now { "may occur" } else { "cease" });
        println!();
        
        Ok(())
    });

    // Echo with glitched text
    // Stack: →
    interp.register(":glitch-echo", |_interp| {
        if !GLITCH_MODE.load(Ordering::Relaxed) {
            println!("(no glitches today)");
            return Ok(());
        }
        
        let mut base = "woflang glyph stream".to_string();
        let bytes: Vec<char> = base.chars().collect();
        base.clear();
        
        for (i, c) in bytes.iter().enumerate() {
            if *c != ' ' && (i % 4 == 0) {
                base.push(random_glyph_char());
            } else {
                base.push(*c);
            }
        }
        
        println!("{}", base);
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // DEITY MODE
    // ═══════════════════════════════════════════════════════════════
    
    // Toggle divine recursion mode
    // Stack: → 0|1
    interp.register(":deity", |interp| {
        let now = !DEITY_MODE.load(Ordering::Relaxed);
        DEITY_MODE.store(now, Ordering::Relaxed);
        
        println!();
        println!("👁  Deity mode {}.", if now { "ENABLED" } else { "DISABLED" });
        if now {
            println!("    Recursion guards are ignored where possible.");
            println!("    The call stack gazes back.");
        } else {
            println!("    Mortal limits restored.");
        }
        println!();
        
        interp.stack_mut().push(WofValue::integer(if now { 1 } else { 0 }));
        Ok(())
    });

    // Query deity mode state
    // Stack: → 0|1
    interp.register(":deity?", |interp| {
        let on = DEITY_MODE.load(Ordering::Relaxed);
        interp.stack_mut().push(WofValue::integer(if on { 1 } else { 0 }));
        Ok(())
    });
}
