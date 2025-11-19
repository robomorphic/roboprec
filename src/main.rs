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
use crate::examples::fk_7dof;
use crate::ir::precision::Precision;
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

use crate::config::Config;

fn main() -> Result<()> {
    let args = Args::parse();
    let precision = Precision::from_str(&args.precision).map_err(|e| anyhow::anyhow!(e))?;

    let config = Config {
        precision,
        ..Default::default()
    };

    fk_7dof(config)?;
    //rnea_deriv_4dof(config)?;
    //rnea_deriv_7dof(config)?;



    /*
    let input_range: (Real, Real) = (Real::from_f64(0.0), Real::from_f64(1.0));
    let default_val = 0.5;
    let x = &get_program().add_input_scalar("x", input_range, default_val);

    // Do operation
    let mut res = x * x * x;

    // Register output
    register_scalar_output(&mut res, "output");

    // Do the analysis
    analysis()?;

    // If you want, you can check the output values in rust
    assert!(res.value.to_f64() == default_val.powi(3));
    */


    Ok(())
}
