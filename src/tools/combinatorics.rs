use crate::tools::shared::factorial;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CombinatoricsInput {
    #[schemars(
        description = "Operation: factorial, double_factorial, permutations, combinations, binomial_coeff, catalan"
    )]
    pub operation: String,
    #[schemars(description = "n (primary non-negative integer)")]
    pub n: u64,
    #[schemars(
        description = "k (required for permutations, combinations, binomial_coeff; k <= n)"
    )]
    pub k: Option<u64>,
}

pub fn combinatorics(input: CombinatoricsInput) -> String {
    let n = input.n;
    match input.operation.as_str() {
        "factorial" => match factorial(n) {
            Ok(r) => format!("{n}! = {r}"),
            Err(e) => format!("Error: {e}"),
        },
        "double_factorial" => {
            // n!! = n * (n-2) * (n-4) * ... down to 1 or 2
            if n > 33 { return "Error: double_factorial({n}) overflows u64; max is 33".to_string(); }
            let result: u64 = (1..=n).rev().step_by(2).product();
            format!("{n}!! = {result}")
        }
        "permutations" => {
            let k = match input.k { Some(x) => x, None => return "Error: permutations requires k".to_string() };
            if k > n { return format!("Error: k ({k}) must be <= n ({n})"); }
            let result: u128 = ((n - k + 1)..=n).map(|x| x as u128).product();
            format!("P({n},{k}) = {result}")
        }
        "combinations" => {
            let k = match input.k { Some(x) => x, None => return "Error: combinations requires k".to_string() };
            if k > n { return format!("Error: k ({k}) must be <= n ({n})"); }
            let k = k.min(n - k);
            let num: u128 = ((n - k + 1)..=n).map(|x| x as u128).product();
            let den: u128 = (1..=k).map(|x| x as u128).product();
            format!("C({n},{}) = {}", input.k.unwrap(), num / den)
        }
        "binomial_coeff" => {
            let k = match input.k { Some(x) => x, None => return "Error: binomial_coeff requires k".to_string() };
            if k > n { return format!("Error: k ({k}) must be <= n ({n})"); }
            let k = k.min(n - k);
            let num: u128 = ((n - k + 1)..=n).map(|x| x as u128).product();
            let den: u128 = (1..=k).map(|x| x as u128).product();
            format!("C({n},{}) = {}", input.k.unwrap(), num / den)
        }
        "catalan" => {
            if n > 30 { return "Error: catalan({n}) overflows u64; max is 30".to_string(); }
            // C(n) = C(2n, n) / (n+1)
            let two_n = 2 * n;
            let k = n.min(two_n - n);
            let num: u128 = ((two_n - k + 1)..=two_n).map(|x| x as u128).product();
            let den: u128 = (1..=k).map(|x| x as u128).product();
            let binom = num / den;
            let result = binom / (n as u128 + 1);
            format!("Catalan({n}) = {result}")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: factorial, double_factorial, permutations, combinations, binomial_coeff, catalan"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(n: u64, op: &str, k: Option<u64>) -> String {
        combinatorics(CombinatoricsInput {
            operation: op.to_string(),
            n,
            k,
        })
    }

    #[test]
    fn test_factorial() {
        assert_eq!(c(5, "factorial", None), "5! = 120");
    }
    #[test]
    fn test_factorial_zero() {
        assert_eq!(c(0, "factorial", None), "0! = 1");
    }
    #[test]
    fn test_double_factorial() {
        assert_eq!(c(5, "double_factorial", None), "5!! = 15");
    }
    #[test]
    fn test_permutations() {
        assert!(c(5, "permutations", Some(2)).contains("20"));
    }
    #[test]
    fn test_combinations() {
        assert!(c(5, "combinations", Some(2)).contains("10"));
    }
    #[test]
    fn test_catalan_zero() {
        assert!(c(0, "catalan", None).contains("1"));
    }
    #[test]
    fn test_catalan_5() {
        assert!(c(5, "catalan", None).contains("42"));
    }
    #[test]
    fn test_k_exceeds_n() {
        assert!(c(3, "combinations", Some(5)).contains("Error"));
    }
}
