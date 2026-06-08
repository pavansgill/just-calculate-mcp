use schemars::JsonSchema;
use serde::Deserialize;

fn mean(vals: &[f64]) -> f64 {
    vals.iter().sum::<f64>() / vals.len() as f64
}

fn sorted(vals: &[f64]) -> Vec<f64> {
    let mut v = vals.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v
}

fn median_of(vals: &[f64]) -> f64 {
    let s = sorted(vals);
    let n = s.len();
    if n % 2 == 0 {
        (s[n / 2 - 1] + s[n / 2]) / 2.0
    } else {
        s[n / 2]
    }
}

fn variance(vals: &[f64], population: bool) -> f64 {
    let m = mean(vals);
    let denom = if population { vals.len() as f64 } else { (vals.len() - 1) as f64 };
    vals.iter().map(|x| (x - m).powi(2)).sum::<f64>() / denom
}

fn percentile_of(sorted_vals: &[f64], p: f64) -> f64 {
    let n = sorted_vals.len();
    if n == 1 { return sorted_vals[0]; }
    let idx = p / 100.0 * (n - 1) as f64;
    let lo = idx.floor() as usize;
    let hi = idx.ceil() as usize;
    let frac = idx.fract();
    sorted_vals[lo] * (1.0 - frac) + sorted_vals[hi] * frac
}

// ── Descriptive stats ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DescriptiveStatsInput {
    #[schemars(description = "Operation: mean, median, mode, geometric_mean, harmonic_mean, rms, variance_pop, variance_sample, std_dev_pop, std_dev_sample, range, iqr, mad, skewness, kurtosis, sum, min, max, count, midrange, coefficient_of_variation")]
    pub operation: String,
    #[schemars(description = "The list of numbers")]
    pub values: Vec<f64>,
}

pub fn descriptive_stats(input: DescriptiveStatsInput) -> String {
    let vals = &input.values;
    if vals.is_empty() {
        return "Error: values list is empty".to_string();
    }
    if vals.len() < 2 && matches!(input.operation.as_str(), "variance_sample" | "std_dev_sample") {
        return "Error: sample variance/std_dev requires at least 2 values".to_string();
    }

    match input.operation.as_str() {
        "mean" => format!("mean = {}", mean(vals)),
        "median" => format!("median = {}", median_of(vals)),
        "mode" => {
            let s = sorted(vals);
            let mut best = s[0];
            let mut best_count = 1usize;
            let mut cur = s[0];
            let mut cur_count = 1usize;
            for &x in &s[1..] {
                if (x - cur).abs() < 1e-12 {
                    cur_count += 1;
                } else {
                    if cur_count > best_count { best = cur; best_count = cur_count; }
                    cur = x; cur_count = 1;
                }
            }
            if cur_count > best_count { best = cur; }
            format!("mode = {best}")
        }
        "geometric_mean" => {
            if vals.iter().any(|&x| x <= 0.0) {
                return "Error: geometric_mean requires all values > 0".to_string();
            }
            let result = (vals.iter().map(|x| x.ln()).sum::<f64>() / vals.len() as f64).exp();
            format!("geometric_mean = {result}")
        }
        "harmonic_mean" => {
            if vals.iter().any(|&x| x == 0.0) {
                return "Error: harmonic_mean requires all values ≠ 0".to_string();
            }
            let result = vals.len() as f64 / vals.iter().map(|x| 1.0 / x).sum::<f64>();
            format!("harmonic_mean = {result}")
        }
        "rms" => {
            let result = (vals.iter().map(|x| x * x).sum::<f64>() / vals.len() as f64).sqrt();
            format!("rms = {result}")
        }
        "variance_pop" => format!("variance_pop = {}", variance(vals, true)),
        "variance_sample" => format!("variance_sample = {}", variance(vals, false)),
        "std_dev_pop" => format!("std_dev_pop = {}", variance(vals, true).sqrt()),
        "std_dev_sample" => format!("std_dev_sample = {}", variance(vals, false).sqrt()),
        "range" => {
            let s = sorted(vals);
            format!("range = {}", s.last().unwrap() - s.first().unwrap())
        }
        "iqr" => {
            let s = sorted(vals);
            let q1 = percentile_of(&s, 25.0);
            let q3 = percentile_of(&s, 75.0);
            format!("IQR = {} (Q1={q1}, Q3={q3})", q3 - q1)
        }
        "mad" => {
            let m = median_of(vals);
            let deviations: Vec<f64> = vals.iter().map(|x| (x - m).abs()).collect();
            format!("MAD = {}", median_of(&deviations))
        }
        "skewness" => {
            if vals.len() < 3 { return "Error: skewness requires at least 3 values".to_string(); }
            let m = mean(vals);
            let s = variance(vals, false).sqrt();
            if s == 0.0 { return "skewness = 0 (all values equal)".to_string(); }
            let n = vals.len() as f64;
            let result = vals.iter().map(|x| ((x - m) / s).powi(3)).sum::<f64>()
                * (n / ((n - 1.0) * (n - 2.0)));
            format!("skewness = {result}")
        }
        "kurtosis" => {
            if vals.len() < 4 { return "Error: kurtosis requires at least 4 values".to_string(); }
            let m = mean(vals);
            let s = variance(vals, false).sqrt();
            if s == 0.0 { return "kurtosis = 0 (all values equal)".to_string(); }
            let n = vals.len() as f64;
            let result = vals.iter().map(|x| ((x - m) / s).powi(4)).sum::<f64>() / n - 3.0;
            format!("excess kurtosis = {result}")
        }
        "sum" => format!("sum = {}", vals.iter().sum::<f64>()),
        "min" => format!("min = {}", vals.iter().cloned().fold(f64::INFINITY, f64::min)),
        "max" => format!("max = {}", vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
        "count" => format!("count = {}", vals.len()),
        "midrange" => {
            let s = sorted(vals);
            format!("midrange = {}", (s.first().unwrap() + s.last().unwrap()) / 2.0)
        }
        "coefficient_of_variation" => {
            let s = variance(vals, false).sqrt();
            let m = mean(vals);
            if m == 0.0 { return "Error: coefficient_of_variation undefined when mean = 0".to_string(); }
            format!("CV = {}%", (s / m) * 100.0)
        }
        op => format!("Error: Unknown operation '{op}'"),
    }
}

// ── Percentile ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PercentileInput {
    #[schemars(description = "Operation: percentile, quartile, zscore")]
    pub operation: String,
    #[schemars(description = "The list of numbers")]
    pub values: Vec<f64>,
    #[schemars(description = "p for percentile (0-100), quartile number 1/2/3, or x value for zscore")]
    pub parameter: f64,
}

