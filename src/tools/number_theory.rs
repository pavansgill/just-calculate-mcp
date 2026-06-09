use crate::tools::shared::{gcd, lcm};
use schemars::JsonSchema;
use serde::Deserialize;

fn is_prime_impl(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    let mut i = 3u64;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 2;
    }
    true
}

fn prime_factors_impl(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        while n.is_multiple_of(d) {
            factors.push(d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

fn next_prime_after(n: u64) -> u64 {
    let mut candidate = n + 1;
    while !is_prime_impl(candidate) {
        candidate += 1;
    }
    candidate
}

// ── Number theory ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NumberTheoryInput {
    #[schemars(
        description = "Operation: gcd, lcm, is_prime, prime_factors, next_prime, nth_prime, divisors, divisor_count, sum_of_divisors, euler_totient, fibonacci, lucas, digit_sum, digital_root"
    )]
    pub operation: String,
    #[schemars(description = "Primary non-negative integer")]
    pub n: u64,
    #[schemars(description = "Second integer (required for gcd, lcm)")]
    pub m: Option<u64>,
}

pub fn number_theory(input: NumberTheoryInput) -> String {
    let n = input.n;
    match input.operation.as_str() {
        "gcd" => {
            let m = match input.m {
                Some(x) => x,
                None => return "Error: gcd requires m".to_string(),
            };
            format!("gcd({n}, {m}) = {}", gcd(n, m))
        }
        "lcm" => {
            let m = match input.m {
                Some(x) => x,
                None => return "Error: lcm requires m".to_string(),
            };
            format!("lcm({n}, {m}) = {}", lcm(n, m))
        }
        "is_prime" => format!("{n} is{} prime", if is_prime_impl(n) { "" } else { " not" }),
        "prime_factors" => {
            let f = prime_factors_impl(n);
            format!(
                "prime_factors({n}) = [{}]",
                f.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        "next_prime" => format!("next_prime after {n} = {}", next_prime_after(n)),
        "nth_prime" => {
            if n == 0 {
                return "Error: nth_prime is 1-indexed".to_string();
            }
            if n > 10_000 {
                return "Error: n too large (max 10,000)".to_string();
            }
            let mut count = 0u64;
            let mut candidate = 1u64;
            loop {
                candidate += 1;
                if is_prime_impl(candidate) {
                    count += 1;
                }
                if count == n {
                    break;
                }
            }
            format!("prime #{n} = {candidate}")
        }
        "divisors" => {
            let mut divs: Vec<u64> = (1..=n).filter(|&d| n.is_multiple_of(d)).collect();
            divs.sort();
            format!(
                "divisors({n}) = [{}]",
                divs.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        "divisor_count" => {
            let count = (1..=n).filter(|&d| n.is_multiple_of(d)).count();
            format!("divisor_count({n}) = {count}")
        }
        "sum_of_divisors" => {
            let sum: u64 = (1..=n).filter(|&d| n.is_multiple_of(d)).sum();
            format!("sum_of_divisors({n}) = {sum}")
        }
        "euler_totient" => {
            let result = (1..=n).filter(|&k| gcd(k, n) == 1).count() as u64;
            format!("φ({n}) = {result}")
        }
        "fibonacci" => {
            if n > 93 {
                return "Error: fibonacci({n}) overflows u64; max is 93".to_string();
            }
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 0..n {
                let t = a + b;
                a = b;
                b = t;
            }
            format!("fibonacci({n}) = {a}")
        }
        "lucas" => {
            if n > 92 {
                return "Error: lucas({n}) overflows u64; max is 92".to_string();
            }
            let (mut a, mut b) = (2u64, 1u64);
            for _ in 0..n {
                let t = a + b;
                a = b;
                b = t;
            }
            format!("lucas({n}) = {a}")
        }
        "digit_sum" => {
            let sum: u64 = n.to_string().chars().map(|c| c as u64 - '0' as u64).sum();
            format!("digit_sum({n}) = {sum}")
        }
        "digital_root" => {
            let result = if n == 0 { 0 } else { 1 + (n - 1) % 9 };
            format!("digital_root({n}) = {result}")
        }
        op => format!("Error: Unknown operation '{op}'"),
    }
}

// ── Modular arithmetic ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ModularArithmeticInput {
    #[schemars(description = "Operation: mod_pow, mod_inverse, mod_add, mod_multiply")]
    pub operation: String,
    #[schemars(description = "Base value a")]
    pub a: u64,
    #[schemars(
        description = "Exponent (mod_pow) or second operand (mod_inverse: unused, mod_add/mod_multiply: b)"
    )]
    pub b: u64,
    #[schemars(description = "Modulus m (must be > 0)")]
    pub modulus: u64,
}

fn mod_pow_impl(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as u64;
        }
        exp >>= 1;
        base = (base as u128 * base as u128 % modulus as u128) as u64;
    }
    result
}

