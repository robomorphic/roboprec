mod algorithms;
mod helpers;
mod examples;
#[cfg(test)]
mod tests;

use anyhow::Result;
use clap::Parser;
use roboprec::{Config, Precision};
use crate::examples::fk_7dof;
use std::str::FromStr;

// Re-export types so macros using $crate work in the binary
pub use roboprec::{Vector, Matrix};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Precision format (e.g., Fixed16-8, Float32, Float64)
    #[arg(short, long)]
    precision: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let precision = Precision::from_str(&args.precision).map_err(|e| anyhow::anyhow!(e))?;

    let config = Config {
        precision,
        ..Default::default()
    };

    fk_7dof(config)?;
    // rnea_deriv_4dof(config)?;
    // rnea_deriv_7dof(config)?;

    Ok(())
}