pub fn percentile(input: PercentileInput) -> String {
    let vals = &input.values;
    if vals.is_empty() { return "Error: values list is empty".to_string(); }
    let s = sorted(vals);

    match input.operation.as_str() {
        "percentile" => {
            let p = input.parameter;
            if p < 0.0 || p > 100.0 { return "Error: p must be 0-100".to_string(); }
            format!("P{p} = {}", percentile_of(&s, p))
        }
        "quartile" => {
            let q = input.parameter as u32;
            let p = match q { 1 => 25.0, 2 => 50.0, 3 => 75.0, _ => return "Error: quartile must be 1, 2, or 3".to_string() };
            format!("Q{q} = {}", percentile_of(&s, p))
        }
        "zscore" => {
            if vals.len() < 2 { return "Error: zscore requires at least 2 values".to_string(); }
            let m = mean(vals);
            let sd = variance(vals, false).sqrt();
            if sd == 0.0 { return "Error: std_dev is 0, zscore undefined".to_string(); }
            format!("z-score({}) = {}", input.parameter, (input.parameter - m) / sd)
        }
        op => format!("Error: Unknown operation '{op}'. Supported: percentile, quartile, zscore"),
    }
}

// ── Correlation ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CorrelationInput {
    #[schemars(description = "Operation: pearson, spearman, covariance")]
    pub operation: String,
    #[schemars(description = "First list of values")]
    pub x_values: Vec<f64>,
    #[schemars(description = "Second list of values (same length as x_values)")]
    pub y_values: Vec<f64>,
}

fn rank_vector(vals: &[f64]) -> Vec<f64> {
    let n = vals.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| vals[a].partial_cmp(&vals[b]).unwrap());
    let mut ranks = vec![0.0f64; n];
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && (vals[idx[j]] - vals[idx[i]]).abs() < 1e-12 { j += 1; }
        let avg_rank = (i + j - 1) as f64 / 2.0 + 1.0;
        for k in i..j { ranks[idx[k]] = avg_rank; }
        i = j;
    }
    ranks
}

