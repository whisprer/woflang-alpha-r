//! Moses operations for Woflang.
//!
//! Part the stack-sea like Moses parted the Red Sea:
//! - `moses` - Non-destructive visualization of stack halves
//! - `moses_split` - Destructive split with separator marker

use woflang_core::WofValue;
use woflang_runtime::Interpreter;

/// Format a WofValue for display.
fn describe_value(v: &WofValue) -> String {
    match v {
        WofValue::Integer(n) => n.to_string(),
        WofValue::Float(f) => format!("{:.6}", f),
        WofValue::String(s) => format!("\"{}\"", s),
        WofValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
        WofValue::List(items) => format!("[{} items]", items.len()),
        WofValue::Nil => "nil".to_string(),
    }
}

/// Register moses operations.
pub fn register(interp: &mut Interpreter) {
    // Non-destructive "part the sea" visualization
    // Stack: ... → ... (unchanged)
    interp.register("moses", |interp| {
        let len = interp.stack().len();
        
        if len == 0 {
            println!("[moses] The sea is dry. The stack is empty.");
            return Ok(());
        }
        
        if len == 1 {
            println!("[moses] Only one value in the sea; nothing to part:");
            if let Ok(top) = interp.stack().peek() {
                println!("        top → {}", describe_value(top));
            }
            return Ok(());
        }
        
        let mid = len / 2;
        
        println!("🌊 [moses] Parting the stack-sea of {} values...", len);
        println!("    left ({} values, bottom side):", mid);
        
        // We need to peek at stack elements by index
        // The stack grows upward, so index 0 is bottom
        for i in 0..mid {
            if let Ok(val) = interp.stack().peek_at(i) {
                println!("      [{}] {}", i, describe_value(val));
            }
        }
        
        println!("    ───────────────  ⟡  ───────────────");
        
        println!("    right ({} values, including top):", len - mid);
        for i in mid..len {
            let is_top = i + 1 == len;
            if let Ok(val) = interp.stack().peek_at(i) {
                println!("      [{}] {}{}", i, describe_value(val), 
                    if is_top { "   ← top" } else { "" });
            }
        }
        
        Ok(())
    });

    // Destructive variant: insert separator marker
    // Stack: a b c d → a b "⟡-SEA-SPLIT-⟡" c d
    interp.register("moses_split", |interp| {
        let len = interp.stack().len();
        
        if len < 2 {
            println!("[moses_split] Need at least two values to part the sea.");
            return Ok(());
        }
        
        let mid = len / 2;
        
        // Collect all values
        let mut all_values = Vec::new();
        while !interp.stack().is_empty() {
            all_values.push(interp.stack_mut().pop()?);
        }
        all_values.reverse(); // Now in bottom-to-top order
        
        // Push left half
        for v in all_values[..mid].iter() {
            interp.stack_mut().push(v.clone());
        }
        
        // Push separator
        interp.stack_mut().push(WofValue::string("⟡-SEA-SPLIT-⟡".to_string()));
        
        // Push right half
        for v in all_values[mid..].iter() {
            interp.stack_mut().push(v.clone());
        }
        
        println!("🌊 [moses_split] The stack-sea has been parted.");
        println!("    Left side size:  {}", mid);
        println!("    Right side size: {}", len - mid);
        println!("    Marker value:    \"⟡-SEA-SPLIT-⟡\" (in the middle of the stack)");
        
        Ok(())
    });

    // Find the sea split marker
    // Stack: → position|-1
    interp.register("moses_find", |interp| {
        let len = interp.stack().len();
        let mut found_pos: i64 = -1;
        
        for i in 0..len {
            if let Ok(val) = interp.stack().peek_at(i) {
                if let WofValue::String(s) = val {
                    if s == "⟡-SEA-SPLIT-⟡" {
                        found_pos = i as i64;
                        break;
                    }
                }
            }
        }
        
        interp.stack_mut().push(WofValue::integer(found_pos));
        Ok(())
    });
}
