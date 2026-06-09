use crate::tools::shared::{deg_to_rad, rad_to_deg};
use schemars::JsonSchema;
use serde::Deserialize;
use std::f64::consts::PI;

// ── Primary / reciprocal / inverse trig ───────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TrigInput {
    #[schemars(
        description = "Operation: sin, cos, tan, sec, csc, cot, asin, acos, atan, asec, acsc, acot"
    )]
    pub operation: String,
    #[schemars(description = "Input angle (for forward ops) or ratio (for inverse ops)")]
    pub value: f64,
    #[schemars(
        description = "Angle unit for forward operations: degrees or radians (default radians)"
    )]
    pub angle_unit: Option<String>,
}

pub fn trigonometry(input: TrigInput) -> String {
    let v = input.value;
    let unit = input.angle_unit.as_deref().unwrap_or("radians");
    let angle_rad = if unit == "degrees" { deg_to_rad(v) } else { v };

    match input.operation.as_str() {
        "sin" => format!("sin({v} {unit}) = {}", angle_rad.sin()),
        "cos" => format!("cos({v} {unit}) = {}", angle_rad.cos()),
        "tan" => format!("tan({v} {unit}) = {}", angle_rad.tan()),
        "sec" => {
            let c = angle_rad.cos();
            if c.abs() < 1e-15 { return "Error: sec undefined (cos = 0)".to_string(); }
            format!("sec({v} {unit}) = {}", 1.0 / c)
        }
        "csc" => {
            let s = angle_rad.sin();
            if s.abs() < 1e-15 { return "Error: csc undefined (sin = 0)".to_string(); }
            format!("csc({v} {unit}) = {}", 1.0 / s)
        }
        "cot" => {
            let s = angle_rad.sin();
            if s.abs() < 1e-15 { return "Error: cot undefined (sin = 0)".to_string(); }
            format!("cot({v} {unit}) = {}", angle_rad.cos() / s)
        }
        "asin" => {
            if v < -1.0 || v > 1.0 { return "Error: asin domain is [-1, 1]".to_string(); }
            let r = v.asin();
            format!("asin({v}) = {} radians = {} degrees", r, rad_to_deg(r))
        }
        "acos" => {
            if v < -1.0 || v > 1.0 { return "Error: acos domain is [-1, 1]".to_string(); }
            let r = v.acos();
            format!("acos({v}) = {} radians = {} degrees", r, rad_to_deg(r))
        }
        "atan" => {
            let r = v.atan();
            format!("atan({v}) = {} radians = {} degrees", r, rad_to_deg(r))
        }
        "asec" => {
            if v.abs() < 1.0 { return "Error: asec domain is |x| >= 1".to_string(); }
            let r = (1.0 / v).acos();
            format!("asec({v}) = {} radians = {} degrees", r, rad_to_deg(r))
        }
        "acsc" => {
            if v.abs() < 1.0 { return "Error: acsc domain is |x| >= 1".to_string(); }
            let r = (1.0 / v).asin();
            format!("acsc({v}) = {} radians = {} degrees", r, rad_to_deg(r))
        }
        "acot" => {
            let r = (1.0 / v).atan();
            format!("acot({v}) = {} radians = {} degrees", r, rad_to_deg(r))
        }
        op => format!("Error: Unknown operation '{op}'. Supported: sin, cos, tan, sec, csc, cot, asin, acos, atan, asec, acsc, acot"),
    }
}

// ── Two-argument trig ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Trig2ArgInput {
    #[schemars(description = "Operation: atan2, hypot")]
    pub operation: String,
    #[schemars(description = "y (for atan2) or first value (for hypot)")]
    pub y: f64,
    #[schemars(description = "x (for atan2) or second value (for hypot)")]
    pub x: f64,
}

pub fn trigonometry_2arg(input: Trig2ArgInput) -> String {
    match input.operation.as_str() {
        "atan2" => {
            let r = input.y.atan2(input.x);
            format!(
                "atan2({}, {}) = {} radians = {} degrees",
                input.y,
                input.x,
                r,
                rad_to_deg(r)
            )
        }
        "hypot" => format!(
            "hypot({}, {}) = {}",
            input.y,
            input.x,
            input.y.hypot(input.x)
        ),
        op => format!("Error: Unknown operation '{op}'. Supported: atan2, hypot"),
    }
}

// ── Hyperbolic ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct HyperbolicInput {
    #[schemars(
        description = "Operation: sinh, cosh, tanh, sech, csch, coth, asinh, acosh, atanh, asech, acsch, acoth"
    )]
    pub operation: String,
    #[schemars(description = "Input value")]
    pub value: f64,
}