pub fn modular_arithmetic(input: ModularArithmeticInput) -> String {
    let (a, b, m) = (input.a, input.b, input.modulus);
    if m == 0 {
        return "Error: modulus must be > 0".to_string();
    }

    match input.operation.as_str() {
        "mod_pow" => format!("{a}^{b} mod {m} = {}", mod_pow_impl(a, b, m)),
        "mod_inverse" => {
            // Extended Euclidean algorithm for modular inverse
            let g = gcd(a, m);
            if g != 1 { return format!("Error: mod_inverse({a}, {m}) does not exist (gcd={g})"); }
            // Find x such that a*x ≡ 1 (mod m) via Fermat's little theorem when m is prime,
            // or via repeated squaring with Euler's totient otherwise.
            // Use extended Euclidean:
            let (mut old_r, mut r) = (a as i64, m as i64);
            let (mut old_s, mut s) = (1i64, 0i64);
            while r != 0 {
                let q = old_r / r;
                (old_r, r) = (r, old_r - q * r);
                (old_s, s) = (s, old_s - q * s);
            }
            let inv = ((old_s % m as i64) + m as i64) as u64 % m;
            format!("mod_inverse({a}, {m}) = {inv}")
        }
        "mod_add" => format!("({a} + {b}) mod {m} = {}", (a % m + b % m) % m),
        "mod_multiply" => format!("({a} * {b}) mod {m} = {}", (a as u128 * b as u128 % m as u128) as u64),
        op => format!("Error: Unknown operation '{op}'. Supported: mod_pow, mod_inverse, mod_add, mod_multiply"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nt(n: u64, op: &str, m: Option<u64>) -> String {
        number_theory(NumberTheoryInput {
            operation: op.to_string(),
            n,
            m,
        })
    }

    #[test]
    fn test_gcd() {
        assert!(nt(12, "gcd", Some(8)).contains("4"));
    }
    #[test]
    fn test_is_prime_true() {
        assert!(nt(7, "is_prime", None).contains("is prime"));
    }
    #[test]
    fn test_is_prime_false() {
        assert!(nt(9, "is_prime", None).contains("is not prime"));
    }
    #[test]
    fn test_fibonacci() {
        assert!(nt(10, "fibonacci", None).contains("55"));
    }
    #[test]
    fn test_lucas() {
        assert!(nt(5, "lucas", None).contains("11"));
    }
    #[test]
    fn test_digit_sum() {
        assert!(nt(123, "digit_sum", None).contains("6"));
    }
    #[test]
    fn test_digital_root() {
        assert!(nt(493, "digital_root", None).contains("7"));
    }
    #[test]
    fn test_euler_totient() {
        assert!(nt(9, "euler_totient", None).contains("6"));
    }
    #[test]
    fn test_mod_pow() {
        let r = modular_arithmetic(ModularArithmeticInput {
            operation: "mod_pow".to_string(),
            a: 2,
            b: 10,
            modulus: 1000,
        });
        assert!(r.contains("24"), "{r}");
    }
    #[test]
    fn test_mod_inverse() {
        let r = modular_arithmetic(ModularArithmeticInput {
            operation: "mod_inverse".to_string(),
            a: 3,
            b: 0,
            modulus: 7,
        });
        assert!(r.contains("5"), "{r}");
    }
}
