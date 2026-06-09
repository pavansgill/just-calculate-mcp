use schemars::JsonSchema;
use serde::Deserialize;
use statrs::distribution::{
    Binomial, Continuous, ContinuousCDF, Discrete, DiscreteCDF, Exp, Geometric, Normal, Poisson,
    Uniform,
};

// ── Distribution ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DistributionInput {
    #[schemars(description = "Query type: pdf (or pmf), cdf, inverse_cdf")]
    pub query: String,
    #[schemars(
        description = "Distribution name: normal, binomial, poisson, exponential, uniform, geometric"
    )]
    pub distribution: String,
    #[schemars(description = "x value for pdf/cdf, or probability p for inverse_cdf")]
    pub x: f64,
    #[schemars(
        description = "Parameter 1: mean (normal), n trials (binomial), lambda (poisson/exponential), lower bound (uniform), p success (geometric)"
    )]
    pub param1: f64,
    #[schemars(
        description = "Parameter 2: std_dev (normal), p success (binomial), upper bound (uniform)"
    )]
    pub param2: Option<f64>,
}

pub fn distribution(input: DistributionInput) -> String {
    let x = input.x;
    let p1 = input.param1;
    let p2 = input.param2.unwrap_or(1.0);

    match input.distribution.as_str() {
        "normal" => {
            let dist = match Normal::new(p1, p2) {
                Ok(d) => d,
                Err(e) => return format!("Error: {e}"),
            };
            match input.query.as_str() {
                "pdf" => format!("Normal({p1}, {p2}) pdf({x}) = {}", dist.pdf(x)),
                "cdf" => format!("Normal({p1}, {p2}) cdf({x}) = {}", dist.cdf(x)),
                "inverse_cdf" => {
                    if x <= 0.0 || x >= 1.0 { return "Error: inverse_cdf requires 0 < x < 1".to_string(); }
                    format!("Normal({p1}, {p2}) inverse_cdf({x}) = {}", dist.inverse_cdf(x))
                }
                q => format!("Error: Unknown query '{q}'. Supported: pdf, cdf, inverse_cdf"),
            }
        }
        "binomial" => {
            let n = p1 as u64;
            let dist = match Binomial::new(p2, n) {
                Ok(d) => d,
                Err(e) => return format!("Error: {e}"),
            };
            let xi = x as u64;
            match input.query.as_str() {
                "pmf" | "pdf" => format!("Binomial(n={n}, p={p2}) pmf({xi}) = {}", dist.pmf(xi)),
                "cdf" => format!("Binomial(n={n}, p={p2}) cdf({xi}) = {}", dist.cdf(xi)),
                "inverse_cdf" => {
                    if x <= 0.0 || x >= 1.0 { return "Error: inverse_cdf requires 0 < x < 1".to_string(); }
                    format!("Binomial(n={n}, p={p2}) inverse_cdf({x}) = {}", dist.inverse_cdf(x))
                }
                q => format!("Error: Unknown query '{q}'. Supported: pmf, cdf, inverse_cdf"),
            }
        }
        "poisson" => {
            let dist = match Poisson::new(p1) {
                Ok(d) => d,
                Err(e) => return format!("Error: {e}"),
            };
            let xi = x as u64;
            match input.query.as_str() {
                "pmf" | "pdf" => format!("Poisson(λ={p1}) pmf({xi}) = {}", dist.pmf(xi)),
                "cdf" => format!("Poisson(λ={p1}) cdf({xi}) = {}", dist.cdf(xi)),
                "inverse_cdf" => {
                    if x <= 0.0 || x >= 1.0 { return "Error: inverse_cdf requires 0 < x < 1".to_string(); }
                    format!("Poisson(λ={p1}) inverse_cdf({x}) = {}", dist.inverse_cdf(x))
                }
                q => format!("Error: Unknown query '{q}'. Supported: pmf, cdf, inverse_cdf"),
            }
        }
        "exponential" => {
            let dist = match Exp::new(p1) {
                Ok(d) => d,
                Err(e) => return format!("Error: {e}"),
            };
            match input.query.as_str() {
                "pdf" => format!("Exponential(λ={p1}) pdf({x}) = {}", dist.pdf(x)),
                "cdf" => format!("Exponential(λ={p1}) cdf({x}) = {}", dist.cdf(x)),
                "inverse_cdf" => {
                    if x <= 0.0 || x >= 1.0 { return "Error: inverse_cdf requires 0 < x < 1".to_string(); }
                    format!("Exponential(λ={p1}) inverse_cdf({x}) = {}", dist.inverse_cdf(x))
                }
                q => format!("Error: Unknown query '{q}'. Supported: pdf, cdf, inverse_cdf"),
            }
        }
        "uniform" => {
            let dist = match Uniform::new(p1, p2) {
                Ok(d) => d,
                Err(e) => return format!("Error: {e}"),
            };
            match input.query.as_str() {
                "pdf" => format!("Uniform({p1}, {p2}) pdf({x}) = {}", dist.pdf(x)),
                "cdf" => format!("Uniform({p1}, {p2}) cdf({x}) = {}", dist.cdf(x)),
                "inverse_cdf" => {
                    if x < 0.0 || x > 1.0 { return "Error: inverse_cdf requires 0 <= x <= 1".to_string(); }
                    format!("Uniform({p1}, {p2}) inverse_cdf({x}) = {}", dist.inverse_cdf(x))
                }
                q => format!("Error: Unknown query '{q}'. Supported: pdf, cdf, inverse_cdf"),
            }
        }
        "geometric" => {
            let dist = match Geometric::new(p1) {
                Ok(d) => d,
                Err(e) => return format!("Error: {e}"),
            };
            let xi = x as u64;
            match input.query.as_str() {
                "pmf" | "pdf" => format!("Geometric(p={p1}) pmf({xi}) = {}", dist.pmf(xi)),
                "cdf" => format!("Geometric(p={p1}) cdf({xi}) = {}", dist.cdf(xi)),
                "inverse_cdf" => {
                    if x <= 0.0 || x >= 1.0 { return "Error: inverse_cdf requires 0 < x < 1".to_string(); }
                    format!("Geometric(p={p1}) inverse_cdf({x}) = {}", dist.inverse_cdf(x))
                }
                q => format!("Error: Unknown query '{q}'. Supported: pmf, cdf, inverse_cdf"),
            }
        }
        d => format!("Error: Unknown distribution '{d}'. Supported: normal, binomial, poisson, exponential, uniform, geometric"),
    }
}

