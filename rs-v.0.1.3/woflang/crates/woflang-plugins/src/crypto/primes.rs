//! Prime number operations for Woflang.
//!
//! Provides primality testing, prime generation, and factorization.

use woflang_core::{WofError, WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

/// Register prime number operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // PRIMALITY TESTING
    // ═══════════════════════════════════════════════════════════════
    
    // Miller-Rabin primality test (probabilistic but accurate for 64-bit)
    interp.register("is_prime_mr", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        let result = if n <= 1 { 0 } else { if miller_rabin(n as u64) { 1 } else { 0 } };
        interp.stack_mut().push(WofValue::integer(result));
        Ok(())
    });

    // Fermat primality test
    interp.register("is_prime_fermat", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        let result = if n <= 1 { 0 } else { if fermat_test(n as u64, 10) { 1 } else { 0 } };
        interp.stack_mut().push(WofValue::integer(result));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // FACTORIZATION
    // ═══════════════════════════════════════════════════════════════
    
    // Prime factorization - push all factors onto stack
    interp.register("prime_factors", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        let factors = prime_factors(n.unsigned_abs());
        for f in factors {
            interp.stack_mut().push(WofValue::integer(f as i64));
        }
        Ok(())
    });

    // Count of prime factors (with multiplicity)
    interp.register("omega", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        let factors = prime_factors(n.unsigned_abs());
        interp.stack_mut().push(WofValue::integer(factors.len() as i64));
        Ok(())
    });

    // Count of distinct prime factors
    interp.register("omega_distinct", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        let mut factors = prime_factors(n.unsigned_abs());
        factors.sort_unstable();
        factors.dedup();
        interp.stack_mut().push(WofValue::integer(factors.len() as i64));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // PRIME GENERATION
    // ═══════════════════════════════════════════════════════════════
    
    // Nth prime (1-indexed: prime(1) = 2)
    interp.register("nth_prime", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        if n <= 0 {
            return Err(WofError::Runtime("nth_prime: n must be >= 1".into()));
        }
        let result = nth_prime(n as usize);
        interp.stack_mut().push(WofValue::integer(result as i64));
        Ok(())
    });

    // Prime counting function π(n)
    interp.register("prime_pi", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        if n < 0 {
            interp.stack_mut().push(WofValue::integer(0));
            return Ok(());
        }
        let count = prime_count(n as u64);
        interp.stack_mut().push(WofValue::integer(count as i64));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // SIEVE
    // ═══════════════════════════════════════════════════════════════
    
    // Generate primes up to n, push count onto stack
    interp.register("sieve", |interp| {
        let n = interp.stack_mut().pop()?.as_integer()?;
        if n < 2 {
            interp.stack_mut().push(WofValue::integer(0));
            return Ok(());
        }
        let primes = sieve_of_eratosthenes(n as usize);
        let count = primes.len();
        for p in primes {
            interp.stack_mut().push(WofValue::integer(p as i64));
        }
        interp.stack_mut().push(WofValue::integer(count as i64));
        Ok(())
    });
}

// ═══════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════

/// Miller-Rabin primality test (deterministic for 64-bit integers).
fn miller_rabin(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 { return false; }

    // Write n-1 as 2^r * d
    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    // Witnesses sufficient for 64-bit integers
    let witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    
    'witness: for &a in &witnesses {
        if a >= n { continue; }
        
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 { continue; }
        
        for _ in 0..r - 1 {
            x = mod_mul(x, x, n);
            if x == n - 1 { continue 'witness; }
        }
        return false;
    }
    true
}

/// Fermat primality test.
fn fermat_test(n: u64, iterations: u32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }

    for i in 0..iterations {
        let a = 2 + (i as u64 % (n - 2));
        if mod_pow(a, n - 1, n) != 1 {
            return false;
        }
    }
    true
}

/// Modular exponentiation: a^b mod m.
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

/// Prime factorization by trial division.
fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    if n <= 1 { return factors; }

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let mut d = 3;
    while d * d <= n {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 2;
    }

    if n > 1 {
        factors.push(n);
    }
    factors
}

/// Sieve of Eratosthenes.
fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    if limit < 2 { return Vec::new(); }
    
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    is_prime.iter().enumerate()
        .filter(|(_, &p)| p)
        .map(|(i, _)| i)
        .collect()
}

/// Get the nth prime (1-indexed).
fn nth_prime(n: usize) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 2; }
    
    let mut count = 1;
    let mut candidate = 3u64;
    
    while count < n {
        if miller_rabin(candidate) {
            count += 1;
            if count == n { return candidate; }
        }
        candidate += 2;
    }
    candidate
}

/// Count primes up to n.
fn prime_count(n: u64) -> usize {
    if n < 2 { return 0; }
    sieve_of_eratosthenes(n as usize).len()
}
