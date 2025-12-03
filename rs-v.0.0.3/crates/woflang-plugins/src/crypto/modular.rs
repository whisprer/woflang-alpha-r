//! Modular arithmetic operations for Woflang.
//!
//! Provides modular exponentiation, inverse, GCD/LCM, and related functions.

use woflang_core::{WofError, WofValue};
use woflang_runtime::Interpreter;

/// Register modular arithmetic operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // MODULAR EXPONENTIATION
    // ═══════════════════════════════════════════════════════════════
    
    // Modular exponentiation: base^exp mod m
    // Stack: base exp mod → result
    interp.register("modexp", |interp| {
        let m = interp.stack_mut().pop()?.as_integer()?;
        let exp = interp.stack_mut().pop()?.as_integer()?;
        let base = interp.stack_mut().pop()?.as_integer()?;
        
        if m <= 0 {
            return Err(WofError::Runtime("modexp: modulus must be positive".into()));
        }
        if exp < 0 {
            return Err(WofError::Runtime("modexp: exponent must be non-negative".into()));
        }
        
        let result = mod_pow(base as u64, exp as u64, m as u64);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // MODULAR INVERSE
    // ═══════════════════════════════════════════════════════════════
    
    // Modular inverse: a^(-1) mod m (where a * result ≡ 1 mod m)
    // Stack: a m → inverse (or error if not coprime)
    interp.register("modinv", |interp| {
        let m = interp.stack_mut().pop()?.as_integer()?;
        let a = interp.stack_mut().pop()?.as_integer()?;
        
        if m <= 0 {
            return Err(WofError::Runtime("modinv: modulus must be positive".into()));
        }
        
        match mod_inverse(a, m) {
            Some(inv) => {
                interp.stack_mut().push(WofValue::integer(inv));
                Ok(())
            }
            None => {
                Err(WofError::Runtime(format!(
                    "modinv: {} has no inverse mod {} (not coprime)", a, m
                )))
            }
        }
    });

    // ═══════════════════════════════════════════════════════════════
    // EXTENDED GCD
    // ═══════════════════════════════════════════════════════════════
    
    // Extended GCD: returns (gcd, x, y) where ax + by = gcd
    // Stack: a b → gcd x y
    interp.register("extgcd", |interp| {
        let b = interp.stack_mut().pop()?.as_integer()?;
        let a = interp.stack_mut().pop()?.as_integer()?;
        
        let (g, x, y) = extended_gcd(a, b);
        interp.stack_mut().push(WofValue::integer(g));
        interp.stack_mut().push(WofValue::integer(x));
        interp.stack_mut().push(WofValue::integer(y));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // MODULAR ARITHMETIC
    // ═══════════════════════════════════════════════════════════════
    
    // Modular addition: (a + b) mod m
    // Stack: a b m → result
    interp.register("modadd", |interp| {
        let m = interp.stack_mut().pop()?.as_integer()?;
        let b = interp.stack_mut().pop()?.as_integer()?;
        let a = interp.stack_mut().pop()?.as_integer()?;
        
        if m <= 0 {
            return Err(WofError::Runtime("modadd: modulus must be positive".into()));
        }
        
        let result = ((a % m) + (b % m)) % m;
        let result = if result < 0 { result + m } else { result };
        interp.stack_mut().push(WofValue::integer(result));
        Ok(())
    });

    // Modular subtraction: (a - b) mod m
    // Stack: a b m → result
    interp.register("modsub", |interp| {
        let m = interp.stack_mut().pop()?.as_integer()?;
        let b = interp.stack_mut().pop()?.as_integer()?;
        let a = interp.stack_mut().pop()?.as_integer()?;
        
        if m <= 0 {
            return Err(WofError::Runtime("modsub: modulus must be positive".into()));
        }
        
        let result = ((a % m) - (b % m)) % m;
        let result = if result < 0 { result + m } else { result };
        interp.stack_mut().push(WofValue::integer(result));
        Ok(())
    });

    // Modular multiplication: (a * b) mod m
    // Stack: a b m → result
    interp.register("modmul", |interp| {
        let m = interp.stack_mut().pop()?.as_integer()?;
        let b = interp.stack_mut().pop()?.as_integer()?;
        let a = interp.stack_mut().pop()?.as_integer()?;
        
        if m <= 0 {
            return Err(WofError::Runtime("modmul: modulus must be positive".into()));
        }
        
        let result = mod_mul(a as u64, b as u64, m as u64);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // EULER'S TOTIENT
    // ═══════════════════════════════════════════════════════════════
    
    // Euler's totient function φ(n)
    interp.register("totient", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        if n <= 0 {
            return Err(WofError::Runtime("totient: n must be positive".into()));
        }
        
        let result = euler_totient(n as u64);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    interp.register("φ", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        if n <= 0 {
            return Err(WofError::Runtime("φ: n must be positive".into()));
        }
        
        let result = euler_totient(n as u64);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // COPRIMALITY
    // ═══════════════════════════════════════════════════════════════
    
    // Check if a and b are coprime
    interp.register("coprime?", |interp| {
        let b = interp.stack_mut().pop()?.as_integer()?.unsigned_abs();
        let a = interp.stack_mut().pop()?.as_integer()?.unsigned_abs();
        let result = if gcd(a, b) == 1 { 1 } else { 0 };
        interp.stack_mut().push(WofValue::integer(result));
        Ok(())
    });
}

// ═══════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════

/// Modular exponentiation: base^exp mod m.
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0; }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, modulus);
        }
        exp /= 2;
        base = mod_mul(base, base, modulus);
    }
    result
}

/// Modular multiplication avoiding overflow.
fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

/// Extended Euclidean algorithm.
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a.abs(), if a >= 0 { 1 } else { -1 }, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/// Modular inverse using extended GCD.
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        None
    } else {
        let result = x % m;
        Some(if result < 0 { result + m } else { result })
    }
}

/// GCD using Euclidean algorithm.
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Euler's totient function.
fn euler_totient(mut n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut result = n;
    
    // Check for factor 2
    if n % 2 == 0 {
        result -= result / 2;
        while n % 2 == 0 { n /= 2; }
    }
    
    // Check odd factors
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            result -= result / i;
            while n % i == 0 { n /= i; }
        }
        i += 2;
    }
    
    // If n is still > 1, it's a prime factor
    if n > 1 {
        result -= result / n;
    }
    
    result
}
