//! Discrete mathematics operations for Woflang.
//!
//! Provides factorial, fibonacci, GCD, LCM, combinations, permutations,
//! and other number theory functions.

use woflang_core::{WofError, WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

/// Register discrete math operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // FACTORIAL AND COMBINATORICS
    // ═══════════════════════════════════════════════════════════════
    
    // Factorial: n!
    interp.register("factorial", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        if n < 0 {
            return Err(WofError::Runtime("factorial: n must be >= 0".into()));
        }
        let result = factorial(n as u64);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    interp.register("!", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        if n < 0 {
            return Err(WofError::Runtime("!: n must be >= 0".into()));
        }
        let result = factorial(n as u64);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    // Combinations: C(n, k) = n! / (k! * (n-k)!)
    // Stack: n k → C(n,k)
    interp.register("choose", |interp| {
        let k = interp.stack_mut().pop()?.as_integer()? as u64;
        let n = interp.stack_mut().pop()?.as_integer()? as u64;
        let result = combinations(n, k);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    interp.register("C", |interp| {
        let k = interp.stack_mut().pop()?.as_integer()? as u64;
        let n = interp.stack_mut().pop()?.as_integer()? as u64;
        let result = combinations(n, k);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    // Permutations: P(n, k) = n! / (n-k)!
    // Stack: n k → P(n,k)
    interp.register("permute", |interp| {
        let k = interp.stack_mut().pop()?.as_integer()? as u64;
        let n = interp.stack_mut().pop()?.as_integer()? as u64;
        let result = permutations(n, k);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    interp.register("P", |interp| {
        let k = interp.stack_mut().pop()?.as_integer()? as u64;
        let n = interp.stack_mut().pop()?.as_integer()? as u64;
        let result = permutations(n, k);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // NUMBER THEORY
    // ═══════════════════════════════════════════════════════════════
    
    // Greatest Common Divisor
    interp.register("gcd", |interp| {
        let b = interp.stack_mut().pop()?.as_integer()?.unsigned_abs();
        let a = interp.stack_mut().pop()?.as_integer()?.unsigned_abs();
        interp.stack_mut().push(WofValue::integer(gcd(a, b) as i64));
        Ok(())
    });

    // Least Common Multiple
    interp.register("lcm", |interp| {
        let b = interp.stack_mut().pop()?.as_integer()?.unsigned_abs();
        let a = interp.stack_mut().pop()?.as_integer()?.unsigned_abs();
        interp.stack_mut().push(WofValue::integer(lcm(a, b) as i64));
        Ok(())
    });

    // Fibonacci
    interp.register("fib", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        if n < 0 {
            return Err(WofError::Runtime("fib: n must be >= 0".into()));
        }
        let result = fibonacci(n as u64);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    // Is prime?
    interp.register("is_prime", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        let result = if n <= 1 { 0 } else { if is_prime(n as u64) { 1 } else { 0 } };
        interp.stack_mut().push(WofValue::integer(result));
        Ok(())
    });

    // Next prime
    interp.register("next_prime", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?.max(1) as u64;
        let result = next_prime(n);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // SEQUENCES
    // ═══════════════════════════════════════════════════════════════
    
    // Triangular number: T(n) = n(n+1)/2
    interp.register("triangular", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        let result = n * (n + 1) / 2;
        interp.stack_mut().push(WofValue::integer(result));
        Ok(())
    });

    // Square number: n²
    interp.register("square", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        interp.stack_mut().push(WofValue::integer(n * n));
        Ok(())
    });

    // Cube number: n³
    interp.register("cube", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        interp.stack_mut().push(WofValue::integer(n * n * n));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // BIT OPERATIONS (Integer)
    // ═══════════════════════════════════════════════════════════════
    
    interp.register("popcount", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        interp.stack_mut().push(WofValue::integer(n.count_ones() as i64));
        Ok(())
    });

    interp.register("leading_zeros", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        interp.stack_mut().push(WofValue::integer(n.leading_zeros() as i64));
        Ok(())
    });

    interp.register("trailing_zeros", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        interp.stack_mut().push(WofValue::integer(n.trailing_zeros() as i64));
        Ok(())
    });

    interp.register("bit_reverse", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        interp.stack_mut().push(WofValue::integer(n.reverse_bits()));
        Ok(())
    });
}

// ═══════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════

fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { (2..=n).product() }
}

fn combinations(n: u64, k: u64) -> u64 {
    if k > n { return 0; }
    if k == 0 || k == n { return 1; }
    let k = k.min(n - k); // Use symmetry
    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

fn permutations(n: u64, k: u64) -> u64 {
    if k > n { return 0; }
    (n - k + 1..=n).product()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { 0 } else { a / gcd(a, b) * b }
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 { return n; }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let t = a + b;
        a = b;
        b = t;
    }
    b
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let limit = (n as f64).sqrt() as u64 + 1;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

fn next_prime(n: u64) -> u64 {
    let mut candidate = if n < 2 { 2 } else { n + 1 };
    while !is_prime(candidate) {
        candidate += 1;
    }
    candidate
}
