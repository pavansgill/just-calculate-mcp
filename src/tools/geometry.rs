use schemars::JsonSchema;
use serde::Deserialize;
use std::f64::consts::PI;
use crate::tools::shared::deg_to_rad;

// ── 2D Area ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Area2dInput {
    #[schemars(description = "Shape: rectangle, square, triangle, triangle_heron, circle, sector, trapezoid, parallelogram, ellipse, regular_polygon")]
    pub shape: String,
    #[schemars(description = "a: width/base/radius/side-a depending on shape")]
    pub a: f64,
    #[schemars(description = "b: height/side-b/semi-minor/slant depending on shape")]
    pub b: Option<f64>,
    #[schemars(description = "c: side-c (triangle_heron), angle in degrees (sector), number of sides (regular_polygon)")]
    pub c: Option<f64>,
}

pub fn area_2d(input: Area2dInput) -> String {
    let a = input.a;
    let b = || input.b.unwrap_or(0.0);
    let c = || input.c.unwrap_or(0.0);
    match input.shape.as_str() {
        "rectangle" => format!("area = {}", a * b()),
        "square" => format!("area = {}", a * a),
        "triangle" => format!("area = {}", 0.5 * a * b()),
        "triangle_heron" => {
            let s = (a + b() + c()) / 2.0;
            let area_sq = s * (s - a) * (s - b()) * (s - c());
            if area_sq < 0.0 { return "Error: invalid triangle sides".to_string(); }
            format!("area = {}", area_sq.sqrt())
        }
        "circle" => format!("area = {}", PI * a * a),
        "sector" => {
            let angle_deg = b();
            format!("area = {}", 0.5 * a * a * deg_to_rad(angle_deg))
        }
        "trapezoid" => format!("area = {}", 0.5 * (a + b()) * c()),
        "parallelogram" => format!("area = {}", a * b()),
        "ellipse" => format!("area = {}", PI * a * b()),
        "regular_polygon" => {
            let n = c();
            if n < 3.0 { return "Error: regular_polygon needs n >= 3 sides".to_string(); }
            format!("area = {}", (n * a * a) / (4.0 * (PI / n).tan()))
        }
        s => format!("Error: Unknown shape '{s}'"),
    }
}

// ── 2D Perimeter ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Perimeter2dInput {
    #[schemars(description = "Shape: rectangle, square, triangle, circle, ellipse_approx, regular_polygon")]
    pub shape: String,
    #[schemars(description = "a: width/side-a/radius/semi-major")]
    pub a: f64,
    #[schemars(description = "b: height/side-b/semi-minor")]
    pub b: Option<f64>,
    #[schemars(description = "c: side-c (triangle), number of sides (regular_polygon)")]
    pub c: Option<f64>,
}

pub fn perimeter_2d(input: Perimeter2dInput) -> String {
    let a = input.a;
    let b = input.b.unwrap_or(0.0);
    let c = input.c.unwrap_or(0.0);
    match input.shape.as_str() {
        "rectangle" => format!("perimeter = {}", 2.0 * (a + b)),
        "square" => format!("perimeter = {}", 4.0 * a),
        "triangle" => format!("perimeter = {}", a + b + c),
        "circle" => format!("circumference = {}", 2.0 * PI * a),
        "ellipse_approx" => {
            // Ramanujan approximation
            let h = ((a - b) / (a + b)).powi(2);
            let p = PI * (a + b) * (1.0 + 3.0 * h / (10.0 + (4.0 - 3.0 * h).sqrt()));
            format!("perimeter ≈ {p}")
        }
        "regular_polygon" => format!("perimeter = {}", c * a),
        s => format!("Error: Unknown shape '{s}'"),
    }
}

// ── 3D Volume ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Volume3dInput {
    #[schemars(description = "Shape: box_, sphere, cylinder, cone, pyramid, prism, torus")]
    pub shape: String,
    #[schemars(description = "a: length/radius/base-area depending on shape")]
    pub a: f64,
    #[schemars(description = "b: width/height/tube-radius depending on shape")]
    pub b: Option<f64>,
    #[schemars(description = "c: height (box_, cylinder, cone, pyramid, prism)")]
    pub c: Option<f64>,
}

