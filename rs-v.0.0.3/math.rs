//! Extended mathematical functions.
//!
//! Provides trigonometric, exponential, and other mathematical operations:
//!
//! | Operation  | Stack Effect | Description |
//! |------------|--------------|-------------|
//! | `sin`      | (a -- b)     | Sine |
//! | `cos`      | (a -- b)     | Cosine |
//! | `tan`      | (a -- b)     | Tangent |
//! | `asin`     | (a -- b)     | Arc sine |
//! | `acos`     | (a -- b)     | Arc cosine |
//! | `atan`     | (a -- b)     | Arc tangent |
//! | `atan2`    | (y x -- a)   | Two-argument arc tangent |
//! | `sqrt`     | (a -- b)     | Square root |
//! | `cbrt`     | (a -- b)     | Cube root |
//! | `pow`      | (a b -- c)   | Power |
//! | `exp`      | (a -- b)     | e^a |
//! | `ln`       | (a -- b)     | Natural log |
//! | `log10`    | (a -- b)     | Base-10 log |
//! | `log2`     | (a -- b)     | Base-2 log |
//! | `floor`    | (a -- b)     | Floor |
//! | `ceil`     | (a -- b)     | Ceiling |
//! | `round`    | (a -- b)     | Round |
//! | `trunc`    | (a -- b)     | Truncate |
//! | `hypot`    | (a b -- c)   | Hypotenuse |

use woflang_core::{InterpreterContext, Result, WofError, WofValue};
use woflang_runtime::Interpreter;

/// Register all math operations.
pub fn register(interp: &mut Interpreter) {
    // Trigonometric functions
    interp.register("sin", op_sin);
    interp.register("cos", op_cos);
    interp.register("tan", op_tan);
    interp.register("asin", op_asin);
    interp.register("acos", op_acos);
    interp.register("atan", op_atan);
    interp.register("atan2", op_atan2);
    interp.register("sinh", op_sinh);
    interp.register("cosh", op_cosh);
    interp.register("tanh", op_tanh);

    // Powers and roots
    interp.register("sqrt", op_sqrt);
    interp.register("√", op_sqrt);
    interp.register("cbrt", op_cbrt);
    interp.register("pow", op_pow);
    interp.register("^", op_pow);

    // Exponential and logarithmic
    interp.register("exp", op_exp);
    interp.register("ln", op_ln);
    interp.register("log", op_ln);
    interp.register("log10", op_log10);
    interp.register("log2", op_log2);

    // Rounding
    interp.register("floor", op_floor);
    interp.register("ceil", op_ceil);
    interp.register("round", op_round);
    interp.register("trunc", op_trunc);
    interp.register("frac", op_frac);

    // Other
    interp.register("hypot", op_hypot);
    interp.register("sign", op_sign);
    interp.register("factorial", op_factorial);
    interp.register("gcd", op_gcd);
    interp.register("lcm", op_lcm);

    // Temperature conversions
    interp.register("celsius_to_kelvin", op_celsius_to_kelvin);
    interp.register("kelvin_to_celsius", op_kelvin_to_celsius);
    interp.register("celsius_to_fahrenheit", op_celsius_to_fahrenheit);
    interp.register("fahrenheit_to_celsius", op_fahrenheit_to_celsius);

    // Degree/radian conversion
    interp.register("deg2rad", op_deg_to_rad);
    interp.register("rad2deg", op_rad_to_deg);
}

// ═══════════════════════════════════════════════════════════════════════
// TRIGONOMETRIC FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════

fn op_sin(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.sin()));
    Ok(())
}

fn op_cos(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.cos()));
    Ok(())
}

fn op_tan(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.tan()));
    Ok(())
}

fn op_asin(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    if !(-1.0..=1.0).contains(&a) {
        return Err(WofError::InvalidArgument(format!(
            "asin: argument {a} out of range [-1, 1]"
        )));
    }
    interp.push(WofValue::double(a.asin()));
    Ok(())
}

fn op_acos(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    if !(-1.0..=1.0).contains(&a) {
        return Err(WofError::InvalidArgument(format!(
            "acos: argument {a} out of range [-1, 1]"
        )));
    }
    interp.push(WofValue::double(a.acos()));
    Ok(())
}

fn op_atan(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.atan()));
    Ok(())
}

fn op_atan2(interp: &mut Interpreter) -> Result<()> {
    let x = interp.stack_mut().pop_numeric()?;
    let y = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(y.atan2(x)));
    Ok(())
}

fn op_sinh(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.sinh()));
    Ok(())
}

fn op_cosh(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.cosh()));
    Ok(())
}

fn op_tanh(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.tanh()));
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════
// POWERS AND ROOTS
// ═══════════════════════════════════════════════════════════════════════

fn op_sqrt(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    if a < 0.0 {
        return Err(WofError::InvalidArgument(format!(
            "sqrt: negative argument {a}"
        )));
    }
    interp.push(WofValue::double(a.sqrt()));
    Ok(())
}

fn op_cbrt(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.cbrt()));
    Ok(())
}

fn op_pow(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_numeric()?;
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.powf(b)));
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════
// EXPONENTIAL AND LOGARITHMIC
// ═══════════════════════════════════════════════════════════════════════

fn op_exp(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.exp()));
    Ok(())
}

