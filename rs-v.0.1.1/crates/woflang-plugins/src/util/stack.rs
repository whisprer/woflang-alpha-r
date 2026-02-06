//! Stack manipulation utilities for Woflang.
//!
//! Additional stack operations beyond the basic dup/swap/drop.

use woflang_core::{WofError, WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

/// Register stack utility operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // STACK INFO
    // ═══════════════════════════════════════════════════════════════
    
    // Push stack depth
    interp.register("depth", |interp| {
        let depth = interp.stack().len();
        interp.stack_mut().push(WofValue::integer(depth as i64));
        Ok(())
    });

    interp.register("stack_depth", |interp| {
        let depth = interp.stack().len();
        interp.stack_mut().push(WofValue::integer(depth as i64));
        Ok(())
    });

    // Check if stack is empty (push 1 if empty, 0 otherwise)
    interp.register("empty?", |interp| {
        let is_empty = interp.stack().is_empty();
        interp.stack_mut().push(WofValue::integer(if is_empty { 1 } else { 0 }));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // EXTENDED STACK MANIPULATION
    // ═══════════════════════════════════════════════════════════════
    
    // nip: remove second item (a b → b)
    interp.register("nip", |interp| {
        let b = interp.stack_mut().pop()?;
        let _a = interp.stack_mut().pop()?;
        interp.stack_mut().push(b);
        Ok(())
    });

    // tuck: copy top under second (a b → b a b)
    interp.register("tuck", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        interp.stack_mut().push(b.clone());
        interp.stack_mut().push(a);
        interp.stack_mut().push(b);
        Ok(())
    });

    // 2dup: duplicate top two (a b → a b a b)
    interp.register("2dup", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        interp.stack_mut().push(a.clone());
        interp.stack_mut().push(b.clone());
        interp.stack_mut().push(a);
        interp.stack_mut().push(b);
        Ok(())
    });

    // 2drop: drop top two (a b → )
    interp.register("2drop", |interp| {
        interp.stack_mut().pop()?;
        interp.stack_mut().pop()?;
        Ok(())
    });

    // 2swap: swap top two pairs (a b c d → c d a b)
    interp.register("2swap", |interp| {
        let d = interp.stack_mut().pop()?;
        let c = interp.stack_mut().pop()?;
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        interp.stack_mut().push(c);
        interp.stack_mut().push(d);
        interp.stack_mut().push(a);
        interp.stack_mut().push(b);
        Ok(())
    });

    // 2over: copy second pair over top (a b c d → a b c d a b)
    interp.register("2over", |interp| {
        let d = interp.stack_mut().pop()?;
        let c = interp.stack_mut().pop()?;
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        interp.stack_mut().push(a.clone());
        interp.stack_mut().push(b.clone());
        interp.stack_mut().push(c);
        interp.stack_mut().push(d);
        interp.stack_mut().push(a);
        interp.stack_mut().push(b);
        Ok(())
    });

    // pick: copy nth item to top (... n → ... item)
    interp.register("pick", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()? as usize;
        let len = interp.stack().len();
        if n >= len {
            return Err(WofError::Runtime(format!("pick: index {} out of range", n)));
        }
        let item = interp.stack().peek_at(n)?.clone();
        interp.stack_mut().push(item);
        Ok(())
    });

    // roll: rotate n items (move nth to top)
    interp.register("roll", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()? as usize;
        let len = interp.stack().len();
        if n >= len || n == 0 {
            return Ok(()); // Nothing to do
        }
        
        // Collect items
        let mut items = Vec::with_capacity(n + 1);
        for _ in 0..=n {
            items.push(interp.stack_mut().pop()?);
        }
        
        // Put back with rotation: move bottom item to top
        let bottom = items.pop().unwrap();
        for item in items.into_iter().rev() {
            interp.stack_mut().push(item);
        }
        interp.stack_mut().push(bottom);
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // CLEAR AND REVERSE
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("clear", |interp| {
        interp.stack_mut().clear();
        Ok(())
    });

    interp.register("stack_clear", |interp| {
        interp.stack_mut().clear();
        Ok(())
    });

    // Reverse entire stack
    interp.register("reverse", |interp| {
        let mut items = Vec::new();
        while let Ok(item) = interp.stack_mut().pop() {
            items.push(item);
        }
        for item in items {
            interp.stack_mut().push(item);
        }
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // TYPE CHECKING
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("type?", |interp| {
        let val = interp.stack_mut().pop()?;
        let type_name = match &val {
            v if v.is_integer() => "integer",
            v if v.is_double() => "double",
            v if v.is_string() => "string",
            v if v.is_symbol() => "symbol",
            v if v.is_nil() => "nil",
            _ => "unknown",
        };
        interp.stack_mut().push(WofValue::string(type_name));
        Ok(())
    });

    interp.register("is_num?", |interp| {
        let val = interp.stack().peek()?;
        let result = if val.is_integer() || val.is_double() { 1 } else { 0 };
        interp.stack_mut().push(WofValue::integer(result));
        Ok(())
    });

    interp.register("is_str?", |interp| {
        let val = interp.stack().peek()?;
        let result = if val.is_string() { 1 } else { 0 };
        interp.stack_mut().push(WofValue::integer(result));
        Ok(())
    });
}
