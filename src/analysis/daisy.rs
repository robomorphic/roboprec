use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context, Result};
use indexmap::map::Entry;
use indexmap::IndexMap;

use crate::analysis::real::Integer;
use crate::ir::precision::Precision;

/// Inclusive range bounds reported by Daisy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DaisyRange {
    pub lower: f64,
    pub upper: f64,
}

pub type DaisyRanges = IndexMap<String, DaisyRange>;
pub type DaisyErrors = IndexMap<String, f64>;
pub type DaisyPrecisions = IndexMap<String, Precision>;

/// Parse Daisy range analysis output (e.g. `daisy/ranges.txt`) into a map.
pub fn parse_daisy_ranges<P: AsRef<Path>>(path: P) -> Result<DaisyRanges> {
    let file = File::open(&path).with_context(|| {
        format!("Failed to open Daisy ranges file at {}", path.as_ref().display())
    })?;
    let reader = BufReader::new(file);

    let mut ranges = DaisyRanges::new();

    for (idx, line) in reader.lines().enumerate() {
        let line = line.with_context(|| {
            format!("Failed to read line {} of Daisy ranges file", idx + 1)
        })?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (identifier, raw_values) = trimmed.split_once(':').with_context(|| {
            format!(
                "Malformed line {} in Daisy ranges file: missing ':' separator",
                idx + 1
            )
        })?;

        let identifier = identifier.trim().to_string();
        let values = raw_values.trim();

        let values = values
            .strip_prefix('[')
            .and_then(|v| v.strip_suffix(']'))
            .with_context(|| {
                format!(
                    "Malformed line {} in Daisy ranges file: expected '[lower, upper]'",
                    idx + 1
                )
            })?;

        let mut parts = values.split(',').map(str::trim);
        let lower = parts
            .next()
            .with_context(|| {
                format!(
                    "Malformed line {} in Daisy ranges file: missing lower bound",
                    idx + 1
                )
            })?
            .parse::<f64>()
            .with_context(|| {
                format!(
                    "Malformed line {} in Daisy ranges file: invalid lower bound '{}'.",
                    idx + 1,
                    values
                )
            })?;

        let upper = parts
            .next()
            .with_context(|| {
                format!(
                    "Malformed line {} in Daisy ranges file: missing upper bound",
                    idx + 1
                )
            })?
            .parse::<f64>()
            .with_context(|| {
                format!(
                    "Malformed line {} in Daisy ranges file: invalid upper bound '{}'.",
                    idx + 1,
                    values
                )
            })?;

        if parts.next().is_some() {
            anyhow::bail!(
                "Malformed line {} in Daisy ranges file: unexpected extra data",
                idx + 1
            );
        }

        let entry = ranges
            .entry(identifier)
            .or_insert(DaisyRange { lower, upper });
        entry.lower = entry.lower.min(lower);
        entry.upper = entry.upper.max(upper);
    }

    Ok(ranges)
}

/// Parse Daisy error analysis output (e.g. `daisy/errors.txt`) into a map.
pub fn parse_daisy_errors<P: AsRef<Path>>(path: P) -> Result<DaisyErrors> {
    let file = File::open(&path).with_context(|| {
        format!("Failed to open Daisy errors file at {}", path.as_ref().display())
    })?;
    let reader = BufReader::new(file);

    let mut errors = DaisyErrors::new();

    for (idx, line) in reader.lines().enumerate() {
        let line = line.with_context(|| {
            format!("Failed to read line {} of Daisy errors file", idx + 1)
        })?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (identifier, value) = trimmed.split_once(':').with_context(|| {
            format!(
                "Malformed line {} in Daisy errors file: missing ':' separator",
                idx + 1
            )
        })?;

        let identifier = identifier.trim().to_string();
        let value = value.trim().parse::<f64>().with_context(|| {
            format!(
                "Malformed line {} in Daisy errors file: failed to parse numeric value",
                idx + 1
            )
        })?;

        match errors.entry(identifier) {
            Entry::Vacant(slot) => {
                slot.insert(value);
            }
            Entry::Occupied(mut slot) => {
                if value.abs() > slot.get().abs() {
                    slot.insert(value);
                }
            }
        }
    }

    Ok(errors)
}

