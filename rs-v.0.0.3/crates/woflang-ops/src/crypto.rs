//! Cryptographic and number-theoretic operations.
//!
//! | Operation       | Stack Effect | Description |
//! |-----------------|--------------|-------------|
//! | `prime_check`   | (n -- b)     | Miller-Rabin primality test |
//! | `random`        | (lo hi -- n) | Random integer in range |
//! | `hash`          | (n -- h)     | Simple hash function |
//! | `gcd`           | (a b -- c)   | Greatest common divisor |
//! | `mod_exp`       | (b e m -- r) | Modular exponentiation |
//! | `diffie_hellman`| ( -- )       | Demonstrate DH key exchange |

use rand::Rng;
use woflang_core::{InterpreterContext, Result, WofError, WofValue};
use woflang_runtime::Interpreter;

/// Register all cryptographic operations.
pub fn register(interp: &mut Interpreter) {
    // Primality
    interp.register("prime_check", op_prime_check);
    interp.register("is_prime", op_prime_check);
    interp.register("next_prime", op_next_prime);

    // Random
    interp.register("random", op_random);
    interp.register("rand", op_rand);

    // Hashing
    interp.register("hash", op_hash);

    // Modular arithmetic
    interp.register("mod_exp", op_mod_exp);
    interp.register("mod_inv", op_mod_inv);

    // Key exchange demo
    interp.register("diffie_hellman", op_diffie_hellman);

    // Encoding
    interp.register("base64_encode", op_base64_encode);
}

/// Miller-Rabin primality test with deterministic witnesses for 64-bit integers.
fn is_prime_miller_rabin(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // Write n-1 as 2^r * d
    let mut d = n - 1;
    let mut r = 0u32;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    // Deterministic witnesses for 64-bit integers
    // These witnesses are sufficient for all n < 2^64
    let witnesses: &[u64] = if n < 2047 {
        &[2]
    } else if n < 1_373_653 {
        &[2, 3]
    } else if n < 9_080_191 {
        &[31, 73]
    } else if n < 25_326_001 {
        &[2, 3, 5]
    } else if n < 3_215_031_751 {
        &[2, 3, 5, 7]
    } else if n < 4_759_123_141 {
        &[2, 7, 61]
    } else if n < 1_122_004_669_633 {
        &[2, 13, 23, 1662803]
    } else if n < 2_152_302_898_747 {
        &[2, 3, 5, 7, 11]
    } else if n < 3_474_749_660_383 {
        &[2, 3, 5, 7, 11, 13]
    } else if n < 341_550_071_728_321 {
        &[2, 3, 5, 7, 11, 13, 17]
    } else {
        &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    };

    'witness: for &a in witnesses {
        if a >= n {
            continue;
        }

        // Compute a^d mod n
        let mut x = mod_pow(a, d, n);

        if x == 1 || x == n - 1 {
            continue 'witness;
        }

        for _ in 0..r - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                continue 'witness;
            }
        }

        return false;
    }

    true
}

/// Modular exponentiation: base^exp mod modulus
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1u128;
    let modulus = modulus as u128;
    base = (base as u128 % modulus) as u64;

    let mut base = base as u128;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp /= 2;
        base = (base * base) % modulus;
    }

    result as u64
}

fn op_prime_check(interp: &mut Interpreter) -> Result<()> {
    let n = interp.stack_mut().pop_integer()?;
    if n < 0 {
        interp.push(WofValue::boolean(false));
        return Ok(());
    }

    let is_prime = is_prime_miller_rabin(n as u64);
    interp.push(WofValue::boolean(is_prime));
    Ok(())
}

fn op_next_prime(interp: &mut Interpreter) -> Result<()> {
    let mut n = interp.stack_mut().pop_integer()?;
    if n < 2 {
        interp.push(WofValue::integer(2));
        return Ok(());
    }

    // Start from next odd number
    if n % 2 == 0 {
        n += 1;
    } else {
        n += 2;
    }

    while !is_prime_miller_rabin(n as u64) {
        n += 2;
        if n < 0 {
            return Err(WofError::Overflow("next_prime overflow".into()));
        }
    }

    interp.push(WofValue::integer(n));
    Ok(())
}

fn op_random(interp: &mut Interpreter) -> Result<()> {
    let hi = interp.stack_mut().pop_integer()?;
    let lo = interp.stack_mut().pop_integer()?;

    if lo > hi {
        return Err(WofError::InvalidArgument(format!(
            "random: lo ({lo}) > hi ({hi})"
        )));
    }

    let mut rng = rand::thread_rng();
    let value = rng.gen_range(lo..=hi);
    interp.push(WofValue::integer(value));
    Ok(())
}

fn op_rand(interp: &mut Interpreter) -> Result<()> {
    let value: f64 = rand::random();
    interp.push(WofValue::double(value));
    Ok(())
}

fn op_hash(interp: &mut Interpreter) -> Result<()> {
    let n = interp.stack_mut().pop_integer()? as u64;

    // FNV-1a hash
    let mut hash: u64 = 0xcbf29ce484222325;
    let bytes = n.to_le_bytes();
    for byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }

    interp.push(WofValue::integer(hash as i64));
    Ok(())
}

fn op_mod_exp(interp: &mut Interpreter) -> Result<()> {
    let m = interp.stack_mut().pop_integer()? as u64;
    let e = interp.stack_mut().pop_integer()? as u64;
    let b = interp.stack_mut().pop_integer()? as u64;

    if m == 0 {
        return Err(WofError::DivisionByZero);
    }

    let result = mod_pow(b, e, m);
    interp.push(WofValue::integer(result as i64));
    Ok(())
}

