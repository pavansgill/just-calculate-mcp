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
    let ArithmeticInput { number_a, number_b, operator } = input;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn run(a: f64, b: f64, op: &str) -> String {
        compute(ArithmeticInput { number_a: a, number_b: b, operator: op.to_string() })
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
}