pub fn parse_daisy_precisions<P: AsRef<Path>>(path: P, range_results: &DaisyRanges) -> Result<DaisyPrecisions> {
    let file = File::open(&path).with_context(|| {
        format!("Failed to open Daisy precisions file at {}", path.as_ref().display())
    })?;
    let reader = BufReader::new(file);

    let mut precisions = DaisyPrecisions::new();

    // Every Line is like: "id": Precision, Precision is either Fixed{num}, Float32, Float64
    for (idx, line) in reader.lines().enumerate() {
        let line = line.with_context(|| {
            format!("Failed to read line {} of Daisy precisions file", idx + 1)
        })?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (identifier, value) = trimmed.split_once(':').with_context(|| {
            format!(
                "Malformed line {} in Daisy precisions file: missing ':' separator",
                idx + 1
            )
        })?;

        let identifier = identifier.trim().to_string();
        let value = value.trim();

        let precision = if value.starts_with("Fixed") {
            let bits_str = value
                .strip_prefix("Fixed")
                .with_context(|| {
                    format!(
                        "Malformed line {} in Daisy precisions file: expected 'Fixed{{total}}'",
                        idx + 1
                    )
                })?;

            if let Some((total_str, frac_str)) = bits_str.split_once('-') {
                let total_bits = total_str.parse::<i32>().with_context(|| {
                    format!(
                        "Malformed line {} in Daisy precisions file: invalid total bits",
                        idx + 1
                    )
                })?;
                let fractional_bits = frac_str.parse::<i32>().with_context(|| {
                    format!(
                        "Malformed line {} in Daisy precisions file: invalid fractional bits",
                        idx + 1
                    )
                })?;
                Precision::Fixed {
                    total_bits,
                    fractional_bits,
                }
            } else {
                let total_bits = bits_str
                    .parse::<i32>()
                    .with_context(|| {
                        format!(
                            "Malformed line {} in Daisy precisions file: missing total bits",
                            idx + 1
                        )
                    })?;

                // first, get the absolute max of ranges
                let range = range_results.get(&identifier).with_context(|| {
                    format!(
                        "Daisy precisions file line {} references unknown identifier '{}'",
                        idx + 1,
                        identifier
                    )
                })?;
                let abs_max = range.lower.abs().max(range.upper.abs());
                let abs_max = abs_max.floor() as u32;
                // convert it to rug Integer
                // TODO: get rid of rug dependency later
                let abs_max = Integer::from_u32(abs_max);
                let integer_bits = if abs_max.is_zero() {
                    1
                } else {
                    abs_max.bits() as i32 + 1
                };

                Precision::Fixed {
                    total_bits,
                    fractional_bits: total_bits - integer_bits,
                }
            }
        } else if value == "Float32" {
            Precision::Float32
        } else if value == "Double" {
            Precision::Float64
        } else {
            anyhow::bail!(
                "Malformed line {} in Daisy precisions file: unknown precision '{}'",
                idx + 1,
                value
            );
        };
        precisions.insert(identifier, precision);
    }

    Ok(precisions)
}

pub fn write_ranges_to_file<P: AsRef<Path>>(ranges: &DaisyRanges, path: P) -> Result<()> {
    let mut file = File::create(&path).with_context(|| {
        format!(
            "Failed to create Daisy ranges output file at {}",
            path.as_ref().display()
        )
    })?;

    use std::io::Write;
    for (identifier, range) in ranges {
        writeln!(file, "{}: [{}, {}]", identifier, range.lower, range.upper).with_context(|| {
            format!(
                "Failed to write to Daisy ranges output file at {}",
                path.as_ref().display()
            )
        })?;
    }

    Ok(())
}

pub fn write_errors_to_file<P: AsRef<Path>>(errors: &DaisyErrors, path: P) -> Result<()> {
    let mut file = File::create(&path).with_context(|| {
        format!(
            "Failed to create Daisy errors output file at {}",
            path.as_ref().display()
        )
    })?;

    use std::io::Write;
    for (identifier, error) in errors {
        writeln!(file, "{}: {}", identifier, error).with_context(|| {
            format!(
                "Failed to write to Daisy errors output file at {}",
                path.as_ref().display()
            )
        })?;
    }

    Ok(())
}

pub fn write_precisions_to_file<P: AsRef<Path>>(precisions: &DaisyPrecisions, path: P) -> Result<()> {
    let mut file = File::create(&path).with_context(|| {
        format!(
            "Failed to create Daisy precisions output file at {}",
            path.as_ref().display()
        )
    })?;

    use std::io::Write;
    for (identifier, precision) in precisions {
        let precision_str = match precision {
            Precision::Fixed { total_bits, fractional_bits } => format!("Fixed{{{}, {}}}", total_bits, fractional_bits),
            Precision::Float32 => "Float32".to_string(),
            Precision::Float64 => "Float64".to_string(),
        };
        writeln!(file, "{}: {}", identifier, precision_str).with_context(|| {
            format!(
                "Failed to write to Daisy precisions output file at {}",
                path.as_ref().display()
            )
        })?;
    }

    Ok(())
}