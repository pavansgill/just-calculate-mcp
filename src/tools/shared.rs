use std::f64::consts::PI;

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a, b) * b
}

pub fn factorial(n: u64) -> Result<u64, String> {
    if n > 20 {
        return Err(format!("factorial({n}) overflows u64; max is 20!"));
    }
    Ok((1..=n).product())
}

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 3), 1);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(0, 5), 0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0).unwrap(), 1);
        assert_eq!(factorial(5).unwrap(), 120);
        assert!(factorial(21).is_err());
    }

    #[test]
    fn test_angle_conv() {
        let r = deg_to_rad(180.0);
        assert!((r - PI).abs() < 1e-10);
        assert!((rad_to_deg(PI) - 180.0).abs() < 1e-10);
    }
}
