use schemars::JsonSchema;
use serde::Deserialize;
use std::f64::consts::PI;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnitConvertInput {
    #[schemars(description = "Value to convert")]
    pub value: f64,
    #[schemars(
        description = "Source unit (e.g. km, lb, fahrenheit, hours, mph, MB, psi, joule, watt, newton, degrees)"
    )]
    pub from_unit: String,
    #[schemars(description = "Target unit (same family as from_unit)")]
    pub to_unit: String,
}

// All conversions via SI base units.
// Returns Some((value_in_si, unit_name)) or None if unknown.
fn to_si(value: f64, unit: &str) -> Option<(f64, &'static str)> {
    match unit {
        // Length → meters
        "m" | "meter" | "meters" => Some((value, "m")),
        "km" | "kilometer" | "kilometers" => Some((value * 1_000.0, "m")),
        "cm" | "centimeter" | "centimeters" => Some((value / 100.0, "m")),
        "mm" | "millimeter" | "millimeters" => Some((value / 1_000.0, "m")),
        "um" | "micrometer" | "micrometers" | "micron" => Some((value / 1_000_000.0, "m")),
        "nm" | "nanometer" | "nanometers" => Some((value / 1_000_000_000.0, "m")),
        "inch" | "inches" | "in" => Some((value * 0.0254, "m")),
        "foot" | "feet" | "ft" => Some((value * 0.3048, "m")),
        "yard" | "yards" | "yd" => Some((value * 0.9144, "m")),
        "mile" | "miles" | "mi" => Some((value * 1_609.344, "m")),
        "nautical_mile" | "nmi" => Some((value * 1_852.0, "m")),
        // Mass → kilograms
        "kg" | "kilogram" | "kilograms" => Some((value, "kg")),
        "g" | "gram" | "grams" => Some((value / 1_000.0, "kg")),
        "mg" | "milligram" | "milligrams" => Some((value / 1_000_000.0, "kg")),
        "ug" | "microgram" | "micrograms" => Some((value / 1_000_000_000.0, "kg")),
        "lb" | "lbs" | "pound" | "pounds" => Some((value * 0.453_592_37, "kg")),
        "oz" | "ounce" | "ounces" => Some((value * 0.028_349_523_125, "kg")),
        "ton" | "tonne" | "metric_ton" => Some((value * 1_000.0, "kg")),
        "short_ton" | "us_ton" => Some((value * 907.184_74, "kg")),
        // Time → seconds
        "s" | "sec" | "second" | "seconds" => Some((value, "s")),
        "ms" | "millisecond" | "milliseconds" => Some((value / 1_000.0, "s")),
        "us" | "microsecond" | "microseconds" => Some((value / 1_000_000.0, "s")),
        "min" | "minute" | "minutes" => Some((value * 60.0, "s")),
        "hour" | "hours" | "hr" | "h" => Some((value * 3_600.0, "s")),
        "day" | "days" | "d" => Some((value * 86_400.0, "s")),
        "week" | "weeks" => Some((value * 604_800.0, "s")),
        "year" | "years" | "yr" => Some((value * 31_557_600.0, "s")),
        // Speed → m/s
        "m/s" | "mps" => Some((value, "m/s")),
        "km/h" | "kph" | "kmh" => Some((value / 3.6, "m/s")),
        "mph" | "mi/h" => Some((value * 0.44704, "m/s")),
        "knot" | "knots" | "kt" => Some((value * 0.514_444, "m/s")),
        "ft/s" | "fps" => Some((value * 0.3048, "m/s")),
        // Data size → bytes
        "B" | "byte" | "bytes" => Some((value, "B")),
        "KB" | "kilobyte" | "kilobytes" => Some((value * 1_000.0, "B")),
        "MB" | "megabyte" | "megabytes" => Some((value * 1_000_000.0, "B")),
        "GB" | "gigabyte" | "gigabytes" => Some((value * 1_000_000_000.0, "B")),
        "TB" | "terabyte" | "terabytes" => Some((value * 1_000_000_000_000.0, "B")),
        "KiB" | "kibibyte" | "kibibytes" => Some((value * 1_024.0, "B")),
        "MiB" | "mebibyte" | "mebibytes" => Some((value * 1_048_576.0, "B")),
        "GiB" | "gibibyte" | "gibibytes" => Some((value * 1_073_741_824.0, "B")),
        "TiB" | "tebibyte" | "tebibytes" => Some((value * 1_099_511_627_776.0, "B")),
        // Pressure → pascals
        "Pa" | "pascal" | "pascals" => Some((value, "Pa")),
        "kPa" | "kilopascal" => Some((value * 1_000.0, "Pa")),
        "MPa" | "megapascal" => Some((value * 1_000_000.0, "Pa")),
        "bar" => Some((value * 100_000.0, "Pa")),
        "mbar" | "millibar" => Some((value * 100.0, "Pa")),
        "atm" | "atmosphere" => Some((value * 101_325.0, "Pa")),
        "psi" => Some((value * 6_894.757_293, "Pa")),
        "mmHg" | "torr" => Some((value * 133.322_387, "Pa")),
        // Energy → joules
        "J" | "joule" | "joules" => Some((value, "J")),
        "kJ" | "kilojoule" => Some((value * 1_000.0, "J")),
        "MJ" | "megajoule" => Some((value * 1_000_000.0, "J")),
        "cal" | "calorie" | "calories" => Some((value * 4.184, "J")),
        "kcal" | "kilocalorie" | "Calorie" => Some((value * 4_184.0, "J")),
        "Wh" | "watt_hour" => Some((value * 3_600.0, "J")),
        "kWh" | "kilowatt_hour" => Some((value * 3_600_000.0, "J")),
        "eV" | "electronvolt" => Some((value * 1.602_176_634e-19, "J")),
        "BTU" | "btu" => Some((value * 1_055.06, "J")),
        // Power → watts
        "W" | "watt" | "watts" => Some((value, "W")),
        "kW" | "kilowatt" => Some((value * 1_000.0, "W")),
        "MW" | "megawatt" => Some((value * 1_000_000.0, "W")),
        "hp" | "horsepower" => Some((value * 745.699_872, "W")),
        // Force → newtons
        "N" | "newton" | "newtons" => Some((value, "N")),
        "kN" | "kilonewton" => Some((value * 1_000.0, "N")),
        "lbf" | "pound_force" => Some((value * 4.448_221_615, "N")),
        "dyn" | "dyne" => Some((value * 1e-5, "N")),
        // Angle → radians
        "rad" | "radian" | "radians" => Some((value, "rad")),
        "deg" | "degree" | "degrees" => Some((value * PI / 180.0, "rad")),
        "grad" | "gradian" | "gradians" => Some((value * PI / 200.0, "rad")),
        _ => None,
    }
}

