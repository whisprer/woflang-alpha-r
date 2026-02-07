//! Stack manipulation operations.
//!
//! These operations modify the stack without performing computation:
//!
//! | Operation    | Stack Effect     | Description |
//! |--------------|------------------|-------------|
//! | `dup`        | (a -- a a)       | Duplicate top |
//! | `drop`       | (a -- )          | Remove top |
//! | `swap`       | (a b -- b a)     | Swap top two |
//! | `over`       | (a b -- a b a)   | Copy second to top |
//! | `rot`        | (a b c -- b c a) | Rotate top three |
//! | `nip`        | (a b -- b)       | Remove second |
//! | `tuck`       | (a b -- b a b)   | Copy top below second |
//! | `depth`      | ( -- n)          | Push stack depth |
//! | `clear`      | (... -- )        | Clear entire stack |
//! | `stack_slayer` | (... -- )      | Dramatic clear 🗡️ |

use woflang_core::{InterpreterContext, Result, WofError, WofValue};
use woflang_runtime::Interpreter;

/// Register all stack manipulation operations.
pub fn register(interp: &mut Interpreter) {
    interp.register("dup", op_dup);
    interp.register("drop", op_drop);
    interp.register("swap", op_swap);
    interp.register("over", op_over);
    interp.register("rot", op_rot);
    interp.register("nip", op_nip);
    interp.register("tuck", op_tuck);
    interp.register("2dup", op_2dup);
    interp.register("2drop", op_2drop);
    interp.register("2swap", op_2swap);
    interp.register("depth", op_depth);
    interp.register("clear", op_clear);
    interp.register("pick", op_pick);

    // Dramatic operations 🐺
    interp.register("stack_slayer", op_stack_slayer);
    interp.register("resurrect", op_resurrect);
}

fn op_dup(interp: &mut Interpreter) -> Result<()> {
    interp.stack_mut().dup()
}

fn op_drop(interp: &mut Interpreter) -> Result<()> {
    interp.stack_mut().drop()
}

fn op_swap(interp: &mut Interpreter) -> Result<()> {
    interp.stack_mut().swap()
}

fn op_over(interp: &mut Interpreter) -> Result<()> {
    interp.stack_mut().over()
}

fn op_rot(interp: &mut Interpreter) -> Result<()> {
    interp.stack_mut().rot()
}

fn op_nip(interp: &mut Interpreter) -> Result<()> {
    // (a b -- b)
    let stack = interp.stack_mut();
    if !stack.has(2) {
        return Err(WofError::stack_underflow(2, stack.len()));
    }
    stack.swap()?;
    stack.drop()
}

fn op_tuck(interp: &mut Interpreter) -> Result<()> {
    // (a b -- b a b)
    let stack = interp.stack_mut();
    if !stack.has(2) {
        return Err(WofError::stack_underflow(2, stack.len()));
    }
    stack.swap()?;
    stack.over()
}

fn op_2dup(interp: &mut Interpreter) -> Result<()> {
    // (a b -- a b a b)
    let stack = interp.stack_mut();
    if !stack.has(2) {
        return Err(WofError::stack_underflow(2, stack.len()));
    }
    stack.over()?;
    stack.over()
}

fn op_2drop(interp: &mut Interpreter) -> Result<()> {
    // (a b -- )
    let stack = interp.stack_mut();
    if !stack.has(2) {
        return Err(WofError::stack_underflow(2, stack.len()));
    }
    stack.drop()?;
    stack.drop()
}

fn op_2swap(interp: &mut Interpreter) -> Result<()> {
    // (a b c d -- c d a b)
    let stack = interp.stack_mut();
    if !stack.has(4) {
        return Err(WofError::stack_underflow(4, stack.len()));
    }

    // Manual swap of pairs
    let slice = stack.as_mut_slice();
    let len = slice.len();
    slice.swap(len - 4, len - 2);
    slice.swap(len - 3, len - 1);
    Ok(())
}

fn op_depth(interp: &mut Interpreter) -> Result<()> {
    let depth = interp.stack().len();
    interp.push(WofValue::integer(depth as i64));
    Ok(())
}

fn op_clear(interp: &mut Interpreter) -> Result<()> {
    interp.clear();
    Ok(())
}

fn op_pick(interp: &mut Interpreter) -> Result<()> {
    // (... n -- ... v) where v is the n-th element from top (0 = top)
    let n = interp.stack_mut().pop_integer()? as usize;
    let val = interp.stack().peek_at(n)?.clone();
    interp.push(val);
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════
// DRAMATIC OPERATIONS 🐺
// ═══════════════════════════════════════════════════════════════════════

fn op_stack_slayer(interp: &mut Interpreter) -> Result<()> {
    interp.clear();
    Ok(())
}

fn op_resurrect(interp: &mut Interpreter) -> Result<()> {
    // Push mystical constants onto the stack
    interp.push(WofValue::double(std::f64::consts::PI));
    interp.push(WofValue::double(std::f64::consts::E));
    interp.push(WofValue::double(1.618_033_988_749_895)); // Golden ratio (φ)
    interp.push(WofValue::integer(42)); // The Answer
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_interp() -> Interpreter {
        let mut interp = Interpreter::new();
        register(&mut interp);
        interp
    }

    #[test]
    fn test_dup() {
        let mut interp = make_interp();
        interp.exec_line("42 dup").unwrap();
        assert_eq!(interp.stack().len(), 2);
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 42);
    }

    #[test]
    fn test_drop() {
        let mut interp = make_interp();
        interp.exec_line("1 2 drop").unwrap();
        assert_eq!(interp.stack().len(), 1);
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 1);
    }

    #[test]
    fn test_swap() {
        let mut interp = make_interp();
        interp.exec_line("1 2 swap").unwrap();
        assert_eq!(interp.stack_mut().pop_integer().unwrap(), 1);
        assert_eq!(interp.stack_mut().pop_integer().unwrap(), 2);
    }

    #[test]
    fn test_over() {
        let mut interp = make_interp();
        interp.exec_line("1 2 over").unwrap();
        assert_eq!(interp.stack().len(), 3);
        assert_eq!(interp.stack_mut().pop_integer().unwrap(), 1);
    }

    #[test]
    fn test_rot() {
        let mut interp = make_interp();
        interp.exec_line("1 2 3 rot").unwrap();
        assert_eq!(interp.stack_mut().pop_integer().unwrap(), 1);
        assert_eq!(interp.stack_mut().pop_integer().unwrap(), 3);
        assert_eq!(interp.stack_mut().pop_integer().unwrap(), 2);
    }

    #[test]
    fn test_depth() {
        let mut interp = make_interp();
        interp.exec_line("1 2 3 depth").unwrap();
        assert_eq!(interp.stack_mut().pop_integer().unwrap(), 3);
    }

    #[test]
    fn test_clear() {
        let mut interp = make_interp();
        interp.exec_line("1 2 3 clear").unwrap();
        assert!(interp.stack().is_empty());
    }

    #[test]
    fn test_2dup() {
        let mut interp = make_interp();
        interp.exec_line("1 2 2dup").unwrap();
        assert_eq!(interp.stack().len(), 4);
    }

    #[test]
    fn test_stack_slayer() {
        let mut interp = make_interp();
        interp.exec_line("1 2 3 4 5 stack_slayer").unwrap();
        assert!(interp.stack().is_empty());
    }

    #[test]
    fn test_resurrect() {
        let mut interp = make_interp();
        interp.exec_line("resurrect").unwrap();
        assert_eq!(interp.stack().len(), 4);
        assert_eq!(interp.stack_mut().pop_integer().unwrap(), 42);
    }
}