pub fn volume_3d(input: Volume3dInput) -> String {
    let a = input.a;
    let b = input.b.unwrap_or(0.0);
    let c = input.c.unwrap_or(0.0);
    match input.shape.as_str() {
        "box_" | "box" => format!("volume = {}", a * b * c),
        "sphere" => format!("volume = {}", (4.0 / 3.0) * PI * a.powi(3)),
        "cylinder" => format!("volume = {}", PI * a * a * b),
        "cone" => format!("volume = {}", (1.0 / 3.0) * PI * a * a * b),
        "pyramid" => format!("volume = {}", (1.0 / 3.0) * a * b),
        "prism" => format!("volume = {}", a * b),
        "torus" => format!("volume = {}", 2.0 * PI * PI * b * b * a),
        s => format!("Error: Unknown shape '{s}'"),
    }
}

// ── 3D Surface area ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SurfaceArea3dInput {
    #[schemars(description = "Shape: sphere, cylinder, cone, box_")]
    pub shape: String,
    #[schemars(description = "a: radius (sphere/cylinder/cone) or length (box_)")]
    pub a: f64,
    #[schemars(description = "b: height (cylinder/cone) or width (box_)")]
    pub b: Option<f64>,
    #[schemars(description = "c: height (box_)")]
    pub c: Option<f64>,
}

pub fn surface_area_3d(input: SurfaceArea3dInput) -> String {
    let a = input.a;
    let b = input.b.unwrap_or(0.0);
    let c = input.c.unwrap_or(0.0);
    match input.shape.as_str() {
        "sphere" => format!("surface_area = {}", 4.0 * PI * a * a),
        "cylinder" => format!("surface_area = {}", 2.0 * PI * a * (a + b)),
        "cone" => {
            let slant = (a * a + b * b).sqrt();
            format!("surface_area = {}", PI * a * (a + slant))
        }
        "box_" | "box" => format!("surface_area = {}", 2.0 * (a * b + b * c + a * c)),
        s => format!("Error: Unknown shape '{s}'"),
    }
}

// ── Distance / 2D relations ───────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Distance2dInput {
    #[schemars(description = "Operation: euclidean, manhattan, midpoint_x, midpoint_y, slope, pythagorean, euclidean_3d")]
    pub operation: String,
    #[schemars(description = "x1 (or a for pythagorean)")]
    pub x1: f64,
    #[schemars(description = "y1 (or b for pythagorean)")]
    pub y1: f64,
    #[schemars(description = "x2 (or c for pythagorean — omit to solve for c)")]
    pub x2: Option<f64>,
    #[schemars(description = "y2")]
    pub y2: Option<f64>,
    #[schemars(description = "z1 (euclidean_3d)")]
    pub z1: Option<f64>,
    #[schemars(description = "z2 (euclidean_3d)")]
    pub z2: Option<f64>,
}

pub fn distance_2d(input: Distance2dInput) -> String {
    let (x1, y1) = (input.x1, input.y1);
    let x2 = input.x2.unwrap_or(0.0);
    let y2 = input.y2.unwrap_or(0.0);

    match input.operation.as_str() {
        "euclidean" => format!("distance = {}", ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()),
        "manhattan" => format!("distance = {}", (x2 - x1).abs() + (y2 - y1).abs()),
        "midpoint_x" => format!("midpoint_x = {}", (x1 + x2) / 2.0),
        "midpoint_y" => format!("midpoint_y = {}", (y1 + y2) / 2.0),
        "slope" => {
            if (x2 - x1).abs() < 1e-15 { return "Error: slope undefined (vertical line)".to_string(); }
            format!("slope = {}", (y2 - y1) / (x2 - x1))
        }
        "pythagorean" => {
            let c = input.x2;
            match c {
                None => format!("c = {}", (x1 * x1 + y1 * y1).sqrt()),
                Some(c_val) => {
                    if c_val < x1 { return "Error: hypotenuse must be >= a".to_string(); }
                    format!("b = {}", (c_val * c_val - x1 * x1).sqrt())
                }
            }
        }
        "euclidean_3d" => {
            let z1 = input.z1.unwrap_or(0.0);
            let z2 = input.z2.unwrap_or(0.0);
            format!("distance = {}", ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt())
        }
        op => format!("Error: Unknown operation '{op}'. Supported: euclidean, manhattan, midpoint_x, midpoint_y, slope, pythagorean, euclidean_3d"),
    }
}

