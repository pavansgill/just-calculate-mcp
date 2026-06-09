use schemars::JsonSchema;
use serde::Deserialize;

// ── Base conversion ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BaseConvertInput {
    #[schemars(
        description = "Operation: decimal_to_binary, decimal_to_hex, decimal_to_octal, binary_to_decimal, hex_to_decimal, octal_to_decimal, to_base, from_base"
    )]
    pub operation: String,
    #[schemars(
        description = "Input value as string (to handle arbitrary bases and leading zeros)"
    )]
    pub value: String,
    #[schemars(
        description = "Target base for to_base (2-36), or source base for from_base (2-36)"
    )]
    pub base: Option<u32>,
}

fn to_base_str(mut n: u64, base: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let digits: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = Vec::new();
    while n > 0 {
        result.push(digits[(n % base as u64) as usize]);
        n /= base as u64;
    }
    result.iter().rev().collect()
}

pub fn base_convert(input: BaseConvertInput) -> String {
    let v = input.value.trim().to_lowercase();
    match input.operation.as_str() {
        "decimal_to_binary" => {
            match v.parse::<u64>() {
                Ok(n) => format!("{n} (decimal) = {} (binary)", to_base_str(n, 2)),
                Err(_) => "Error: value must be a non-negative integer".to_string(),
            }
        }
        "decimal_to_hex" => {
            match v.parse::<u64>() {
                Ok(n) => format!("{n} (decimal) = 0x{} (hex)", to_base_str(n, 16).to_uppercase()),
                Err(_) => "Error: value must be a non-negative integer".to_string(),
            }
        }
        "decimal_to_octal" => {
            match v.parse::<u64>() {
                Ok(n) => format!("{n} (decimal) = {} (octal)", to_base_str(n, 8)),
                Err(_) => "Error: value must be a non-negative integer".to_string(),
            }
        }
        "binary_to_decimal" => {
            match u64::from_str_radix(&v, 2) {
                Ok(n) => format!("{} (binary) = {n} (decimal)", input.value),
                Err(_) => "Error: value must be a valid binary string".to_string(),
            }
        }
        "hex_to_decimal" => {
            let clean = v.strip_prefix("0x").unwrap_or(&v);
            match u64::from_str_radix(clean, 16) {
                Ok(n) => format!("{} (hex) = {n} (decimal)", input.value),
                Err(_) => "Error: value must be a valid hex string".to_string(),
            }
        }
        "octal_to_decimal" => {
            match u64::from_str_radix(&v, 8) {
                Ok(n) => format!("{} (octal) = {n} (decimal)", input.value),
                Err(_) => "Error: value must be a valid octal string".to_string(),
            }
        }
        "to_base" => {
            let base = match input.base { Some(b) if (2..=36).contains(&b) => b, _ => return "Error: base must be 2-36".to_string() };
            match v.parse::<u64>() {
                Ok(n) => format!("{n} (base 10) = {} (base {base})", to_base_str(n, base)),
                Err(_) => "Error: value must be a non-negative integer".to_string(),
            }
        }
        "from_base" => {
            let base = match input.base { Some(b) if (2..=36).contains(&b) => b, _ => return "Error: base must be 2-36".to_string() };
            match u64::from_str_radix(&v, base) {
                Ok(n) => format!("{} (base {base}) = {n} (base 10)", input.value),
                Err(_) => format!("Error: value '{}' is not valid in base {base}", input.value),
            }
        }
        op => format!("Error: Unknown operation '{op}'. Supported: decimal_to_binary, decimal_to_hex, decimal_to_octal, binary_to_decimal, hex_to_decimal, octal_to_decimal, to_base, from_base"),
    }
}

// ── Bitwise operations ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BitwiseOpsInput {
    #[schemars(
        description = "Operation: and, or, xor, not, left_shift, right_shift, rotate_left, rotate_right, popcount, bit_length, leading_zeros, trailing_zeros"
    )]
    pub operation: String,
    #[schemars(description = "First operand (64-bit unsigned integer)")]
    pub a: u64,
    #[schemars(
        description = "Second operand or shift amount (required for and, or, xor, left_shift, right_shift, rotate_left, rotate_right)"
    )]
    pub b: Option<u64>,
}

pub fn bitwise_ops(input: BitwiseOpsInput) -> String {
    let a = input.a;
    let b = input.b.unwrap_or(0);
    match input.operation.as_str() {
        "and" => format!("{a} & {b} = {}", a & b),
        "or" => format!("{a} | {b} = {}", a | b),
        "xor" => format!("{a} ^ {b} = {}", a ^ b),
        "not" => format!("~{a} = {}", !a),
        "left_shift" => {
            if b >= 64 { return "Error: shift amount must be < 64".to_string(); }
            format!("{a} << {b} = {}", a << b)
        }
        "right_shift" => {
            if b >= 64 { return "Error: shift amount must be < 64".to_string(); }
            format!("{a} >> {b} = {}", a >> b)
        }
        "rotate_left" => format!("{a} rotl {b} = {}", a.rotate_left(b as u32 % 64)),
        "rotate_right" => format!("{a} rotr {b} = {}", a.rotate_right(b as u32 % 64)),
        "popcount" => format!("popcount({a}) = {}", a.count_ones()),
        "bit_length" => format!("bit_length({a}) = {}", 64 - a.leading_zeros()),
        "leading_zeros" => format!("leading_zeros({a}) = {}", a.leading_zeros()),
        "trailing_zeros" => format!("trailing_zeros({a}) = {}", a.trailing_zeros()),
        op => format!("Error: Unknown operation '{op}'. Supported: and, or, xor, not, left_shift, right_shift, rotate_left, rotate_right, popcount, bit_length, leading_zeros, trailing_zeros"),
    }
}