// ── Odds conversion ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OddsConvertInput {
    #[schemars(
        description = "Operation: probability_to_odds, odds_to_probability, fractional_to_decimal, decimal_to_fractional"
    )]
    pub operation: String,
    #[schemars(
        description = "Probability (0-1) for probability_to_odds; odds ratio for odds_to_probability; numerator/decimal odds for fractional_to_decimal/decimal_to_fractional"
    )]
    pub value: f64,
    #[schemars(description = "Denominator for fractional_to_decimal (e.g. 3 in '5/3' odds)")]
    pub denominator: Option<f64>,
}

pub fn odds_convert(input: OddsConvertInput) -> String {
    let v = input.value;
    match input.operation.as_str() {
        "probability_to_odds" => {
            if v <= 0.0 || v >= 1.0 { return "Error: probability must be between 0 and 1 exclusive".to_string(); }
            format!("P={v} → odds = {:.4}:{:.4} (for:against)", v, 1.0 - v)
        }
        "odds_to_probability" => {
            if v <= 0.0 { return "Error: odds must be > 0".to_string(); }
            format!("odds={v} → P = {}", v / (1.0 + v))
        }
        "fractional_to_decimal" => {
            let d = input.denominator.unwrap_or(1.0);
            format!("{v}/{d} (fractional) = {} (decimal)", v / d + 1.0)
        }
        "decimal_to_fractional" => {
            if v <= 1.0 { return "Error: decimal odds must be > 1".to_string(); }
            let frac = v - 1.0;
            format!("{v} (decimal) ≈ {frac}/1 (fractional)")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: probability_to_odds, odds_to_probability, fractional_to_decimal, decimal_to_fractional"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_pdf_mean() {
        let r = distribution(DistributionInput {
            query: "pdf".to_string(),
            distribution: "normal".to_string(),
            x: 0.0,
            param1: 0.0,
            param2: Some(1.0),
        });
        assert!(r.contains("0.398") || r.contains("0.39"), "{r}");
    }
    #[test]
    fn test_normal_cdf_half() {
        let r = distribution(DistributionInput {
            query: "cdf".to_string(),
            distribution: "normal".to_string(),
            x: 0.0,
            param1: 0.0,
            param2: Some(1.0),
        });
        assert!(r.contains("0.5"), "{r}");
    }
    #[test]
    fn test_uniform_cdf() {
        let r = distribution(DistributionInput {
            query: "cdf".to_string(),
            distribution: "uniform".to_string(),
            x: 5.0,
            param1: 0.0,
            param2: Some(10.0),
        });
        assert!(r.contains("0.5"), "{r}");
    }
    #[test]
    fn test_odds_probability() {
        let r = odds_convert(OddsConvertInput {
            operation: "probability_to_odds".to_string(),
            value: 0.5,
            denominator: None,
        });
        assert!(r.contains("0.5"), "{r}");
    }
}