fn op_mod_inv(interp: &mut Interpreter) -> Result<()> {
    let m = interp.stack_mut().pop_integer()?;
    let a = interp.stack_mut().pop_integer()?;

    if m <= 0 {
        return Err(WofError::InvalidArgument("modulus must be positive".into()));
    }

    // Extended Euclidean algorithm
    let (gcd, x, _) = extended_gcd(a, m);

    if gcd != 1 {
        return Err(WofError::InvalidArgument(format!(
            "no modular inverse: gcd({a}, {m}) = {gcd}"
        )));
    }

    let result = ((x % m) + m) % m;
    interp.push(WofValue::integer(result));
    Ok(())
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x, y) = extended_gcd(b % a, a);
        (gcd, y - (b / a) * x, x)
    }
}

fn op_diffie_hellman(interp: &mut Interpreter) -> Result<()> {
    // Demonstrate Diffie-Hellman with small parameters
    let mut rng = rand::thread_rng();

    // Small safe prime for demonstration
    let p: u64 = 23;
    let g: u64 = 5;

    // Alice's private key
    let a: u64 = rng.gen_range(2..p - 1);
    // Bob's private key
    let b: u64 = rng.gen_range(2..p - 1);

    // Public values
    let big_a = mod_pow(g, a, p); // g^a mod p
    let big_b = mod_pow(g, b, p); // g^b mod p

    // Shared secrets (should be equal)
    let secret_a = mod_pow(big_b, a, p); // B^a mod p = g^(ab) mod p
    let secret_b = mod_pow(big_a, b, p); // A^b mod p = g^(ab) mod p

    println!("Diffie-Hellman Demo (p={p}, g={g}):");
    println!("  Alice: private={a}, public={big_a}");
    println!("  Bob:   private={b}, public={big_b}");
    println!("  Shared secret: {secret_a} (verified: {})", secret_a == secret_b);

    interp.push(WofValue::integer(secret_a as i64));
    Ok(())
}

fn op_base64_encode(interp: &mut Interpreter) -> Result<()> {
    let n = interp.stack_mut().pop_integer()? as u64;

    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let bytes = n.to_be_bytes();
    let mut result = String::new();

    // Simple base64 encoding of the 8 bytes
    let mut i = 0;
    while i < 8 {
        let b0 = bytes.get(i).copied().unwrap_or(0);
        let b1 = bytes.get(i + 1).copied().unwrap_or(0);
        let b2 = bytes.get(i + 2).copied().unwrap_or(0);

        result.push(ALPHABET[(b0 >> 2) as usize] as char);
        result.push(ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        result.push(ALPHABET[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        result.push(ALPHABET[(b2 & 0x3f) as usize] as char);

        i += 3;
    }

    interp.push(WofValue::string(result));
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
    fn test_small_primes() {
        assert!(is_prime_miller_rabin(2));
        assert!(is_prime_miller_rabin(3));
        assert!(is_prime_miller_rabin(5));
        assert!(is_prime_miller_rabin(7));
        assert!(is_prime_miller_rabin(97));
        assert!(is_prime_miller_rabin(997));
    }

    #[test]
    fn test_composites() {
        assert!(!is_prime_miller_rabin(0));
        assert!(!is_prime_miller_rabin(1));
        assert!(!is_prime_miller_rabin(4));
        assert!(!is_prime_miller_rabin(15));
        assert!(!is_prime_miller_rabin(100));
    }

    #[test]
    fn test_carmichael_numbers() {
        // Carmichael numbers fool Fermat primality test but not Miller-Rabin
        assert!(!is_prime_miller_rabin(561));
        assert!(!is_prime_miller_rabin(1105));
        assert!(!is_prime_miller_rabin(1729));
    }

    #[test]
    fn test_large_primes() {
        assert!(is_prime_miller_rabin(2_147_483_647)); // 2^31 - 1 (Mersenne prime)
        assert!(is_prime_miller_rabin(1_000_000_007));
        assert!(is_prime_miller_rabin(1_000_000_009));
    }

    #[test]
    fn test_prime_check_op() {
        let mut interp = make_interp();

        interp.exec_line("17 prime_check").unwrap();
        assert!(interp.stack_mut().pop_bool().unwrap());

        interp.exec_line("15 prime_check").unwrap();
        assert!(!interp.stack_mut().pop_bool().unwrap());
    }

    #[test]
    fn test_random_range() {
        let mut interp = make_interp();
        interp.exec_line("1 100 random").unwrap();
        let val = interp.stack().peek().unwrap().as_integer().unwrap();
        assert!((1..=100).contains(&val));
    }

    #[test]
    fn test_mod_exp() {
        let mut interp = make_interp();
        // 2^10 mod 1000 = 1024 mod 1000 = 24
        interp.exec_line("2 10 1000 mod_exp").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 24);
    }

    #[test]
    fn test_hash_deterministic() {
        let mut interp = make_interp();
        interp.exec_line("42 hash").unwrap();
        let h1 = interp.stack_mut().pop_integer().unwrap();

        interp.exec_line("42 hash").unwrap();
        let h2 = interp.stack_mut().pop_integer().unwrap();

        assert_eq!(h1, h2);
    }

    #[test]
    fn test_next_prime() {
        let mut interp = make_interp();
        interp.exec_line("10 next_prime").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 11);

        interp.exec_line("11 next_prime").unwrap();
        assert_eq!(interp.stack().peek().unwrap().as_integer().unwrap(), 13);
    }
}
