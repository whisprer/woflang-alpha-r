//! Mathematical and physical constants.
//!
//! Provides named constants that push values onto the stack:
//!
//! | Operation   | Value | Description |
//! |-------------|-------|-------------|
//! | `pi`        | π     | Circle ratio |
//! | `e`         | e     | Euler's number |
//! | `phi`       | φ     | Golden ratio |
//! | `tau`       | τ     | 2π |
//! | `sqrt2`     | √2    | Square root of 2 |
//! | `avogadro`  | Nₐ    | Avogadro's constant |

use woflang_core::{InterpreterContext, WofValue};
use woflang_runtime::Interpreter;

/// Register all constant operations.
pub fn register(interp: &mut Interpreter) {
    // Mathematical constants
    interp.register("pi", |i| {
        i.push(WofValue::double(std::f64::consts::PI));
        Ok(())
    });

    interp.register("π", |i| {
        i.push(WofValue::double(std::f64::consts::PI));
        Ok(())
    });

    interp.register("e", |i| {
        i.push(WofValue::double(std::f64::consts::E));
        Ok(())
    });

    interp.register("tau", |i| {
        i.push(WofValue::double(std::f64::consts::TAU));
        Ok(())
    });

    interp.register("τ", |i| {
        i.push(WofValue::double(std::f64::consts::TAU));
        Ok(())
    });

    interp.register("phi", |i| {
        // Golden ratio: (1 + √5) / 2
        i.push(WofValue::double(1.618_033_988_749_895));
        Ok(())
    });

    interp.register("φ", |i| {
        i.push(WofValue::double(1.618_033_988_749_895));
        Ok(())
    });

    interp.register("sqrt2", |i| {
        i.push(WofValue::double(std::f64::consts::SQRT_2));
        Ok(())
    });

    interp.register("√2", |i| {
        i.push(WofValue::double(std::f64::consts::SQRT_2));
        Ok(())
    });

    interp.register("ln2", |i| {
        i.push(WofValue::double(std::f64::consts::LN_2));
        Ok(())
    });

    interp.register("ln10", |i| {
        i.push(WofValue::double(std::f64::consts::LN_10));
        Ok(())
    });

    // Physical constants
    interp.register("avogadro", |i| {
        // 6.02214076 × 10²³ mol⁻¹
        i.push(WofValue::double(6.022_140_76e23));
        Ok(())
    });

    interp.register("c", |i| {
        // Speed of light: 299,792,458 m/s
        i.push(WofValue::integer(299_792_458));
        Ok(())
    });

    interp.register("planck", |i| {
        // Planck's constant: 6.62607015 × 10⁻³⁴ J⋅s
        i.push(WofValue::double(6.626_070_15e-34));
        Ok(())
    });

    interp.register("boltzmann", |i| {
        // Boltzmann constant: 1.380649 × 10⁻²³ J/K
        i.push(WofValue::double(1.380_649e-23));
        Ok(())
    });

    // Infinity and special values
    interp.register("inf", |i| {
        i.push(WofValue::double(f64::INFINITY));
        Ok(())
    });

    interp.register("∞", |i| {
        i.push(WofValue::double(f64::INFINITY));
        Ok(())
    });

    interp.register("-inf", |i| {
        i.push(WofValue::double(f64::NEG_INFINITY));
        Ok(())
    });

    interp.register("nan", |i| {
        i.push(WofValue::double(f64::NAN));
        Ok(())
    });

    // Integer limits
    interp.register("max_int", |i| {
        i.push(WofValue::integer(i64::MAX));
        Ok(())
    });

    interp.register("min_int", |i| {
        i.push(WofValue::integer(i64::MIN));
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use woflang_core::InterpreterContext;

    fn make_interp() -> Interpreter {
        let mut interp = Interpreter::new();
        register(&mut interp);
        interp
    }

    #[test]
    fn test_pi() {
        let mut interp = make_interp();
        interp.exec_line("pi").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - std::f64::consts::PI).abs() < f64::EPSILON);
    }

    #[test]
    fn test_e() {
        let mut interp = make_interp();
        interp.exec_line("e").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - std::f64::consts::E).abs() < f64::EPSILON);
    }

    #[test]
    fn test_phi() {
        let mut interp = make_interp();
        interp.exec_line("phi").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - 1.618_033_988_749_895).abs() < 1e-10);
    }

    #[test]
    fn test_unicode_pi() {
        let mut interp = make_interp();
        interp.exec_line("π").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!((result - std::f64::consts::PI).abs() < f64::EPSILON);
    }

    #[test]
    fn test_infinity() {
        let mut interp = make_interp();
        interp.exec_line("inf").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!(result.is_infinite() && result > 0.0);
    }

    #[test]
    fn test_avogadro() {
        let mut interp = make_interp();
        interp.exec_line("avogadro").unwrap();
        let result = interp.stack().peek().unwrap().as_double().unwrap();
        assert!(result > 6e23);
    }
}
