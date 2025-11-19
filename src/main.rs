mod algorithms;
mod helpers;
mod examples;
#[cfg(test)]
mod tests;

use anyhow::Result;
use clap::Parser;
use roboprec::analysis::analysis::analysis;
use roboprec::{Config, Real, Precision, add_input_scalar, register_scalar_output, Scalar};
use crate::examples::fk_7dof;
use std::path::PathBuf;
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

    // fk_7dof(config)?;
    // rnea_deriv_4dof(config)?;
    // rnea_deriv_7dof(config)?;



        
    // Define the input variable 'x' with its range and a default value.
    let input_range: (Real, Real) = (Real::from_f64(0.0), Real::from_f64(1.0));
    let default_val = 0.5;
    let mut x = add_input_scalar("x", input_range, default_val);

    // Perform the computation, in this case, cubing the input.
    for i in 0..3 {
        println!("Iteration {}", i);
        x = &x * &x;
        if i == 0 {
            x = x + Scalar!(1.0);
        }
        if i == 2 {
            register_scalar_output(&mut x, "intermediate_output");
        }
    }

    // Set config for analysis
    let config = Config {
        precision: Precision::Fixed { total_bits: 16, fractional_bits: -1 },
        //precision: Precision::Float64,
        output_dir: PathBuf::from("output/"),
    };

    // Trigger the main analysis process.
    analysis(config)?;
    


    Ok(())
}
