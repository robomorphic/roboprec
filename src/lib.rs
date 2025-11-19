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