fn from_si(si_value: f64, unit: &str) -> Option<f64> {
    match unit {
        "m" | "meter" | "meters" => Some(si_value),
        "km" | "kilometer" | "kilometers" => Some(si_value / 1_000.0),
        "cm" | "centimeter" | "centimeters" => Some(si_value * 100.0),
        "mm" | "millimeter" | "millimeters" => Some(si_value * 1_000.0),
        "um" | "micrometer" | "micrometers" | "micron" => Some(si_value * 1_000_000.0),
        "nm" | "nanometer" | "nanometers" => Some(si_value * 1_000_000_000.0),
        "inch" | "inches" | "in" => Some(si_value / 0.0254),
        "foot" | "feet" | "ft" => Some(si_value / 0.3048),
        "yard" | "yards" | "yd" => Some(si_value / 0.9144),
        "mile" | "miles" | "mi" => Some(si_value / 1_609.344),
        "nautical_mile" | "nmi" => Some(si_value / 1_852.0),
        "kg" | "kilogram" | "kilograms" => Some(si_value),
        "g" | "gram" | "grams" => Some(si_value * 1_000.0),
        "mg" | "milligram" | "milligrams" => Some(si_value * 1_000_000.0),
        "ug" | "microgram" | "micrograms" => Some(si_value * 1_000_000_000.0),
        "lb" | "lbs" | "pound" | "pounds" => Some(si_value / 0.453_592_37),
        "oz" | "ounce" | "ounces" => Some(si_value / 0.028_349_523_125),
        "ton" | "tonne" | "metric_ton" => Some(si_value / 1_000.0),
        "short_ton" | "us_ton" => Some(si_value / 907.184_74),
        "s" | "sec" | "second" | "seconds" => Some(si_value),
        "ms" | "millisecond" | "milliseconds" => Some(si_value * 1_000.0),
        "us" | "microsecond" | "microseconds" => Some(si_value * 1_000_000.0),
        "min" | "minute" | "minutes" => Some(si_value / 60.0),
        "hour" | "hours" | "hr" | "h" => Some(si_value / 3_600.0),
        "day" | "days" | "d" => Some(si_value / 86_400.0),
        "week" | "weeks" => Some(si_value / 604_800.0),
        "year" | "years" | "yr" => Some(si_value / 31_557_600.0),
        "m/s" | "mps" => Some(si_value),
        "km/h" | "kph" | "kmh" => Some(si_value * 3.6),
        "mph" | "mi/h" => Some(si_value / 0.44704),
        "knot" | "knots" | "kt" => Some(si_value / 0.514_444),
        "ft/s" | "fps" => Some(si_value / 0.3048),
        "B" | "byte" | "bytes" => Some(si_value),
        "KB" | "kilobyte" | "kilobytes" => Some(si_value / 1_000.0),
        "MB" | "megabyte" | "megabytes" => Some(si_value / 1_000_000.0),
        "GB" | "gigabyte" | "gigabytes" => Some(si_value / 1_000_000_000.0),
        "TB" | "terabyte" | "terabytes" => Some(si_value / 1_000_000_000_000.0),
        "KiB" | "kibibyte" | "kibibytes" => Some(si_value / 1_024.0),
        "MiB" | "mebibyte" | "mebibytes" => Some(si_value / 1_048_576.0),
        "GiB" | "gibibyte" | "gibibytes" => Some(si_value / 1_073_741_824.0),
        "TiB" | "tebibyte" | "tebibytes" => Some(si_value / 1_099_511_627_776.0),
        "Pa" | "pascal" | "pascals" => Some(si_value),
        "kPa" | "kilopascal" => Some(si_value / 1_000.0),
        "MPa" | "megapascal" => Some(si_value / 1_000_000.0),
        "bar" => Some(si_value / 100_000.0),
        "mbar" | "millibar" => Some(si_value / 100.0),
        "atm" | "atmosphere" => Some(si_value / 101_325.0),
        "psi" => Some(si_value / 6_894.757_293),
        "mmHg" | "torr" => Some(si_value / 133.322_387),
        "J" | "joule" | "joules" => Some(si_value),
        "kJ" | "kilojoule" => Some(si_value / 1_000.0),
        "MJ" | "megajoule" => Some(si_value / 1_000_000.0),
        "cal" | "calorie" | "calories" => Some(si_value / 4.184),
        "kcal" | "kilocalorie" | "Calorie" => Some(si_value / 4_184.0),
        "Wh" | "watt_hour" => Some(si_value / 3_600.0),
        "kWh" | "kilowatt_hour" => Some(si_value / 3_600_000.0),
        "eV" | "electronvolt" => Some(si_value / 1.602_176_634e-19),
        "BTU" | "btu" => Some(si_value / 1_055.06),
        "W" | "watt" | "watts" => Some(si_value),
        "kW" | "kilowatt" => Some(si_value / 1_000.0),
        "MW" | "megawatt" => Some(si_value / 1_000_000.0),
        "hp" | "horsepower" => Some(si_value / 745.699_872),
        "N" | "newton" | "newtons" => Some(si_value),
        "kN" | "kilonewton" => Some(si_value / 1_000.0),
        "lbf" | "pound_force" => Some(si_value / 4.448_221_615),
        "dyn" | "dyne" => Some(si_value / 1e-5),
        "rad" | "radian" | "radians" => Some(si_value),
        "deg" | "degree" | "degrees" => Some(si_value * 180.0 / PI),
        "grad" | "gradian" | "gradians" => Some(si_value * 200.0 / PI),
        _ => None,
    }
}

