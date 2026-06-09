use schemars::JsonSchema;
use serde::Deserialize;

// ── Power / Root ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PowerRootInput {
    #[schemars(description = "Operation: square, cube, sqrt, cbrt, nth_root, pow")]
    pub operation: String,
    #[schemars(description = "Base value")]
    pub value: f64,
    #[schemars(description = "Exponent for pow; root degree for nth_root")]
    pub exponent: Option<f64>,
}

pub fn power_root(input: PowerRootInput) -> String {
    let v = input.value;
    match input.operation.as_str() {
        "square" => format!("{v}² = {}", v * v),
        "cube" => format!("{v}³ = {}", v * v * v),
        "sqrt" => {
            if v < 0.0 {
                "Error: sqrt of negative number is not real".to_string()
            } else {
                format!("sqrt({v}) = {}", v.sqrt())
            }
        }
        "cbrt" => format!("cbrt({v}) = {}", v.cbrt()),
        "nth_root" => {
            let n = match input.exponent {
                Some(x) => x,
                None => return "Error: nth_root requires exponent (the root degree)".to_string(),
            };
            if n == 0.0 {
                return "Error: zeroth root is undefined".to_string();
            }
            let result = v.powf(1.0 / n);
            format!("{v}^(1/{n}) = {result}")
        }
        "pow" => {
            let exp = match input.exponent {
                Some(x) => x,
                None => return "Error: pow requires exponent".to_string(),
            };
            format!("{v}^{exp} = {}", v.powf(exp))
        }
        op => format!(
            "Error: Unknown operation '{op}'. Supported: square, cube, sqrt, cbrt, nth_root, pow"
        ),
    }
}

// ── Exponential ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExponentialInput {
    #[schemars(description = "Operation: exp, expm1, exp2, exp10")]
    pub operation: String,
    #[schemars(description = "The exponent value")]
    pub value: f64,
}

pub fn exponential(input: ExponentialInput) -> String {
    let v = input.value;
    match input.operation.as_str() {
        "exp" => format!("e^{v} = {}", v.exp()),
        "expm1" => format!("e^{v} - 1 = {}", v.exp_m1()),
        "exp2" => format!("2^{v} = {}", v.exp2()),
        "exp10" => format!("10^{v} = {}", (10f64).powf(v)),
        op => format!("Error: Unknown operation '{op}'. Supported: exp, expm1, exp2, exp10"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pr(v: f64, op: &str, exp: Option<f64>) -> String {
        power_root(PowerRootInput {
            operation: op.to_string(),
            value: v,
            exponent: exp,
        })
    }

    fn ex(v: f64, op: &str) -> String {
        exponential(ExponentialInput {
            operation: op.to_string(),
            value: v,
        })
    }

    #[test]
    fn test_square() {
        assert!(pr(4.0, "square", None).contains("16"));
    }
    #[test]
    fn test_sqrt() {
        assert!(pr(9.0, "sqrt", None).contains("3"));
    }
    #[test]
    fn test_sqrt_negative() {
        assert!(pr(-1.0, "sqrt", None).contains("Error"));
    }
    #[test]
    fn test_cbrt() {
        assert!(pr(27.0, "cbrt", None).contains("3"));
    }
    #[test]
    fn test_pow() {
        assert!(pr(2.0, "pow", Some(10.0)).contains("1024"));
    }
    #[test]
    fn test_nth_root() {
        assert!(pr(8.0, "nth_root", Some(3.0)).contains("2"));
    }
    #[test]
    fn test_exp() {
        let r = ex(0.0, "exp");
        assert!(r.contains("1"), "{r}");
    }
    #[test]
    fn test_exp2() {
        assert!(ex(3.0, "exp2").contains("8"));
    }
}
