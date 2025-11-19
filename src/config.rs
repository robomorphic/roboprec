use crate::ir::precision::Precision;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub precision: Precision,
    pub codegen_dir: PathBuf,
    pub codegen_filename: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            precision: Precision::Float64,
            codegen_dir: PathBuf::from("output/"),
            codegen_filename: "codegen".to_string(),
        }
    }
}

