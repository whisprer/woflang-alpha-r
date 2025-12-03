//! Boolean and propositional logic operations.
//!
//! | Operation  | Stack Effect | Description |
//! |------------|--------------|-------------|
//! | `and`      | (a b -- c)   | Logical AND |
//! | `or`       | (a b -- c)   | Logical OR |
//! | `xor`      | (a b -- c)   | Logical XOR |
//! | `not`      | (a -- b)     | Logical NOT |
//! | `implies`  | (a b -- c)   | Material implication (a → b) |
//! | `iff`      | (a b -- c)   | Biconditional (a ↔ b) |
//! | `nand`     | (a b -- c)   | NOT AND |
//! | `nor`      | (a b -- c)   | NOT OR |

use woflang_core::{InterpreterContext, Result, WofValue};
use woflang_runtime::Interpreter;

/// Register all logic operations.
pub fn register(interp: &mut Interpreter) {
    // Basic boolean
    interp.register("and", op_and);
    interp.register("or", op_or);
    interp.register("xor", op_xor);
    interp.register("not", op_not);

    // Unicode aliases
    interp.register("∧", op_and);
    interp.register("∨", op_or);
    interp.register("⊕", op_xor);
    interp.register("¬", op_not);

    // Extended logic
    interp.register("nand", op_nand);
    interp.register("nor", op_nor);
    interp.register("implies", op_implies);
    interp.register("→", op_implies);
    interp.register("iff", op_iff);
    interp.register("↔", op_iff);

    // Comparison
    interp.register("=", op_eq);
    interp.register("==", op_eq);
    interp.register("!=", op_ne);
    interp.register("≠", op_ne);
    interp.register("<", op_lt);
    interp.register(">", op_gt);
    interp.register("<=", op_le);
    interp.register("≤", op_le);
    interp.register(">=", op_ge);
    interp.register("≥", op_ge);

    // Boolean constants
    interp.register("true", |i| {
        i.push(WofValue::boolean(true));
        Ok(())
    });
    interp.register("false", |i| {
        i.push(WofValue::boolean(false));
        Ok(())
    });

    // Tautology demonstration
    interp.register("tautology", op_tautology);
}

fn op_and(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_bool()?;
    let a = interp.stack_mut().pop_bool()?;
    interp.push(WofValue::boolean(a && b));
    Ok(())
}

fn op_or(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_bool()?;
    let a = interp.stack_mut().pop_bool()?;
    interp.push(WofValue::boolean(a || b));
    Ok(())
}

fn op_xor(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_bool()?;
    let a = interp.stack_mut().pop_bool()?;
    interp.push(WofValue::boolean(a ^ b));
    Ok(())
}

fn op_not(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_bool()?;
    interp.push(WofValue::boolean(!a));
    Ok(())
}

fn op_nand(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_bool()?;
    let a = interp.stack_mut().pop_bool()?;
    interp.push(WofValue::boolean(!(a && b)));
    Ok(())
}

fn op_nor(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_bool()?;
    let a = interp.stack_mut().pop_bool()?;
    interp.push(WofValue::boolean(!(a || b)));
    Ok(())
}

fn op_implies(interp: &mut Interpreter) -> Result<()> {
    // a → b ≡ ¬a ∨ b
    let b = interp.stack_mut().pop_bool()?;
    let a = interp.stack_mut().pop_bool()?;
    interp.push(WofValue::boolean(!a || b));
    Ok(())
}

fn op_iff(interp: &mut Interpreter) -> Result<()> {
    // a ↔ b ≡ (a → b) ∧ (b → a) ≡ a == b
    let b = interp.stack_mut().pop_bool()?;
    let a = interp.stack_mut().pop_bool()?;
    interp.push(WofValue::boolean(a == b));
    Ok(())
}

fn op_eq(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop()?;
    let a = interp.stack_mut().pop()?;
    interp.push(WofValue::boolean(a == b));
    Ok(())
}

fn op_ne(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop()?;
    let a = interp.stack_mut().pop()?;
    interp.push(WofValue::boolean(a != b));
    Ok(())
}

fn op_lt(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_numeric()?;
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::boolean(a < b));
    Ok(())
}

fn op_gt(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_numeric()?;
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::boolean(a > b));
    Ok(())
}

fn op_le(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_numeric()?;
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::boolean(a <= b));
    Ok(())
}

fn op_ge(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_numeric()?;
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::boolean(a >= b));
    Ok(())
}

fn op_tautology(interp: &mut Interpreter) -> Result<()> {
    // Demonstrate: (P → Q) ↔ (¬P ∨ Q)
    // Push 1 (true) as proof
    interp.push(WofValue::boolean(true));
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
    fn test_and() {
        let mut interp = make_interp();
        interp.exec_line("1 1 and").unwrap();
        assert!(interp.stack().peek().unwrap().as_bool());

        interp.exec_line("1 0 and").unwrap();
        assert!(!interp.stack().peek().unwrap().as_bool());
    }

    #[test]
    fn test_or() {
        let mut interp = make_interp();
        interp.exec_line("0 1 or").unwrap();
        assert!(interp.stack().peek().unwrap().as_bool());

        interp.exec_line("0 0 or").unwrap();
        assert!(!interp.stack().peek().unwrap().as_bool());
    }

    #[test]
    fn test_not() {
        let mut interp = make_interp();
        interp.exec_line("0 not").unwrap();
        assert!(interp.stack().peek().unwrap().as_bool());

        interp.exec_line("1 not").unwrap();
        assert!(!interp.stack().peek().unwrap().as_bool());
    }

    #[test]
    fn test_implies() {
        let mut interp = make_interp();
        // True implies False = False
        interp.exec_line("1 0 implies").unwrap();
        assert!(!interp.stack_mut().pop_bool().unwrap());

        // True implies True = True
        interp.exec_line("1 1 implies").unwrap();
        assert!(interp.stack_mut().pop_bool().unwrap());

        // False implies True = True
        interp.exec_line("0 1 implies").unwrap();
        assert!(interp.stack_mut().pop_bool().unwrap());
    }

    #[test]
    fn test_comparisons() {
        let mut interp = make_interp();

        interp.exec_line("5 3 >").unwrap();
        assert!(interp.stack_mut().pop_bool().unwrap());

        interp.exec_line("3 5 <").unwrap();
        assert!(interp.stack_mut().pop_bool().unwrap());

        interp.exec_line("5 5 ==").unwrap();
        assert!(interp.stack_mut().pop_bool().unwrap());
    }

    #[test]
    fn test_unicode_logic() {
        let mut interp = make_interp();
        interp.exec_line("1 1 ∧").unwrap();
        assert!(interp.stack().peek().unwrap().as_bool());
    }
}
