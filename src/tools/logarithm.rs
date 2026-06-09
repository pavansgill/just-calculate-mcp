use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LogarithmInput {
    #[schemars(description = "Operation: ln, log10, log2, log_base, log1p, logit")]
    pub operation: String,
    #[schemars(description = "Input value")]
    pub value: f64,
    #[schemars(description = "Base for log_base operation")]
    pub base: Option<f64>,
}

pub fn logarithm(input: LogarithmInput) -> String {
    let v = input.value;
    match input.operation.as_str() {
        "ln" => {
            if v <= 0.0 {
                return "Error: ln requires value > 0".to_string();
            }
            format!("ln({v}) = {}", v.ln())
        }
        "log10" => {
            if v <= 0.0 {
                return "Error: log10 requires value > 0".to_string();
            }
            format!("log10({v}) = {}", v.log10())
        }
        "log2" => {
            if v <= 0.0 {
                return "Error: log2 requires value > 0".to_string();
            }
            format!("log2({v}) = {}", v.log2())
        }
        "log_base" => {
            let b = match input.base {
                Some(x) => x,
                None => return "Error: log_base requires base".to_string(),
            };
            if b <= 0.0 || b == 1.0 {
                return "Error: base must be > 0 and ≠ 1".to_string();
            }
            if v <= 0.0 {
                return "Error: value must be > 0".to_string();
            }
            format!("log_{b}({v}) = {}", v.log(b))
        }
        "log1p" => {
            if v <= -1.0 {
                return "Error: log1p requires value > -1".to_string();
            }
            format!("ln(1 + {v}) = {}", v.ln_1p())
        }
        "logit" => {
            if v <= 0.0 || v >= 1.0 {
                return "Error: logit requires 0 < value < 1".to_string();
            }
            let result = (v / (1.0 - v)).ln();
            format!("logit({v}) = {result}")
        }
        op => format!(
            "Error: Unknown operation '{op}'. Supported: ln, log10, log2, log_base, log1p, logit"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::E;

    fn lg(v: f64, op: &str, base: Option<f64>) -> String {
        logarithm(LogarithmInput {
            operation: op.to_string(),
            value: v,
            base,
        })
    }

    #[test]
    fn test_ln() {
        assert!(lg(E, "ln", None).contains("1"));
    }
    #[test]
    fn test_log10() {
        assert!(lg(100.0, "log10", None).contains("2"));
    }
    #[test]
    fn test_log2() {
        assert!(lg(8.0, "log2", None).contains("3"));
    }
    #[test]
    fn test_log_base() {
        assert!(lg(8.0, "log_base", Some(2.0)).contains("3"));
    }
    #[test]
    fn test_log1p() {
        assert!(lg(0.0, "log1p", None).contains("0"));
    }
    #[test]
    fn test_logit_invalid() {
        assert!(lg(1.0, "logit", None).contains("Error"));
    }
    #[test]
    fn test_logit_half() {
        assert!(lg(0.5, "logit", None).contains("0"));
    }
    #[test]
    fn test_ln_negative() {
        assert!(lg(-1.0, "ln", None).contains("Error"));
    }
}
