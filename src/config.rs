use crate::ir::precision::Precision;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub precision: Precision,
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

