use once_cell::sync::Lazy;
use std::sync::RwLock;
use crate::ir::precision::Precision;

pub const CODEGEN_FILENAME: &str = "codegen";
pub const CODEGEN_DIR: &str = "output/codegen";

#[derive(Debug, Clone)]
pub struct ConfigType {
    pub precision: Precision,
}

pub static CONFIG: Lazy<RwLock<ConfigType>> = Lazy::new(|| {
    RwLock::new(ConfigType {
        precision: Precision::Fixed {
            total_bits: 32,
            fractional_bits: 16,
        },
    })
});

pub static TEST_CONFIG: Lazy<ConfigType> = Lazy::new(|| {
    ConfigType {
        precision: Precision::Fixed {
            total_bits: 32,
            fractional_bits: 16,
        },
    }
});

pub fn load_test_config() {
    *CONFIG.write().unwrap() = TEST_CONFIG.clone();
}

pub fn load_special_config(config: ConfigType) {
    *CONFIG.write().unwrap() = config;
}

pub fn init_config(precision: Precision) {
    let mut config = CONFIG.write().unwrap();
    config.precision = precision;
}
