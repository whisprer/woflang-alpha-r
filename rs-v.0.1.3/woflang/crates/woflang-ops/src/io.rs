//! Input/Output and display operations.
//!
//! | Operation  | Stack Effect | Description |
//! |------------|--------------|-------------|
//! | `print`    | (a -- )      | Print top value |
//! | `.`        | ( -- )       | Display stack |
//! | `.s`       | ( -- )       | Display stack (alias) |
//! | `show`     | (a -- a)     | Print without consuming |
//! | `cr`       | ( -- )       | Print newline |
//! | `emit`     | (n -- )      | Print char by codepoint |

use woflang_core::{InterpreterContext, Result, WofValue};
use woflang_runtime::Interpreter;

/// Register all I/O operations.
pub fn register(interp: &mut Interpreter) {
    interp.register("print", op_print);
    interp.register(".", op_show_stack);
    interp.register(".s", op_show_stack);
    interp.register("show", op_show);
    interp.register("cr", op_cr);
    interp.register("emit", op_emit);
    interp.register("space", op_space);
    interp.register("spaces", op_spaces);
    interp.register("type", op_type);
    interp.register("typeof", op_typeof);
}

fn op_print(interp: &mut Interpreter) -> Result<()> {
    let val = interp.stack_mut().pop()?;
    println!("{val}");
    Ok(())
}

fn op_show_stack(interp: &mut Interpreter) -> Result<()> {
    println!("{}", interp.stack());
    Ok(())
}

fn op_show(interp: &mut Interpreter) -> Result<()> {
    let val = interp.stack().peek()?;
    println!("{val}");
    Ok(())
}

fn op_cr(_interp: &mut Interpreter) -> Result<()> {
    println!();
    Ok(())
}

fn op_emit(interp: &mut Interpreter) -> Result<()> {
    let code = interp.stack_mut().pop_integer()? as u32;
    if let Some(c) = char::from_u32(code) {
        print!("{c}");
    }
    Ok(())
}

fn op_space(_interp: &mut Interpreter) -> Result<()> {
    print!(" ");
    Ok(())
}

fn op_spaces(interp: &mut Interpreter) -> Result<()> {
    let n = interp.stack_mut().pop_integer()?;
    for _ in 0..n {
        print!(" ");
    }
    Ok(())
}

fn op_type(interp: &mut Interpreter) -> Result<()> {
    let s = interp.stack_mut().pop_string()?;
    print!("{s}");
    Ok(())
}

fn op_typeof(interp: &mut Interpreter) -> Result<()> {
    let val = interp.stack().peek()?;
    let type_name = format!("{}", val.value_type());
    interp.push(WofValue::string(type_name));
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
    fn test_typeof() {
        let mut interp = make_interp();
        interp.exec_line("42 typeof").unwrap();
        assert_eq!(interp.stack_mut().pop_string().unwrap(), "integer");

        interp.exec_line("3.14 typeof").unwrap();
        assert_eq!(interp.stack_mut().pop_string().unwrap(), "double");
    }
}