// ── Coordinate conversion ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CoordinateConvertInput {
    #[schemars(description = "Operation: cartesian_to_polar, polar_to_cartesian, cartesian_to_spherical, spherical_to_cartesian")]
    pub operation: String,
    #[schemars(description = "x (cartesian) or r (polar/spherical)")]
    pub a: f64,
    #[schemars(description = "y (cartesian) or theta in degrees (polar/spherical)")]
    pub b: f64,
    #[schemars(description = "z (cartesian_to_spherical) or phi in degrees (spherical_to_cartesian)")]
    pub c: Option<f64>,
}

pub fn coordinate_convert(input: CoordinateConvertInput) -> String {
    let a = input.a;
    let b = input.b;
    match input.operation.as_str() {
        "cartesian_to_polar" => {
            let r = (a * a + b * b).sqrt();
            let theta = b.atan2(a).to_degrees();
            format!("r = {r}, theta = {theta}°")
        }
        "polar_to_cartesian" => {
            let theta_rad = deg_to_rad(b);
            format!("x = {}, y = {}", a * theta_rad.cos(), a * theta_rad.sin())
        }
        "cartesian_to_spherical" => {
            let z = input.c.unwrap_or(0.0);
            let r = (a * a + b * b + z * z).sqrt();
            let theta = if r == 0.0 { 0.0 } else { (z / r).acos().to_degrees() };
            let phi = b.atan2(a).to_degrees();
            format!("r = {r}, theta = {theta}° (polar angle), phi = {phi}° (azimuth)")
        }
        "spherical_to_cartesian" => {
            let phi_rad = deg_to_rad(b);
            let theta_rad = deg_to_rad(input.c.unwrap_or(90.0));
            let x = a * theta_rad.sin() * phi_rad.cos();
            let y = a * theta_rad.sin() * phi_rad.sin();
            let z = a * theta_rad.cos();
            format!("x = {x}, y = {y}, z = {z}")
        }
        op => format!("Error: Unknown operation '{op}'. Supported: cartesian_to_polar, polar_to_cartesian, cartesian_to_spherical, spherical_to_cartesian"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let r = area_2d(Area2dInput { shape: "circle".to_string(), a: 1.0, b: None, c: None });
        assert!(r.contains("3.14"), "{r}");
    }
    #[test]
    fn test_sphere_volume() {
        let r = volume_3d(Volume3dInput { shape: "sphere".to_string(), a: 1.0, b: None, c: None });
        assert!(r.contains("4.18") || r.contains("4.19"), "{r}");
    }
    #[test]
    fn test_pythagorean() {
        let r = distance_2d(Distance2dInput {
            operation: "pythagorean".to_string(), x1: 3.0, y1: 4.0,
            x2: None, y2: None, z1: None, z2: None,
        });
        assert!(r.contains("5"), "{r}");
    }
    #[test]
    fn test_euclidean() {
        let r = distance_2d(Distance2dInput {
            operation: "euclidean".to_string(), x1: 0.0, y1: 0.0,
            x2: Some(3.0), y2: Some(4.0), z1: None, z2: None,
        });
        assert!(r.contains("5"), "{r}");
    }
    #[test]
    fn test_polar_roundtrip() {
        let cart = coordinate_convert(CoordinateConvertInput {
            operation: "polar_to_cartesian".to_string(), a: 1.0, b: 0.0, c: None,
        });
        assert!(cart.contains("x = 1"), "{cart}");
    }
}
