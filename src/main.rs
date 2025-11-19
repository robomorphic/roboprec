mod algorithms;
mod analysis;
mod codegen;
mod config;
mod helpers;
mod ir; // intermediate representation
mod logger;
mod tests;
mod types;
mod examples;

use anyhow::Result;
use clap::Parser;
use roboprec::analysis::analysis::analysis;
use roboprec::{Config, Real, Precision, add_input_scalar, register_scalar_output};
use crate::examples::fk_7dof;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Precision format (e.g., Fixed16-8, Float32, Float64)
    #[arg(short, long)]
    precision: String,
}

// Re-export the types and macros at the crate root
pub use types::matrix::Matrix;
pub use types::scalar::Scalar;
pub use types::vector::Vector;


fn main() -> Result<()> {
    let args = Args::parse();
    let precision = Precision::from_str(&args.precision).map_err(|e| anyhow::anyhow!(e))?;

    let config = Config {
        precision,
        ..Default::default()
    };

    //fk_7dof(config)?;
    //rnea_deriv_4dof(config)?;
    //rnea_deriv_7dof(config)?;



        
    // Define the input variable 'x' with its range and a default value.
    let input_range: (Real, Real) = (Real::from_f64(0.0), Real::from_f64(1.0));
    let default_val = 0.5;
    let x = &add_input_scalar("x", input_range, default_val);

    // Perform the computation, in this case, cubing the input.
    let mut res = x * x * x;

    // Register the result as a scalar output named "output".
    register_scalar_output(&mut res, "output");

    // Optionally, assert the correctness of the computation in Rust.
    assert!(res.value.to_f64() == default_val.powi(3));

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
