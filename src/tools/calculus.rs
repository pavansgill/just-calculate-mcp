use schemars::JsonSchema;
use serde::Deserialize;

// ── Polynomial calculus ───────────────────────────────────────────────────────
// Polynomials represented as Vec<f64> where coefficients[i] is coefficient of x^i.
// e.g. [1, 2, 3] = 1 + 2x + 3x²

fn eval_poly(coeffs: &[f64], x: f64) -> f64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(i, &c)| c * x.powi(i as i32))
        .sum()
}

fn differentiate(coeffs: &[f64]) -> Vec<f64> {
    if coeffs.len() <= 1 {
        return vec![0.0];
    }
    coeffs[1..]
        .iter()
        .enumerate()
        .map(|(i, &c)| c * (i as f64 + 1.0))
        .collect()
}

fn integrate_indefinite(coeffs: &[f64]) -> Vec<f64> {
    let mut result = vec![0.0]; // constant of integration = 0
    for (i, &c) in coeffs.iter().enumerate() {
        result.push(c / (i as f64 + 1.0));
    }
    result
}

fn fmt_poly(coeffs: &[f64]) -> String {
    if coeffs.iter().all(|&c| c == 0.0) {
        return "0".to_string();
    }
    let terms: Vec<String> = coeffs
        .iter()
        .enumerate()
        .filter(|(_, &c)| c != 0.0)
        .map(|(i, &c)| {
            if i == 0 {
                format!("{c}")
            } else if i == 1 {
                format!("{c}x")
            } else {
                format!("{c}x^{i}")
            }
        })
        .collect();
    terms.join(" + ")
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PolynomialCalcInput {
    #[schemars(
        description = "Operation: evaluate, differentiate, integrate_indefinite, integrate_definite"
    )]
    pub operation: String,
    #[schemars(
        description = "Polynomial coefficients [c0, c1, c2, ...] where index i = power of x (e.g. [1, 0, 3] = 1 + 3x²)"
    )]
    pub coefficients: Vec<f64>,
    #[schemars(description = "x value for evaluate, or lower bound for integrate_definite")]
    pub x: Option<f64>,
    #[schemars(description = "Upper bound for integrate_definite")]
    pub upper: Option<f64>,
}

pub fn polynomial_calc(input: PolynomialCalcInput) -> String {
    let c = &input.coefficients;
    if c.is_empty() {
        return "Error: coefficients cannot be empty".to_string();
    }

    match input.operation.as_str() {
        "evaluate" => {
            let x = match input.x { Some(v) => v, None => return "Error: x is required".to_string() };
            let result = eval_poly(c, x);
            format!("p({x}) = {result}  (polynomial: {})", fmt_poly(c))
        }
        "differentiate" => {
            let deriv = differentiate(c);
            format!("p'(x) = {}", fmt_poly(&deriv))
        }
        "integrate_indefinite" => {
            let antideriv = integrate_indefinite(c);
            format!("∫p(x)dx = {} + C", fmt_poly(&antideriv))
        }
        "integrate_definite" => {
            let lo = match input.x { Some(v) => v, None => return "Error: x (lower bound) is required".to_string() };
            let hi = match input.upper { Some(v) => v, None => return "Error: upper bound is required".to_string() };
            let antideriv = integrate_indefinite(c);
            let result = eval_poly(&antideriv, hi) - eval_poly(&antideriv, lo);
            format!("∫[{lo}, {hi}] p(x)dx = {result}")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: evaluate, differentiate, integrate_indefinite, integrate_definite"),
    }
}

// ── Numerical methods ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NumericalMethodsInput {
    #[schemars(
        description = "Operation: derivative_at_point, integrate_data, root_bisection_data"
    )]
    pub operation: String,
    #[schemars(
        description = "x values for derivative_at_point (5 equally-spaced points around target) or data x-axis for integrate_data/root_bisection_data"
    )]
    pub x_values: Vec<f64>,
    #[schemars(description = "Corresponding y values (same length as x_values)")]
    pub y_values: Vec<f64>,
}