pub fn correlation(input: CorrelationInput) -> String {
    let x = &input.x_values;
    let y = &input.y_values;
    if x.len() != y.len() { return "Error: x_values and y_values must be same length".to_string(); }
    if x.len() < 2 { return "Error: need at least 2 data points".to_string(); }

    let n = x.len() as f64;
    let mx = mean(x);
    let my = mean(y);

    match input.operation.as_str() {
        "pearson" => {
            let num: f64 = x.iter().zip(y).map(|(xi, yi)| (xi - mx) * (yi - my)).sum();
            let dx: f64 = x.iter().map(|xi| (xi - mx).powi(2)).sum::<f64>().sqrt();
            let dy: f64 = y.iter().map(|yi| (yi - my).powi(2)).sum::<f64>().sqrt();
            if dx == 0.0 || dy == 0.0 { return "Error: variance is 0, correlation undefined".to_string(); }
            format!("Pearson r = {}", num / (dx * dy))
        }
        "spearman" => {
            let rx = rank_vector(x);
            let ry = rank_vector(y);
            let mrx = mean(&rx);
            let mry = mean(&ry);
            let num: f64 = rx.iter().zip(&ry).map(|(ri, rj)| (ri - mrx) * (rj - mry)).sum();
            let dx: f64 = rx.iter().map(|r| (r - mrx).powi(2)).sum::<f64>().sqrt();
            let dy: f64 = ry.iter().map(|r| (r - mry).powi(2)).sum::<f64>().sqrt();
            if dx == 0.0 || dy == 0.0 { return "Error: rank variance is 0".to_string(); }
            format!("Spearman rho = {}", num / (dx * dy))
        }
        "covariance" => {
            let cov: f64 = x.iter().zip(y).map(|(xi, yi)| (xi - mx) * (yi - my)).sum::<f64>() / (n - 1.0);
            format!("covariance = {cov}")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: pearson, spearman, covariance"),
    }
}

// ── Linear regression ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LinearRegressionInput {
    #[schemars(description = "x values (independent variable)")]
    pub x_values: Vec<f64>,
    #[schemars(description = "y values (dependent variable, same length as x_values)")]
    pub y_values: Vec<f64>,
    #[schemars(description = "Optional x value to predict y for")]
    pub predict_x: Option<f64>,
}

pub fn linear_regression(input: LinearRegressionInput) -> String {
    let x = &input.x_values;
    let y = &input.y_values;
    if x.len() != y.len() { return "Error: x_values and y_values must be same length".to_string(); }
    if x.len() < 2 { return "Error: need at least 2 data points".to_string(); }

    let mx = mean(x);
    let my = mean(y);
    let ss_xx: f64 = x.iter().map(|xi| (xi - mx).powi(2)).sum();
    if ss_xx == 0.0 { return "Error: all x values are identical".to_string(); }
    let ss_xy: f64 = x.iter().zip(y).map(|(xi, yi)| (xi - mx) * (yi - my)).sum();
    let slope = ss_xy / ss_xx;
    let intercept = my - slope * mx;

    let ss_res: f64 = x.iter().zip(y).map(|(xi, yi)| (yi - (slope * xi + intercept)).powi(2)).sum();
    let ss_tot: f64 = y.iter().map(|yi| (yi - my).powi(2)).sum();
    let r2 = if ss_tot == 0.0 { 1.0 } else { 1.0 - ss_res / ss_tot };

    let mut result = format!("slope = {slope}, intercept = {intercept}, R² = {r2}\ny = {slope}x + {intercept}");
    if let Some(px) = input.predict_x {
        result.push_str(&format!("\npredict(x={px}) = {}", slope * px + intercept));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ds(vals: &[f64], op: &str) -> String {
        descriptive_stats(DescriptiveStatsInput { operation: op.to_string(), values: vals.to_vec() })
    }

    #[test]
    fn test_mean() { assert!(ds(&[1.0, 2.0, 3.0], "mean").contains("2")); }
    #[test]
    fn test_median_even() { assert!(ds(&[1.0, 2.0, 3.0, 4.0], "median").contains("2.5")); }
    #[test]
    fn test_sum() { assert!(ds(&[1.0, 2.0, 3.0], "sum").contains("6")); }
    #[test]
    fn test_empty() { assert!(ds(&[], "mean").contains("Error")); }
    #[test]
    fn test_std_dev_pop() {
        let r = ds(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0], "std_dev_pop");
        assert!(r.contains("2"), "{r}");
    }
    #[test]
    fn test_pearson_perfect() {
        let r = correlation(CorrelationInput {
            operation: "pearson".to_string(),
            x_values: vec![1.0, 2.0, 3.0],
            y_values: vec![2.0, 4.0, 6.0],
        });
        // Perfect correlation — result should be within floating-point epsilon of 1
        let val: f64 = r.split('=').last().unwrap().trim().parse().unwrap();
        assert!((val - 1.0).abs() < 1e-10, "expected ~1, got {r}");
    }
    #[test]
    fn test_linear_regression() {
        let r = linear_regression(LinearRegressionInput {
            x_values: vec![1.0, 2.0, 3.0],
            y_values: vec![2.0, 4.0, 6.0],
            predict_x: Some(4.0),
        });
        assert!(r.contains("slope = 2") || r.contains("slope=2"), "{r}");
    }
}