fn op_ln(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    if a <= 0.0 {
        return Err(WofError::InvalidArgument(format!(
            "ln: non-positive argument {a}"
        )));
    }
    interp.push(WofValue::double(a.ln()));
    Ok(())
}

fn op_log10(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    if a <= 0.0 {
        return Err(WofError::InvalidArgument(format!(
            "log10: non-positive argument {a}"
        )));
    }
    interp.push(WofValue::double(a.log10()));
    Ok(())
}

fn op_log2(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    if a <= 0.0 {
        return Err(WofError::InvalidArgument(format!(
            "log2: non-positive argument {a}"
        )));
    }
    interp.push(WofValue::double(a.log2()));
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════
// ROUNDING FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════

fn op_floor(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.floor()));
    Ok(())
}

fn op_ceil(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.ceil()));
    Ok(())
}

fn op_round(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.round()));
    Ok(())
}

fn op_trunc(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.trunc()));
    Ok(())
}

fn op_frac(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.fract()));
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════
// OTHER MATHEMATICAL FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════

fn op_hypot(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_numeric()?;
    let a = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(a.hypot(b)));
    Ok(())
}

fn op_sign(interp: &mut Interpreter) -> Result<()> {
    let a = interp.stack_mut().pop_numeric()?;
    let s = if a > 0.0 {
        1
    } else if a < 0.0 {
        -1
    } else {
        0
    };
    interp.push(WofValue::integer(s));
    Ok(())
}

fn op_factorial(interp: &mut Interpreter) -> Result<()> {
    let n = interp.stack_mut().pop_integer()?;
    if n < 0 {
        return Err(WofError::InvalidArgument(format!(
            "factorial: negative argument {n}"
        )));
    }
    if n > 20 {
        // Prevent overflow for i64
        return Err(WofError::Overflow(format!(
            "factorial: argument {n} too large"
        )));
    }

    let result = (1..=n).fold(1i64, |acc, x| acc.saturating_mul(x));
    interp.push(WofValue::integer(result));
    Ok(())
}

fn op_gcd(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_integer()?.unsigned_abs();
    let a = interp.stack_mut().pop_integer()?.unsigned_abs();
    interp.push(WofValue::integer(num_integer::gcd(a, b) as i64));
    Ok(())
}

fn op_lcm(interp: &mut Interpreter) -> Result<()> {
    let b = interp.stack_mut().pop_integer()?.unsigned_abs();
    let a = interp.stack_mut().pop_integer()?.unsigned_abs();
    interp.push(WofValue::integer(num_integer::lcm(a, b) as i64));
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════
// TEMPERATURE CONVERSIONS
// ═══════════════════════════════════════════════════════════════════════

fn op_celsius_to_kelvin(interp: &mut Interpreter) -> Result<()> {
    let c = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(c + 273.15));
    Ok(())
}

fn op_kelvin_to_celsius(interp: &mut Interpreter) -> Result<()> {
    let k = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(k - 273.15));
    Ok(())
}

fn op_celsius_to_fahrenheit(interp: &mut Interpreter) -> Result<()> {
    let c = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(c * 9.0 / 5.0 + 32.0));
    Ok(())
}

fn op_fahrenheit_to_celsius(interp: &mut Interpreter) -> Result<()> {
    let f = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double((f - 32.0) * 5.0 / 9.0));
    Ok(())
}

fn op_deg_to_rad(interp: &mut Interpreter) -> Result<()> {
    let deg = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(deg.to_radians()));
    Ok(())
}

fn op_rad_to_deg(interp: &mut Interpreter) -> Result<()> {
    let rad = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(rad.to_degrees()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants;

    fn make_interp() -> Interpreter {
        let mut interp = Interpreter::new();
        register(&mut interp);
        constants::register(&mut interp);
        interp
    }

    #[test]
    fn test_sqrt() {
        let mut interp = make_interp();
        interp.exec_line("16 sqrt").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_pow() {
        let mut interp = make_interp();
        interp.exec_line("2 8 pow").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - 256.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_sin_cos() {
        let mut interp = make_interp();
        interp.exec_line("0 cos").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_sin_pi_over_2() {
        let mut interp = make_interp();
        interp.exec_line("pi 2 / sin").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_ln_e() {
        let mut interp = make_interp();
        interp.exec_line("e ln").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_floor_ceil() {
        let mut interp = make_interp();
        interp.exec_line("3.7 floor").unwrap();
        assert!((interp.stack_mut().pop_numeric().unwrap() - 3.0).abs() < f64::EPSILON);

        interp.exec_line("3.2 ceil").unwrap();
        assert!((interp.stack_mut().pop_numeric().unwrap() - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_factorial() {
        let mut interp = make_interp();
        interp.exec_line("5 factorial").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 120);
    }

    #[test]
    fn test_gcd() {
        let mut interp = make_interp();
        interp.exec_line("48 18 gcd").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 6);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        let mut interp = make_interp();
        interp.exec_line("25 celsius_to_kelvin").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - 298.15).abs() < f64::EPSILON);
    }

    #[test]
    fn test_deg_to_rad() {
        let mut interp = make_interp();
        interp.exec_line("180 deg2rad").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_negative_error() {
        let mut interp = make_interp();
        let result = interp.exec_line("-4 sqrt");
        assert!(matches!(result, Err(WofError::InvalidArgument(_))));
    }
}