pub fn numerical_methods(input: NumericalMethodsInput) -> String {
    let x = &input.x_values;
    let y = &input.y_values;
    if x.len() != y.len() {
        return "Error: x_values and y_values must be same length".to_string();
    }
    if x.is_empty() {
        return "Error: values cannot be empty".to_string();
    }

    match input.operation.as_str() {
        "derivative_at_point" => {
            if x.len() < 2 { return "Error: need at least 2 data points for derivative".to_string(); }
            // Simple finite difference at midpoint
            let n = x.len();
            let mid = n / 2;
            if n >= 5 {
                // 5-point stencil
                let h = (x[mid + 1] - x[mid - 1]) / 2.0;
                if h == 0.0 { return "Error: x values must be distinct".to_string(); }
                let d = (-y[mid + 2] + 8.0 * y[mid + 1] - 8.0 * y[mid - 1] + y[mid - 2]) / (12.0 * h);
                format!("dy/dx at x={} ≈ {d} (5-point stencil)", x[mid])
            } else {
                let h = x[mid] - x[mid - 1];
                if h == 0.0 { return "Error: x values must be distinct".to_string(); }
                let d = (y[mid] - y[mid - 1]) / h;
                format!("dy/dx at x={} ≈ {d} (finite difference)", x[mid])
            }
        }
        "integrate_data" => {
            if x.len() < 2 { return "Error: need at least 2 data points".to_string(); }
            let result: f64 = x.windows(2).zip(y.windows(2)).map(|(xi, yi)| {
                (xi[1] - xi[0]) * (yi[0] + yi[1]) / 2.0
            }).sum();
            format!("∫ (trapezoidal rule) = {result}")
        }
        "root_bisection_data" => {
            // Find sign changes in the data
            let sign_changes: Vec<usize> = y.windows(2).enumerate()
                .filter(|(_, w)| w[0] * w[1] < 0.0)
                .map(|(i, _)| i)
                .collect();
            if sign_changes.is_empty() { return "No sign change found in data (no root bracket detected)".to_string(); }
            let brackets: Vec<String> = sign_changes.iter().map(|&i| {
                let root_approx = x[i] - y[i] * (x[i + 1] - x[i]) / (y[i + 1] - y[i]);
                format!("root ≈ {root_approx} (between x={} and x={})", x[i], x[i + 1])
            }).collect();
            brackets.join("; ")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: derivative_at_point, integrate_data, root_bisection_data"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn poly(coeffs: Vec<f64>, op: &str, x: Option<f64>, upper: Option<f64>) -> String {
        polynomial_calc(PolynomialCalcInput {
            operation: op.to_string(),
            coefficients: coeffs,
            x,
            upper,
        })
    }

    #[test]
    fn test_evaluate() {
        // p(x) = 1 + 2x + 3x² evaluated at x=2: 1 + 4 + 12 = 17
        let r = poly(vec![1.0, 2.0, 3.0], "evaluate", Some(2.0), None);
        assert!(r.contains("17"), "{r}");
    }
    #[test]
    fn test_differentiate() {
        // p(x) = 1 + 2x + 3x² → p'(x) = 2 + 6x
        let r = poly(vec![1.0, 2.0, 3.0], "differentiate", None, None);
        assert!(r.contains("2") && r.contains("6"), "{r}");
    }
    #[test]
    fn test_definite_integral() {
        // ∫[0,1] x dx = 0.5
        let r = poly(vec![0.0, 1.0], "integrate_definite", Some(0.0), Some(1.0));
        assert!(r.contains("0.5"), "{r}");
    }
    #[test]
    fn test_trapezoidal() {
        // ∫[0,2] x dx ≈ 2.0 (exact)
        let r = numerical_methods(NumericalMethodsInput {
            operation: "integrate_data".to_string(),
            x_values: vec![0.0, 1.0, 2.0],
            y_values: vec![0.0, 1.0, 2.0],
        });
        assert!(r.contains("2"), "{r}");
    }
}
