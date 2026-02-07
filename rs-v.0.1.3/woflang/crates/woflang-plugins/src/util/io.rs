//! I/O and debug operations for Woflang.
//!
//! Provides print, input, debug output, and stack visualization.

use woflang_core::{WofValue, InterpreterContext};
use woflang_runtime::Interpreter;
use std::io::{self, Write};

/// Register I/O and debug operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // OUTPUT
    // ═══════════════════════════════════════════════════════════════
    
    // Print top of stack with newline
    interp.register("print", |interp| {
        let val = interp.stack_mut().pop()?;
        println!("{}", val);
        Ok(())
    });

    interp.register("say", |interp| {
        let val = interp.stack_mut().pop()?;
        println!("{}", val);
        Ok(())
    });

    // Print without newline
    interp.register("emit", |interp| {
        let val = interp.stack_mut().pop()?;
        print!("{}", val);
        let _ = io::stdout().flush();
        Ok(())
    });

    // Print and keep on stack
    interp.register("peek_print", |interp| {
        let val = interp.stack().peek()?;
        println!("{}", val);
        Ok(())
    });

    // Print newline
    interp.register("cr", |_interp| {
        println!();
        Ok(())
    });

    interp.register("newline", |_interp| {
        println!();
        Ok(())
    });

    // Print space
    interp.register("space", |_interp| {
        print!(" ");
        let _ = io::stdout().flush();
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // DEBUG OUTPUT
    // ═══════════════════════════════════════════════════════════════
    
    // Show entire stack
    interp.register(".s", |interp| {
        println!("{}", interp.stack());
        Ok(())
    });

    interp.register("show_stack", |interp| {
        println!("{}", interp.stack());
        Ok(())
    });

    // Debug print with type info
    interp.register("debug", |interp| {
        let val = interp.stack_mut().pop()?;
        println!("[DEBUG] {:?}", val);
        Ok(())
    });

    // Debug peek (don't consume)
    interp.register("debug_peek", |interp| {
        let val = interp.stack().peek()?;
        println!("[DEBUG] {:?}", val);
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // INPUT
    // ═══════════════════════════════════════════════════════════════
    
    // Read a line from stdin
    interp.register("input", |interp| {
        let mut line = String::new();
        io::stdin().read_line(&mut line).map_err(|e| {
            woflang_core::WofError::Runtime(format!("input error: {}", e))
        })?;
        let line = line.trim_end_matches('\n').trim_end_matches('\r');
        interp.stack_mut().push(WofValue::string(line));
        Ok(())
    });

    // Read line with prompt
    interp.register("prompt", |interp| {
        let prompt = interp.stack_mut().pop()?;
        print!("{}", prompt);
        let _ = io::stdout().flush();
        
        let mut line = String::new();
        io::stdin().read_line(&mut line).map_err(|e| {
            woflang_core::WofError::Runtime(format!("input error: {}", e))
        })?;
        let line = line.trim_end_matches('\n').trim_end_matches('\r');
        interp.stack_mut().push(WofValue::string(line));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // CONVERSION
    // ═══════════════════════════════════════════════════════════════
    
    // Convert to string
    interp.register("to_str", |interp| {
        let val = interp.stack_mut().pop()?;
        interp.stack_mut().push(WofValue::string(format!("{}", val)));
        Ok(())
    });

    // Parse string to number
    interp.register("to_num", |interp| {
        let val = interp.stack_mut().pop()?;
        let s = val.as_string().map_err(|_| {
            woflang_core::WofError::Runtime("to_num: expected string".into())
        })?;
        
        // Try integer first, then float
        if let Ok(i) = s.parse::<i64>() {
            interp.stack_mut().push(WofValue::integer(i));
        } else if let Ok(f) = s.parse::<f64>() {
            interp.stack_mut().push(WofValue::double(f));
        } else {
            return Err(woflang_core::WofError::Runtime(
                format!("to_num: cannot parse '{}'", s)
            ));
        }
        Ok(())
    });

    // Convert to integer (truncate)
    interp.register("to_int", |interp| {
        let val = interp.stack_mut().pop()?;
        let i = if val.is_integer() {
            val.as_integer()?
        } else {
            val.as_double()? as i64
        };
        interp.stack_mut().push(WofValue::integer(i));
        Ok(())
    });

    // Convert to float
    interp.register("to_float", |interp| {
        let val = interp.stack_mut().pop()?;
        let f = val.as_double()?;
        interp.stack_mut().push(WofValue::double(f));
        Ok(())
    });
}