pub fn hyperbolic(input: HyperbolicInput) -> String {
    let v = input.value;
    match input.operation.as_str() {
        "sinh" => format!("sinh({v}) = {}", v.sinh()),
        "cosh" => format!("cosh({v}) = {}", v.cosh()),
        "tanh" => format!("tanh({v}) = {}", v.tanh()),
        "sech" => format!("sech({v}) = {}", 1.0 / v.cosh()),
        "csch" => {
            if v == 0.0 { return "Error: csch(0) is undefined".to_string(); }
            format!("csch({v}) = {}", 1.0 / v.sinh())
        }
        "coth" => {
            if v == 0.0 { return "Error: coth(0) is undefined".to_string(); }
            format!("coth({v}) = {}", v.cosh() / v.sinh())
        }
        "asinh" => format!("asinh({v}) = {}", v.asinh()),
        "acosh" => {
            if v < 1.0 { return "Error: acosh domain is x >= 1".to_string(); }
            format!("acosh({v}) = {}", v.acosh())
        }
        "atanh" => {
            if v <= -1.0 || v >= 1.0 { return "Error: atanh domain is (-1, 1)".to_string(); }
            format!("atanh({v}) = {}", v.atanh())
        }
        "asech" => {
            if v <= 0.0 || v > 1.0 { return "Error: asech domain is (0, 1]".to_string(); }
            format!("asech({v}) = {}", (1.0 / v).acosh())
        }
        "acsch" => {
            if v == 0.0 { return "Error: acsch(0) is undefined".to_string(); }
            format!("acsch({v}) = {}", (1.0 / v).asinh())
        }
        "acoth" => {
            if v.abs() <= 1.0 { return "Error: acoth domain is |x| > 1".to_string(); }
            format!("acoth({v}) = {}", (1.0 / v).atanh())
        }
        op => format!("Error: Unknown operation '{op}'. Supported: sinh, cosh, tanh, sech, csch, coth, asinh, acosh, atanh, asech, acsch, acoth"),
    }
}

// ── Angle conversion ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AngleConvertInput {
    #[schemars(
        description = "Operation: deg_to_rad, rad_to_deg, deg_to_grad, grad_to_deg, rad_to_grad, grad_to_rad, normalize_360, normalize_180"
    )]
    pub operation: String,
    #[schemars(description = "Angle value to convert")]
    pub value: f64,
}

pub fn angle_convert(input: AngleConvertInput) -> String {
    let v = input.value;
    match input.operation.as_str() {
        "deg_to_rad" => format!("{v}° = {} rad", deg_to_rad(v)),
        "rad_to_deg" => format!("{v} rad = {}°", rad_to_deg(v)),
        "deg_to_grad" => format!("{v}° = {} grad", v * 10.0 / 9.0),
        "grad_to_deg" => format!("{v} grad = {}°", v * 9.0 / 10.0),
        "rad_to_grad" => format!("{v} rad = {} grad", v * 200.0 / PI),
        "grad_to_rad" => format!("{v} grad = {} rad", v * PI / 200.0),
        "normalize_360" => {
            let n = v % 360.0;
            let n = if n < 0.0 { n + 360.0 } else { n };
            format!("normalize_360({v}°) = {n}°")
        }
        "normalize_180" => {
            let n = v % 360.0;
            let n = if n < 0.0 { n + 360.0 } else { n };
            let n = if n > 180.0 { n - 360.0 } else { n };
            format!("normalize_180({v}°) = {n}°")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: deg_to_rad, rad_to_deg, deg_to_grad, grad_to_deg, rad_to_grad, grad_to_rad, normalize_360, normalize_180"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trig(v: f64, op: &str, unit: Option<&str>) -> String {
        trigonometry(TrigInput {
            operation: op.to_string(),
            value: v,
            angle_unit: unit.map(|s| s.to_string()),
        })
    }

    #[test]
    fn test_sin_90_deg() {
        let r = trig(90.0, "sin", Some("degrees"));
        assert!(r.contains("1"), "{r}");
    }
    #[test]
    fn test_cos_0() {
        assert!(trig(0.0, "cos", None).contains("1"));
    }
    #[test]
    fn test_asin_domain_err() {
        assert!(trig(2.0, "asin", None).contains("Error"));
    }
    #[test]
    fn test_hypot() {
        let r = trigonometry_2arg(Trig2ArgInput {
            operation: "hypot".to_string(),
            y: 3.0,
            x: 4.0,
        });
        assert!(r.contains("5"), "{r}");
    }
    #[test]
    fn test_sinh_zero() {
        assert!(hyperbolic(HyperbolicInput {
            operation: "sinh".to_string(),
            value: 0.0
        })
        .contains("0"));
    }
    #[test]
    fn test_atanh_invalid() {
        assert!(hyperbolic(HyperbolicInput {
            operation: "atanh".to_string(),
            value: 1.0
        })
        .contains("Error"));
    }
    #[test]
    fn test_deg_to_rad() {
        let r = angle_convert(AngleConvertInput {
            operation: "deg_to_rad".to_string(),
            value: 180.0,
        });
        assert!(r.contains("3.14"), "{r}");
    }
    #[test]
    fn test_normalize_360() {
        let r = angle_convert(AngleConvertInput {
            operation: "normalize_360".to_string(),
            value: -90.0,
        });
        assert!(r.contains("270"), "{r}");
    }
}