// ── Number representations ────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NumberReprInput {
    #[schemars(
        description = "Operation: to_twos_complement, from_twos_complement, ascii_to_codepoint, codepoint_to_char, float_to_bits_hex"
    )]
    pub operation: String,
    #[schemars(description = "Numeric input (integer or float depending on operation)")]
    pub value: f64,
    #[schemars(description = "Bit width for twos complement (8, 16, 32, 64)")]
    pub bits: Option<u32>,
}

pub fn number_repr(input: NumberReprInput) -> String {
    let v = input.value;
    match input.operation.as_str() {
        "to_twos_complement" => {
            let bits = input.bits.unwrap_or(8);
            if ![8u32, 16, 32, 64].contains(&bits) { return "Error: bits must be 8, 16, 32, or 64".to_string(); }
            let n = v as i64;
            let mask = if bits == 64 { u64::MAX } else { (1u64 << bits) - 1 };
            let tc = (n as u64) & mask;
            format!("{n} in {bits}-bit two's complement = {tc} (0x{tc:0>width$x})", width = (bits / 4) as usize)
        }
        "from_twos_complement" => {
            let bits = input.bits.unwrap_or(8);
            if ![8u32, 16, 32, 64].contains(&bits) { return "Error: bits must be 8, 16, 32, or 64".to_string(); }
            let n = v as u64;
            let sign_bit = 1u64 << (bits - 1);
            let value = if n & sign_bit != 0 {
                let mask = if bits == 64 { u64::MAX } else { (1u64 << bits) - 1 };
                -(((!n & mask) + 1) as i64)
            } else {
                n as i64
            };
            format!("{n} ({bits}-bit two's complement) = {value} (signed)")
        }
        "ascii_to_codepoint" => {
            let code = v as u32;
            match char::from_u32(code) {
                Some(c) => format!("U+{code:04X} = '{c}'"),
                None => format!("Error: {code} is not a valid Unicode codepoint"),
            }
        }
        "codepoint_to_char" => {
            let code = v as u32;
            match char::from_u32(code) {
                Some(c) => format!("'{c}' = U+{code:04X} = {code} (decimal)"),
                None => format!("Error: {code} is not a valid Unicode codepoint"),
            }
        }
        "float_to_bits_hex" => {
            let bits = (v as f32).to_bits();
            let bits64 = v.to_bits();
            format!("f32: 0x{bits:08X} (sign={}, exp={}, mantissa={})\nf64: 0x{bits64:016X}",
                (bits >> 31) & 1, (bits >> 23) & 0xFF, bits & 0x7FFFFF)
        }
        op => format!("Error: Unknown operation '{op}'. Supported: to_twos_complement, from_twos_complement, ascii_to_codepoint, codepoint_to_char, float_to_bits_hex"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bc(v: &str, op: &str, base: Option<u32>) -> String {
        base_convert(BaseConvertInput {
            operation: op.to_string(),
            value: v.to_string(),
            base,
        })
    }

    #[test]
    fn test_decimal_to_binary() {
        assert!(bc("10", "decimal_to_binary", None).contains("1010"));
    }
    #[test]
    fn test_decimal_to_hex() {
        assert!(bc("255", "decimal_to_hex", None).contains("FF"));
    }
    #[test]
    fn test_binary_to_decimal() {
        assert!(bc("1010", "binary_to_decimal", None).contains("10"));
    }
    #[test]
    fn test_hex_to_decimal() {
        assert!(bc("FF", "hex_to_decimal", None).contains("255"));
    }

    fn bw(a: u64, op: &str, b: Option<u64>) -> String {
        bitwise_ops(BitwiseOpsInput {
            operation: op.to_string(),
            a,
            b,
        })
    }

    #[test]
    fn test_and() {
        assert!(bw(0b1100, "and", Some(0b1010)).contains("8"));
    }
    #[test]
    fn test_popcount() {
        assert!(bw(0b1010, "popcount", None).contains("2"));
    }
    #[test]
    fn test_left_shift() {
        assert!(bw(1, "left_shift", Some(3)).contains("8"));
    }

    #[test]
    fn test_float_bits() {
        let r = number_repr(NumberReprInput {
            operation: "float_to_bits_hex".to_string(),
            value: 0.0,
            bits: None,
        });
        assert!(r.contains("0x00000000"), "{r}");
    }
}