pub fn unit_convert(input: UnitConvertInput) -> String {
    let from = input.from_unit.trim();
    let to = input.to_unit.trim();

    // Temperature handled separately (not a simple factor)
    let temp_result = convert_temperature(input.value, from, to);
    if let Some(result) = temp_result {
        return result;
    }

    let (si_value, si_type) = match to_si(input.value, from) {
        Some(x) => x,
        None => return format!("Error: unknown unit '{from}'"),
    };

    let to_si_type = match to_si(1.0, to) {
        Some((_, t)) => t,
        None => return format!("Error: unknown unit '{to}'"),
    };

    if si_type != to_si_type {
        return format!("Error: cannot convert '{from}' to '{to}' (incompatible units: {si_type} vs {to_si_type})");
    }

    match from_si(si_value, to) {
        Some(result) => format!("{} {from} = {result} {to}", input.value),
        None => format!("Error: unknown unit '{to}'"),
    }
}

fn convert_temperature(value: f64, from: &str, to: &str) -> Option<String> {
    let is_temp = |u: &str| {
        matches!(
            u,
            "celsius" | "c" | "fahrenheit" | "f" | "kelvin" | "k" | "rankine" | "r"
        )
    };
    if !is_temp(from) && !is_temp(to) {
        return None;
    }

    let celsius = match from {
        "celsius" | "c" => value,
        "fahrenheit" | "f" => (value - 32.0) * 5.0 / 9.0,
        "kelvin" | "k" => value - 273.15,
        "rankine" | "r" => (value - 491.67) * 5.0 / 9.0,
        u => return Some(format!("Error: unknown temperature unit '{u}'")),
    };

    let result = match to {
        "celsius" | "c" => celsius,
        "fahrenheit" | "f" => celsius * 9.0 / 5.0 + 32.0,
        "kelvin" | "k" => celsius + 273.15,
        "rankine" | "r" => (celsius + 273.15) * 9.0 / 5.0,
        u => return Some(format!("Error: unknown temperature unit '{u}'")),
    };

    Some(format!("{value} {from} = {result} {to}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conv(v: f64, from: &str, to: &str) -> String {
        unit_convert(UnitConvertInput {
            value: v,
            from_unit: from.to_string(),
            to_unit: to.to_string(),
        })
    }

    #[test]
    fn test_km_to_m() {
        assert!(conv(1.0, "km", "m").contains("1000"));
    }
    #[test]
    fn test_celsius_to_fahrenheit() {
        assert!(conv(0.0, "celsius", "fahrenheit").contains("32"));
    }
    #[test]
    fn test_celsius_to_kelvin() {
        assert!(conv(0.0, "celsius", "kelvin").contains("273.15"));
    }
    #[test]
    fn test_kg_to_lb() {
        let r = conv(1.0, "kg", "lb");
        assert!(r.contains("2.2") || r.contains("2.20"), "{r}");
    }
    #[test]
    fn test_incompatible() {
        assert!(conv(1.0, "km", "kg").contains("Error"));
    }
    #[test]
    fn test_mb_to_kb() {
        assert!(conv(1.0, "MB", "KB").contains("1000"));
    }
}
