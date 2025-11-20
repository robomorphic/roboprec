use crate::ir::precision::Precision;
use std::path::PathBuf;

/// Configuration for RoboPrec analysis and code generation.
///
/// Specifies the target numerical precision and output directory for
/// generated code and analysis results.
///
/// # Examples
///
/// ```rust
/// use roboprec::*;
/// use std::path::PathBuf;
///
/// // Use defaults (Float64, output/ directory)
/// let config = Config::default();
///
/// // Custom configuration
/// let config = Config {
///     precision: Precision::Fixed {
///         total_bits: 32,
///         fractional_bits: -1,  // Auto-optimize
///     },
///     output_dir: PathBuf::from("my_output/"),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Target numerical precision for code generation
    pub precision: Precision,
    /// Directory for generated code and analysis files
    pub output_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            precision: Precision::Float64,
            output_dir: PathBuf::from("output/"),
        }
    }
}

