use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context, Result};
use indexmap::map::Entry;
use indexmap::IndexMap;

/// Inclusive range bounds reported by Daisy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DaisyRange {
    pub lower: f64,
    pub upper: f64,
}

pub type DaisyRanges = IndexMap<String, DaisyRange>;
pub type DaisyErrors = IndexMap<String, f64>;

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