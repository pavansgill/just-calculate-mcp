use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SpecialFunctionsInput {
    #[schemars(description = "Operation: gamma, lgamma, beta, erf, erfc, sigmoid, logistic")]
    pub operation: String,
    #[schemars(description = "Primary input value")]
    pub value: f64,
    #[schemars(description = "Second parameter (required for beta: the y in B(x,y))")]
    pub value_b: Option<f64>,
}

pub fn special_functions(input: SpecialFunctionsInput) -> String {
    let v = input.value;
    match input.operation.as_str() {
        "gamma" => {
            if v <= 0.0 && v.fract() == 0.0 {
                return "Error: gamma undefined at zero and negative integers".to_string();
            }
            format!("Γ({v}) = {}", libm::tgamma(v))
        }
        "lgamma" => {
            if v <= 0.0 && v.fract() == 0.0 {
                return "Error: lgamma undefined at zero and negative integers".to_string();
            }
            format!("ln(Γ({v})) = {}", libm::lgamma(v))
        }
        "beta" => {
            let b = match input.value_b {
                Some(x) => x,
                None => return "Error: beta requires value_b".to_string(),
            };
            if v <= 0.0 || b <= 0.0 {
                return "Error: beta requires both arguments > 0".to_string();
            }
            let result = (libm::lgamma(v) + libm::lgamma(b) - libm::lgamma(v + b)).exp();
            format!("B({v}, {b}) = {result}")
        }
        "erf" => format!("erf({v}) = {}", libm::erf(v)),
        "erfc" => format!("erfc({v}) = {}", libm::erfc(v)),
        "sigmoid" => {
            let result = 1.0 / (1.0 + (-v).exp());
            format!("sigmoid({v}) = {result}")
        }
        "logistic" => {
            // logistic is the same as sigmoid
            let result = 1.0 / (1.0 + (-v).exp());
            format!("logistic({v}) = {result}")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: gamma, lgamma, beta, erf, erfc, sigmoid, logistic"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sf(v: f64, op: &str, b: Option<f64>) -> String {
        special_functions(SpecialFunctionsInput {
            operation: op.to_string(),
            value: v,
            value_b: b,
        })
    }

    #[test]
    fn test_gamma_5() {
        // Γ(5) = 4! = 24
        let r = sf(5.0, "gamma", None);
        assert!(r.contains("24"), "{r}");
    }
    #[test]
    fn test_erf_zero() {
        let r = sf(0.0, "erf", None);
        assert!(r.contains("0"), "{r}");
    }
    #[test]
    fn test_sigmoid_zero() {
        let r = sf(0.0, "sigmoid", None);
        assert!(r.contains("0.5"), "{r}");
    }
    #[test]
    fn test_erfc_zero() {
        let r = sf(0.0, "erfc", None);
        assert!(r.contains("1"), "{r}");
    }
    #[test]
    fn test_beta() {
        let r = sf(1.0, "beta", Some(1.0));
        assert!(r.contains("1"), "{r}");
    }
    #[test]
    fn test_gamma_negative_int() {
        assert!(sf(0.0, "gamma", None).contains("Error"));
    }
}
