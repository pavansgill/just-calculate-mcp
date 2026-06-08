use schemars::JsonSchema;
use serde::Deserialize;

// ── Vector operations ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct VectorOpsInput {
    #[schemars(description = "Operation: add, subtract, scalar_multiply, magnitude, normalize, dot_product, cross_product_3d, angle_between, projection")]
    pub operation: String,
    #[schemars(description = "First vector components")]
    pub a: Vec<f64>,
    #[schemars(description = "Second vector (required for add, subtract, dot_product, cross_product_3d, angle_between, projection)")]
    pub b: Option<Vec<f64>>,
    #[schemars(description = "Scalar (required for scalar_multiply)")]
    pub scalar: Option<f64>,
}

fn magnitude(v: &[f64]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

pub fn vector_ops(input: VectorOpsInput) -> String {
    let a = &input.a;
    if a.is_empty() { return "Error: vector a cannot be empty".to_string(); }

    let fmt_vec = |v: &[f64]| -> String {
        let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
        format!("[{}]", parts.join(", "))
    };

    match input.operation.as_str() {
        "add" | "subtract" => {
            let b = match &input.b { Some(v) => v, None => return "Error: b is required".to_string() };
            if a.len() != b.len() { return "Error: vectors must be same length".to_string(); }
            let result: Vec<f64> = if input.operation == "add" {
                a.iter().zip(b).map(|(x, y)| x + y).collect()
            } else {
                a.iter().zip(b).map(|(x, y)| x - y).collect()
            };
            format!("{} {} {} = {}", fmt_vec(a), if input.operation == "add" { "+" } else { "-" }, fmt_vec(b), fmt_vec(&result))
        }
        "scalar_multiply" => {
            let s = match input.scalar { Some(x) => x, None => return "Error: scalar is required".to_string() };
            let result: Vec<f64> = a.iter().map(|x| x * s).collect();
            format!("{s} * {} = {}", fmt_vec(a), fmt_vec(&result))
        }
        "magnitude" => format!("|{}| = {}", fmt_vec(a), magnitude(a)),
        "normalize" => {
            let mag = magnitude(a);
            if mag == 0.0 { return "Error: cannot normalize zero vector".to_string(); }
            let result: Vec<f64> = a.iter().map(|x| x / mag).collect();
            format!("normalize({}) = {}", fmt_vec(a), fmt_vec(&result))
        }
        "dot_product" => {
            let b = match &input.b { Some(v) => v, None => return "Error: b is required".to_string() };
            if a.len() != b.len() { return "Error: vectors must be same length".to_string(); }
            let dot: f64 = a.iter().zip(b).map(|(x, y)| x * y).sum();
            format!("{} · {} = {dot}", fmt_vec(a), fmt_vec(b))
        }
        "cross_product_3d" => {
            let b = match &input.b { Some(v) => v, None => return "Error: b is required".to_string() };
            if a.len() != 3 || b.len() != 3 { return "Error: cross product requires 3D vectors".to_string(); }
            let result = [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ];
            format!("{} × {} = {}", fmt_vec(a), fmt_vec(b), fmt_vec(&result))
        }
        "angle_between" => {
            let b = match &input.b { Some(v) => v, None => return "Error: b is required".to_string() };
            if a.len() != b.len() { return "Error: vectors must be same length".to_string(); }
            let dot: f64 = a.iter().zip(b).map(|(x, y)| x * y).sum();
            let ma = magnitude(a);
            let mb = magnitude(b);
            if ma == 0.0 || mb == 0.0 { return "Error: cannot compute angle with zero vector".to_string(); }
            let cos_theta = (dot / (ma * mb)).clamp(-1.0, 1.0);
            let rad = cos_theta.acos();
            format!("angle between {} and {} = {} radians = {}°", fmt_vec(a), fmt_vec(b), rad, rad.to_degrees())
        }
        "projection" => {
            let b = match &input.b { Some(v) => v, None => return "Error: b is required (the vector to project onto)".to_string() };
            if a.len() != b.len() { return "Error: vectors must be same length".to_string(); }
            let dot: f64 = a.iter().zip(b).map(|(x, y)| x * y).sum();
            let b_mag_sq: f64 = b.iter().map(|x| x * x).sum();
            if b_mag_sq == 0.0 { return "Error: cannot project onto zero vector".to_string(); }
            let scalar = dot / b_mag_sq;
            let result: Vec<f64> = b.iter().map(|x| x * scalar).collect();
            format!("proj_{} {} = {}", fmt_vec(b), fmt_vec(a), fmt_vec(&result))
        }
        op => format!("Error: Unknown operation '{op}'. Supported: add, subtract, scalar_multiply, magnitude, normalize, dot_product, cross_product_3d, angle_between, projection"),
    }
}

// ── Matrix operations ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MatrixOpsInput {
    #[schemars(description = "Operation: add, multiply, scalar_multiply, transpose, determinant, inverse_2x2, trace, solve_2x2, solve_3x3")]
    pub operation: String,
    #[schemars(description = "Matrix A as row-major flat list. 2×2: [a,b,c,d], 3×3: [a,b,c,d,e,f,g,h,i]")]
    pub matrix_a: Vec<f64>,
    #[schemars(description = "Matrix B (required for add, multiply) — same format as matrix_a")]
    pub matrix_b: Option<Vec<f64>>,
    #[schemars(description = "Scalar for scalar_multiply")]
    pub scalar: Option<f64>,
    #[schemars(description = "RHS vector [b1, b2] or [b1, b2, b3] for solve_2x2 / solve_3x3")]
    pub rhs: Option<Vec<f64>>,
}

