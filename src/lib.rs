//! # RoboPrec
//!
//! A verification-aware compiler framework for generating numerically-guaranteed,
//! high-performance code for robotics algorithms.
//!
//! RoboPrec enables safe deployment of compute-intensive kernels to embedded platforms
//! by providing formal worst-case error bounds across mixed-precision datatypes.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use roboprec::*;
//! use std::path::PathBuf;
//!
//! fn main() -> anyhow::Result<()> {
//!     // Define input variable with range
//!     let x = add_input_scalar("x",
//!         (Real::from_f64(0.0), Real::from_f64(1.0)),
//!         0.5);
//!
//!     // Perform computation
//!     let mut result = &x * &x;
//!
//!     // Register output
//!     register_scalar_output(&mut result, "x_squared");
//!
//!     // Configure and run analysis
//!     let config = Config {
//!         precision: Precision::Fixed {
//!             total_bits: 32,
//!             fractional_bits: -1  // Auto-optimize
//!         },
//!         output_dir: PathBuf::from("output/"),
//!     };
//!
//!     analysis(config)?;
//!     Ok(())
//! }
//! ```
//!
//! ## Core Concepts
//!
//! - **[`Scalar`]**: Single numerical values with tracked operations
//! - **[`Vector`]**: Column vectors for robotics computations  
//! - **[`Matrix`]**: 2D matrices for transformations and dynamics
//! - **[`Precision`]**: Target numerical precision (Float32/64, Fixed-point)
//! - **[`Config`]**: Analysis configuration
//! - **[`analysis`]**: Main analysis entry point
//!
//! ## Features
//!
//! - Formal worst-case error analysis via [Daisy](https://github.com/malyzajko/daisy)
//! - Automatic or manual mixed-precision optimization
//! - Generates optimized C code (floating-point or fixed-point)

extern crate self as roboprec;

pub mod analysis;
pub mod config;
pub mod ir;
pub mod types;
pub mod logger;
pub mod codegen;

pub use analysis::analysis::analysis;
pub use analysis::real::Real;
pub use config::Config;
pub use ir::precision::Precision;
pub use ir::program::{
    register_scalar_output,
    register_vector_output,
    register_matrix_output,
    add_input_scalar,
    add_input_vector,
    add_input_matrix,
};
pub use types::matrix::Matrix;
pub use types::scalar::Scalar;
pub use types::vector::Vector;


