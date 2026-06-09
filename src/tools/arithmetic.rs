use crate::tools::shared;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArithmeticInput {
    #[schemars(description = "First operand")]
    pub number_a: f64,
    #[schemars(description = "Second operand")]
    pub number_b: f64,
    #[schemars(description = "Operator: +, -, *, /, %")]
    pub operator: String,
}

pub fn compute(input: ArithmeticInput) -> String {
    let ArithmeticInput {
        number_a,
        number_b,
        operator,
    } = input;
    match operator.as_str() {
        "+" => format!("{number_a} + {number_b} = {}", number_a + number_b),
        "-" => format!("{number_a} - {number_b} = {}", number_a - number_b),
        "*" => format!("{number_a} * {number_b} = {}", number_a * number_b),
        "/" => {
            if number_b == 0.0 {
                "Error: Division by zero".to_string()
            } else {
                format!("{number_a} / {number_b} = {}", number_a / number_b)
            }
        }
        "%" => {
            if number_b == 0.0 {
                "Error: Modulus by zero".to_string()
            } else {
                format!("{number_a} % {number_b} = {}", number_a % number_b)
            }
        }
        _ => format!("Error: Unknown operator '{operator}'. Supported: +, -, *, /, %"),
    }
}

// ── Rounding ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RoundingInput {
    #[schemars(description = "The number to round")]
    pub value: f64,
    #[schemars(description = "Operation: floor, ceil, round, truncate, round_dp, round_sf")]
    pub operation: String,
    #[schemars(
        description = "Decimal places (round_dp) or significant figures (round_sf). Ignored for other operations."
    )]
    pub precision: Option<u32>,
}

pub fn round(input: RoundingInput) -> String {
    let v = input.value;
    match input.operation.as_str() {
        "floor" => format!("floor({v}) = {}", v.floor()),
        "ceil" => format!("ceil({v}) = {}", v.ceil()),
        "round" => format!("round({v}) = {}", v.round()),
        "truncate" => format!("truncate({v}) = {}", v.trunc()),
        "round_dp" => {
            let p = input.precision.unwrap_or(2);
            let factor = 10f64.powi(p as i32);
            let r = (v * factor).round() / factor;
            format!("round({v}, {p} dp) = {r}")
        }
        "round_sf" => {
            let p = input.precision.unwrap_or(3) as i32;
            if v == 0.0 {
                return format!("round({v}, {p} sf) = 0");
            }
            let magnitude = v.abs().log10().floor() as i32;
            let factor = 10f64.powi(p - 1 - magnitude);
            let r = (v * factor).round() / factor;
            format!("round({v}, {p} sf) = {r}")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: floor, ceil, round, truncate, round_dp, round_sf"),
    }
}

// ── Number properties ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NumberPropertiesInput {
    #[schemars(description = "Operation: abs, sign, reciprocal, min, max, clamp")]
    pub operation: String,
    #[schemars(description = "Primary value")]
    pub value: f64,
    #[schemars(description = "Second value (required for min, max); also low bound for clamp")]
    pub value_b: Option<f64>,
    #[schemars(description = "High bound (required for clamp)")]
    pub value_c: Option<f64>,
}

