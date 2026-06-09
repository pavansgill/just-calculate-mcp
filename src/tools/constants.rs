use schemars::JsonSchema;
use serde::Deserialize;
use std::f64::consts;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MathConstantInput {
    #[schemars(
        description = "Constant name: pi, e, tau, phi, sqrt2, sqrt3, ln2, ln10, c, g, h, avogadro, boltzmann"
    )]
    pub name: String,
}

pub fn math_constant(input: MathConstantInput) -> String {
    match input.name.to_lowercase().as_str() {
        "pi" | "π" => format!("π = {}", consts::PI),
        "e" => format!("e = {}", consts::E),
        "tau" | "τ" => format!("τ = {}", consts::TAU),
        "phi" | "φ" | "golden_ratio" => {
            let phi = (1.0 + 5f64.sqrt()) / 2.0;
            format!("φ = {phi}")
        }
        "sqrt2" | "√2" => format!("√2 = {}", consts::SQRT_2),
        "sqrt3" | "√3" => {
            format!("√3 = {}", 3f64.sqrt())
        }
        "ln2" => format!("ln(2) = {}", consts::LN_2),
        "ln10" => format!("ln(10) = {}", consts::LN_10),
        "c" | "speed_of_light" => "c = 299792458 m/s (speed of light in vacuum)".to_string(),
        "g" | "gravity" | "gravitational_acceleration" => {
            "g = 9.80665 m/s² (standard gravitational acceleration)".to_string()
        }
        "h" | "planck" => "h = 6.62607015e-34 J·s (Planck constant)".to_string(),
        "avogadro" | "na" => "Nₐ = 6.02214076e23 mol⁻¹ (Avogadro constant)".to_string(),
        "boltzmann" | "kb" | "k_b" => {
            "kB = 1.380649e-23 J/K (Boltzmann constant)".to_string()
        }
        name => format!("Error: Unknown constant '{name}'. Supported: pi, e, tau, phi, sqrt2, sqrt3, ln2, ln10, c, g, h, avogadro, boltzmann"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mc(name: &str) -> String {
        math_constant(MathConstantInput {
            name: name.to_string(),
        })
    }

    #[test]
    fn test_pi() {
        assert!(mc("pi").contains("3.14159"));
    }
    #[test]
    fn test_e() {
        assert!(mc("e").contains("2.71828"));
    }
    #[test]
    fn test_tau() {
        assert!(mc("tau").contains("6.28318"));
    }
    #[test]
    fn test_phi() {
        assert!(mc("phi").contains("1.61803"));
    }
    #[test]
    fn test_c() {
        assert!(mc("c").contains("299792458"));
    }
    #[test]
    fn test_avogadro() {
        assert!(mc("avogadro").contains("6.02214"));
    }
    #[test]
    fn test_unknown() {
        assert!(mc("foo").contains("Error"));
    }
}
