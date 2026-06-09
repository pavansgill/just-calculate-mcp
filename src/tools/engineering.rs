use schemars::JsonSchema;
use serde::Deserialize;

// ── Complex numbers ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ComplexNumberInput {
    #[schemars(
        description = "Operation: add, subtract, multiply, divide, conjugate, magnitude, phase_rad, phase_deg, polar_to_rect, rect_to_polar"
    )]
    pub operation: String,
    #[schemars(description = "Real part of first complex number (or magnitude for polar_to_rect)")]
    pub re1: f64,
    #[schemars(
        description = "Imaginary part of first complex number (or angle in degrees for polar_to_rect)"
    )]
    pub im1: f64,
    #[schemars(
        description = "Real part of second complex number (required for add, subtract, multiply, divide)"
    )]
    pub re2: Option<f64>,
    #[schemars(
        description = "Imaginary part of second complex number (required for add, subtract, multiply, divide)"
    )]
    pub im2: Option<f64>,
}

pub fn complex_number(input: ComplexNumberInput) -> String {
    let (a, b) = (input.re1, input.im1);
    let c = input.re2.unwrap_or(0.0);
    let d = input.im2.unwrap_or(0.0);

    let fmt = |re: f64, im: f64| -> String {
        if im >= 0.0 {
            format!("{re} + {im}i")
        } else {
            format!("{re} - {}i", im.abs())
        }
    };

    match input.operation.as_str() {
        "add" => { let r = fmt(a + c, b + d); format!("({} + {}i) + ({} + {}i) = {r}", a, b, c, d) }
        "subtract" => { let r = fmt(a - c, b - d); format!("({} + {}i) - ({} + {}i) = {r}", a, b, c, d) }
        "multiply" => {
            let re = a * c - b * d;
            let im = a * d + b * c;
            format!("({} + {}i) * ({} + {}i) = {}", a, b, c, d, fmt(re, im))
        }
        "divide" => {
            let denom = c * c + d * d;
            if denom == 0.0 { return "Error: division by zero (second complex number is 0)".to_string(); }
            let re = (a * c + b * d) / denom;
            let im = (b * c - a * d) / denom;
            format!("({} + {}i) / ({} + {}i) = {}", a, b, c, d, fmt(re, im))
        }
        "conjugate" => format!("conj({}) = {}", fmt(a, b), fmt(a, -b)),
        "magnitude" => format!("|{a} + {b}i| = {}", (a * a + b * b).sqrt()),
        "phase_rad" => format!("phase({a} + {b}i) = {} radians", b.atan2(a)),
        "phase_deg" => format!("phase({a} + {b}i) = {}°", b.atan2(a).to_degrees()),
        "polar_to_rect" => {
            let angle_rad = b.to_radians();
            let re = a * angle_rad.cos();
            let im = a * angle_rad.sin();
            format!("r={a}, θ={b}° → {}", fmt(re, im))
        }
        "rect_to_polar" => {
            let r = (a * a + b * b).sqrt();
            let theta = b.atan2(a).to_degrees();
            format!("{a} + {b}i → r={r}, θ={theta}°")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: add, subtract, multiply, divide, conjugate, magnitude, phase_rad, phase_deg, polar_to_rect, rect_to_polar"),
    }
}

// ── Electrical ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ElectricalInput {
    #[schemars(description = "Operation: ohms_law, power_law")]
    pub operation: String,
    #[schemars(description = "Voltage (V). Leave None to solve for it.")]
    pub voltage: Option<f64>,
    #[schemars(description = "Current in amps (I). Leave None to solve for it.")]
    pub current: Option<f64>,
    #[schemars(description = "Resistance in ohms (R). Leave None to solve for it.")]
    pub resistance: Option<f64>,
    #[schemars(description = "Power in watts (P). Leave None to solve for it. (power_law only)")]
    pub power: Option<f64>,
}

pub fn electrical(input: ElectricalInput) -> String {
    match input.operation.as_str() {
        "ohms_law" => match (input.voltage, input.current, input.resistance) {
            (None, Some(i), Some(r)) => format!("V = I × R = {i} × {r} = {}", i * r),
            (Some(v), None, Some(r)) => {
                if r == 0.0 {
                    return "Error: resistance cannot be zero".to_string();
                }
                format!("I = V / R = {v} / {r} = {}", v / r)
            }
            (Some(v), Some(i), None) => {
                if i == 0.0 {
                    return "Error: current cannot be zero".to_string();
                }
                format!("R = V / I = {v} / {i} = {}", v / i)
            }
            _ => "Error: provide exactly two of voltage, current, resistance".to_string(),
        },
        "power_law" => match (input.power, input.voltage, input.current, input.resistance) {
            (None, Some(v), Some(i), None) => format!("P = V × I = {v} × {i} = {}", v * i),
            (None, Some(v), None, Some(r)) => {
                if r == 0.0 {
                    return "Error: resistance cannot be zero".to_string();
                }
                format!("P = V² / R = {v}² / {r} = {}", v * v / r)
            }
            (None, None, Some(i), Some(r)) => format!("P = I² × R = {i}² × {r} = {}", i * i * r),
            (Some(p), None, Some(i), None) => {
                if i == 0.0 {
                    return "Error: current cannot be zero".to_string();
                }
                format!("V = P / I = {p} / {i} = {}", p / i)
            }
            (Some(p), Some(v), None, None) => {
                if v == 0.0 {
                    return "Error: voltage cannot be zero".to_string();
                }
                format!("I = P / V = {p} / {v} = {}", p / v)
            }
            _ => "Error: provide exactly two known values (power, voltage, current, or resistance)"
                .to_string(),
        },
        op => format!("Error: Unknown operation '{op}'. Supported: ohms_law, power_law"),
    }
}