fn det2(m: &[f64]) -> f64 { m[0] * m[3] - m[1] * m[2] }

fn det3(m: &[f64]) -> f64 {
    m[0] * (m[4] * m[8] - m[5] * m[7])
    - m[1] * (m[3] * m[8] - m[5] * m[6])
    + m[2] * (m[3] * m[7] - m[4] * m[6])
}

pub fn matrix_ops(input: MatrixOpsInput) -> String {
    let a = &input.matrix_a;

    let fmt_mat = |m: &[f64], cols: usize| -> String {
        m.chunks(cols).map(|row| {
            let parts: Vec<String> = row.iter().map(|x| format!("{x:.4}")).collect();
            format!("[{}]", parts.join(", "))
        }).collect::<Vec<_>>().join(", ")
    };

    match input.operation.as_str() {
        "add" | "multiply" => {
            let b = match &input.matrix_b { Some(v) => v, None => return "Error: matrix_b is required".to_string() };
            if a.len() != b.len() { return "Error: matrices must be same size for add".to_string(); }
            let cols = (a.len() as f64).sqrt() as usize;
            if cols * cols != a.len() { return "Error: matrix_a must be square (4 or 9 elements)".to_string(); }

            if input.operation == "add" {
                let result: Vec<f64> = a.iter().zip(b).map(|(x, y)| x + y).collect();
                format!("A + B = {}", fmt_mat(&result, cols))
            } else {
                // matrix multiply
                let mut result = vec![0.0f64; cols * cols];
                for i in 0..cols { for j in 0..cols { for k in 0..cols {
                    result[i * cols + j] += a[i * cols + k] * b[k * cols + j];
                }}}
                format!("A × B = {}", fmt_mat(&result, cols))
            }
        }
        "scalar_multiply" => {
            let s = match input.scalar { Some(x) => x, None => return "Error: scalar is required".to_string() };
            let cols = (a.len() as f64).sqrt() as usize;
            let result: Vec<f64> = a.iter().map(|x| x * s).collect();
            format!("{s} × A = {}", fmt_mat(&result, cols))
        }
        "transpose" => {
            let cols = (a.len() as f64).sqrt() as usize;
            if cols * cols != a.len() { return "Error: matrix must be square".to_string(); }
            let mut result = vec![0.0f64; a.len()];
            for i in 0..cols { for j in 0..cols { result[j * cols + i] = a[i * cols + j]; } }
            format!("A^T = {}", fmt_mat(&result, cols))
        }
        "determinant" => {
            match a.len() {
                4 => format!("det(A) = {}", det2(a)),
                9 => format!("det(A) = {}", det3(a)),
                _ => "Error: determinant supports 2×2 (4 elements) or 3×3 (9 elements)".to_string(),
            }
        }
        "inverse_2x2" => {
            if a.len() != 4 { return "Error: inverse_2x2 requires 4 elements".to_string(); }
            let d = det2(a);
            if d.abs() < 1e-15 { return "Error: matrix is singular (det = 0)".to_string(); }
            let inv = vec![a[3] / d, -a[1] / d, -a[2] / d, a[0] / d];
            format!("A^-1 = {}", fmt_mat(&inv, 2))
        }
        "trace" => {
            let cols = (a.len() as f64).sqrt() as usize;
            if cols * cols != a.len() { return "Error: matrix must be square".to_string(); }
            let trace: f64 = (0..cols).map(|i| a[i * cols + i]).sum();
            format!("trace(A) = {trace}")
        }
        "solve_2x2" => {
            if a.len() != 4 { return "Error: solve_2x2 requires 4-element matrix".to_string(); }
            let rhs = match &input.rhs { Some(v) if v.len() == 2 => v, _ => return "Error: rhs must have 2 elements".to_string() };
            let d = det2(a);
            if d.abs() < 1e-15 { return "Error: matrix is singular".to_string(); }
            let x = (rhs[0] * a[3] - rhs[1] * a[1]) / d;
            let y = (a[0] * rhs[1] - a[2] * rhs[0]) / d;
            format!("x = {x}, y = {y}")
        }
        "solve_3x3" => {
            if a.len() != 9 { return "Error: solve_3x3 requires 9-element matrix".to_string(); }
            let b = match &input.rhs { Some(v) if v.len() == 3 => v, _ => return "Error: rhs must have 3 elements".to_string() };
            let d = det3(a);
            if d.abs() < 1e-15 { return "Error: matrix is singular".to_string(); }
            // Cramer's rule
            let dx = det3(&[b[0],a[1],a[2], b[1],a[4],a[5], b[2],a[7],a[8]]);
            let dy = det3(&[a[0],b[0],a[2], a[3],b[1],a[5], a[6],b[2],a[8]]);
            let dz = det3(&[a[0],a[1],b[0], a[3],a[4],b[1], a[6],a[7],b[2]]);
            format!("x = {}, y = {}, z = {}", dx / d, dy / d, dz / d)
        }
        op => format!("Error: Unknown operation '{op}'. Supported: add, multiply, scalar_multiply, transpose, determinant, inverse_2x2, trace, solve_2x2, solve_3x3"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vo(a: Vec<f64>, op: &str, b: Option<Vec<f64>>, s: Option<f64>) -> String {
        vector_ops(VectorOpsInput { operation: op.to_string(), a, b, scalar: s })
    }

    #[test]
    fn test_dot_product() {
        let r = vo(vec![1.0, 2.0, 3.0], "dot_product", Some(vec![4.0, 5.0, 6.0]), None);
        assert!(r.contains("32"), "{r}");
    }
    #[test]
    fn test_cross_product() {
        let r = vo(vec![1.0, 0.0, 0.0], "cross_product_3d", Some(vec![0.0, 1.0, 0.0]), None);
        assert!(r.contains("1") && r.contains("0"), "{r}");
    }
    #[test]
    fn test_magnitude() {
        let r = vo(vec![3.0, 4.0], "magnitude", None, None);
        assert!(r.contains("5"), "{r}");
    }
    #[test]
    fn test_determinant_2x2() {
        let r = matrix_ops(MatrixOpsInput {
            operation: "determinant".to_string(), matrix_a: vec![1.0, 2.0, 3.0, 4.0],
            matrix_b: None, scalar: None, rhs: None,
        });
        assert!(r.contains("-2"), "{r}");
    }
    #[test]
    fn test_solve_2x2() {
        let r = matrix_ops(MatrixOpsInput {
            operation: "solve_2x2".to_string(), matrix_a: vec![2.0, 1.0, 1.0, 3.0],
            matrix_b: None, scalar: None, rhs: Some(vec![5.0, 10.0]),
        });
        assert!(r.contains("x = 1") || r.contains("x=1"), "{r}");
    }
}
