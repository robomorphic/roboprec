use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::str::FromStr;

/// Numerical precision specification for code generation.
///
/// Determines the datatype used in generated C code and guides the
/// numerical analysis.
///
/// # Variants
///
/// * `Float32` - IEEE 754 single precision (32-bit float)
/// * `Float64` - IEEE 754 double precision (64-bit double)
/// * `Fixed` - Fixed-point arithmetic with configurable bit allocation
///
/// # Fixed-Point
///
/// For fixed-point, two modes are available:
///
/// * **Manual**: Specify exact fractional bits (e.g., `fractional_bits: 16`)
/// * **Auto-optimize**: Set `fractional_bits: -1` to let RoboPrec determine
///   optimal bit allocation based on range analysis
///
/// # Examples
///
/// ```rust
/// use roboprec::Precision;
/// use std::str::FromStr;
///
/// // Floating-point
/// let p = Precision::Float64;
/// let p = Precision::Float32;
///
/// // Fixed-point with auto-optimization (recommended)
/// let p = Precision::Fixed { total_bits: 32, fractional_bits: -1 };
///
/// // Fixed-point with manual allocation, uniform precision is faster to compute because it requires less shift operations
/// let p = Precision::Fixed { total_bits: 32, fractional_bits: 16 };
///
/// // From string (for CLI/config files)
/// let p = Precision::from_str("Float64").unwrap();
/// let p = Precision::from_str("Fixed32").unwrap();      // Auto-optimize
/// let p = Precision::from_str("Fixed16-8").unwrap();   // 16 integer, 8 fractional bits
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Precision {
    /// Fixed-point arithmetic
    ///
    /// * `total_bits`: Total bit width (e.g., 16, 32, 64)
    /// * `fractional_bits`: Fractional bits, or -1 for auto-optimization
    Fixed {
        total_bits: i32,
        fractional_bits: i32,
    },
    /// IEEE 754 single precision (32-bit)
    Float32,
    /// IEEE 754 double precision (64-bit)
    Float64,
}

impl FromStr for Precision {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Float32" {
            return Ok(Precision::Float32);
        }
        if s == "Float64" {
            return Ok(Precision::Float64);
        }
        if s.starts_with("Fixed") {
            let rest = &s[5..];
            if let Some(idx) = rest.find('-') {
                let int_str = &rest[..idx];
                let frac_str = &rest[idx+1..];
                let int_bits = int_str.parse::<i32>().map_err(|_| "Invalid total bits".to_string())?;
                let fractional_bits = frac_str.parse::<i32>().map_err(|_| "Invalid fractional bits".to_string())?;
                return Ok(Precision::Fixed { total_bits: int_bits + fractional_bits, fractional_bits });
            } else {
                let total_bits = rest.parse::<i32>().map_err(|_| "Invalid total bits".to_string())?;
                return Ok(Precision::Fixed { total_bits, fractional_bits: -1 });
            }
        }
        Err(format!("Unknown precision format: {}", s))
    }
}

impl<'de> Deserialize<'de> for Precision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for Precision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Precision::Fixed { total_bits, fractional_bits } => {
                if *fractional_bits == -1 {
                    write!(f, "Fixed{}", total_bits)
                } else {
                    write!(f, "Fixed{}-{}", total_bits-fractional_bits, fractional_bits)
                }
            },
            Precision::Float32 => write!(f, "Float32"),
            Precision::Float64 => write!(f, "Float64"),
        }
    }
}