// ── Decibels ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DecibelInput {
    #[schemars(
        description = "Operation: power_ratio_to_db, db_to_power_ratio, voltage_ratio_to_db, db_to_voltage_ratio"
    )]
    pub operation: String,
    #[schemars(description = "Input value (ratio or dB depending on operation)")]
    pub value: f64,
}

pub fn decibel(input: DecibelInput) -> String {
    let v = input.value;
    match input.operation.as_str() {
        "power_ratio_to_db" => {
            if v <= 0.0 { return "Error: power ratio must be > 0".to_string(); }
            format!("{v} (power ratio) = {} dB", 10.0 * v.log10())
        }
        "db_to_power_ratio" => format!("{v} dB = {} (power ratio)", 10f64.powf(v / 10.0)),
        "voltage_ratio_to_db" => {
            if v <= 0.0 { return "Error: voltage ratio must be > 0".to_string(); }
            format!("{v} (voltage ratio) = {} dB", 20.0 * v.log10())
        }
        "db_to_voltage_ratio" => format!("{v} dB = {} (voltage ratio)", 10f64.powf(v / 20.0)),
        op => format!("Error: Unknown operation '{op}'. Supported: power_ratio_to_db, db_to_power_ratio, voltage_ratio_to_db, db_to_voltage_ratio"),
    }
}

// ── Interpolation ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InterpolationInput {
    #[schemars(description = "Operation: linear_interpolate, linear_extrapolate, lerp")]
    pub operation: String,
    #[schemars(description = "x1 (known point 1 x) or start value (lerp)")]
    pub x1: f64,
    #[schemars(description = "y1 (known point 1 y) or end value (lerp)")]
    pub y1: f64,
    #[schemars(description = "x2 (known point 2 x). Not used for lerp.")]
    pub x2: Option<f64>,
    #[schemars(description = "y2 (known point 2 y). Not used for lerp.")]
    pub y2: Option<f64>,
    #[schemars(description = "x to interpolate/extrapolate at, or t parameter [0,1] for lerp")]
    pub x: f64,
}

pub fn interpolation(input: InterpolationInput) -> String {
    match input.operation.as_str() {
        "linear_interpolate" | "linear_extrapolate" => {
            let x2 = match input.x2 { Some(v) => v, None => return "Error: x2 is required".to_string() };
            let y2 = match input.y2 { Some(v) => v, None => return "Error: y2 is required".to_string() };
            let (x1, y1) = (input.x1, input.y1);
            if (x2 - x1).abs() < 1e-15 { return "Error: x1 and x2 must be different".to_string(); }
            let result = y1 + (input.x - x1) * (y2 - y1) / (x2 - x1);
            let kind = if input.x >= x1.min(x2) && input.x <= x1.max(x2) { "interpolation" } else { "extrapolation" };
            format!("{kind} at x={} = {result}", input.x)
        }
        "lerp" => {
            let t = input.x;
            let result = input.x1 + t * (input.y1 - input.x1);
            format!("lerp({}, {}, t={t}) = {result}", input.x1, input.y1)
        }
        op => format!("Error: Unknown operation '{op}'. Supported: linear_interpolate, linear_extrapolate, lerp"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_multiply() {
        let r = complex_number(ComplexNumberInput {
            operation: "multiply".to_string(),
            re1: 1.0,
            im1: 2.0,
            re2: Some(3.0),
            im2: Some(4.0),
        });
        assert!(r.contains("-5") || r.contains("-5.0"), "{r}");
    }
    #[test]
    fn test_complex_magnitude() {
        let r = complex_number(ComplexNumberInput {
            operation: "magnitude".to_string(),
            re1: 3.0,
            im1: 4.0,
            re2: None,
            im2: None,
        });
        assert!(r.contains("5"), "{r}");
    }
    #[test]
    fn test_ohms_law_voltage() {
        let r = electrical(ElectricalInput {
            operation: "ohms_law".to_string(),
            voltage: None,
            current: Some(2.0),
            resistance: Some(5.0),
            power: None,
        });
        assert!(r.contains("10"), "{r}");
    }
    #[test]
    fn test_power_ratio_to_db() {
        let r = decibel(DecibelInput {
            operation: "power_ratio_to_db".to_string(),
            value: 100.0,
        });
        assert!(r.contains("20"), "{r}");
    }
    #[test]
    fn test_lerp() {
        let r = interpolation(InterpolationInput {
            operation: "lerp".to_string(),
            x1: 0.0,
            y1: 10.0,
            x2: None,
            y2: None,
            x: 0.5,
        });
        assert!(r.contains("5"), "{r}");
    }
}