pub fn number_properties(input: NumberPropertiesInput) -> String {
    let v = input.value;
    match input.operation.as_str() {
        "abs" => format!("|{v}| = {}", v.abs()),
        "sign" => {
            let s = v.signum();
            format!("sign({v}) = {s}")
        }
        "reciprocal" => {
            if v == 0.0 {
                "Error: reciprocal of zero is undefined".to_string()
            } else {
                format!("1/{v} = {}", 1.0 / v)
            }
        }
        "min" => {
            let b = match input.value_b {
                Some(x) => x,
                None => return "Error: min requires value_b".to_string(),
            };
            format!("min({v}, {b}) = {}", v.min(b))
        }
        "max" => {
            let b = match input.value_b {
                Some(x) => x,
                None => return "Error: max requires value_b".to_string(),
            };
            format!("max({v}, {b}) = {}", v.max(b))
        }
        "clamp" => {
            let lo = match input.value_b {
                Some(x) => x,
                None => return "Error: clamp requires value_b (low) and value_c (high)".to_string(),
            };
            let hi = match input.value_c {
                Some(x) => x,
                None => return "Error: clamp requires value_c (high)".to_string(),
            };
            if lo > hi {
                return "Error: low bound must be <= high bound".to_string();
            }
            format!("clamp({v}, {lo}, {hi}) = {}", v.clamp(lo, hi))
        }
        "gcd" => {
            let b = match input.value_b {
                Some(x) => x,
                None => return "Error: gcd requires value_b".to_string(),
            };
            if v.fract() != 0.0 || b.fract() != 0.0 {
                return "Error: gcd requires integer values".to_string();
            }
            let result = shared::gcd(v.abs() as u64, b.abs() as u64);
            format!("gcd({v}, {b}) = {result}")
        }
        "lcm" => {
            let b = match input.value_b {
                Some(x) => x,
                None => return "Error: lcm requires value_b".to_string(),
            };
            if v.fract() != 0.0 || b.fract() != 0.0 {
                return "Error: lcm requires integer values".to_string();
            }
            let result = shared::lcm(v.abs() as u64, b.abs() as u64);
            format!("lcm({v}, {b}) = {result}")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: abs, sign, reciprocal, min, max, clamp, gcd, lcm"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(a: f64, b: f64, op: &str) -> String {
        compute(ArithmeticInput {
            number_a: a,
            number_b: b,
            operator: op.to_string(),
        })
    }

    #[test]
    fn test_addition() {
        assert_eq!(run(3.0, 4.0, "+"), "3 + 4 = 7");
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(run(10.0, 3.0, "-"), "10 - 3 = 7");
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(run(6.0, 7.0, "*"), "6 * 7 = 42");
    }

    #[test]
    fn test_division() {
        assert_eq!(run(10.0, 2.0, "/"), "10 / 2 = 5");
    }

    #[test]
    fn test_modulus() {
        assert_eq!(run(10.0, 3.0, "%"), "10 % 3 = 1");
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(run(5.0, 0.0, "/"), "Error: Division by zero");
    }

    #[test]
    fn test_modulus_by_zero() {
        assert_eq!(run(5.0, 0.0, "%"), "Error: Modulus by zero");
    }

    #[test]
    fn test_unknown_operator() {
        assert!(run(1.0, 2.0, "^").contains("Unknown operator"));
    }

    fn rnd(v: f64, op: &str, p: Option<u32>) -> String {
        round(RoundingInput {
            value: v,
            operation: op.to_string(),
            precision: p,
        })
    }

    #[test]
    fn test_floor() {
        assert_eq!(rnd(3.7, "floor", None), "floor(3.7) = 3");
    }
    #[test]
    fn test_ceil() {
        assert_eq!(rnd(3.2, "ceil", None), "ceil(3.2) = 4");
    }
    #[test]
    fn test_round() {
        assert_eq!(rnd(3.5, "round", None), "round(3.5) = 4");
    }
    #[test]
    fn test_truncate() {
        assert_eq!(rnd(-3.9, "truncate", None), "truncate(-3.9) = -3");
    }
    #[test]
    fn test_round_dp() {
        assert!(rnd(5.678, "round_dp", Some(2)).contains("5.68"));
    }

    fn prop(v: f64, op: &str, b: Option<f64>, c: Option<f64>) -> String {
        number_properties(NumberPropertiesInput {
            operation: op.to_string(),
            value: v,
            value_b: b,
            value_c: c,
        })
    }

    #[test]
    fn test_abs() {
        assert_eq!(prop(-5.0, "abs", None, None), "|-5| = 5");
    }
    #[test]
    fn test_sign() {
        assert!(prop(-3.0, "sign", None, None).contains("-1"));
    }
    #[test]
    fn test_reciprocal() {
        assert!(prop(4.0, "reciprocal", None, None).contains("0.25"));
    }
    #[test]
    fn test_min() {
        assert!(prop(3.0, "min", Some(7.0), None).contains("3"));
    }
    #[test]
    fn test_max() {
        assert!(prop(3.0, "max", Some(7.0), None).contains("7"));
    }
    #[test]
    fn test_clamp() {
        assert!(prop(10.0, "clamp", Some(0.0), Some(5.0)).contains("5"));
    }
    #[test]
    fn test_gcd() {
        assert!(prop(12.0, "gcd", Some(8.0), None).contains("4"));
    }
    #[test]
    fn test_lcm() {
        assert!(prop(4.0, "lcm", Some(6.0), None).contains("12"));
    }
}
