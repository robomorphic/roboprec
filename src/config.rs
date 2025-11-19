use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;
use std::sync::RwLock;
use crate::ir::precision::Precision;

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigType {
    pub verbose: bool,
    pub codegen_filename: String,
    pub codegen_dir: String,
    pub precision: Precision,
}

pub static CONFIG: Lazy<RwLock<ConfigType>> = Lazy::new(|| {
    let contents = fs::read_to_string("config.toml").expect("Failed to read config.toml");

    let config = toml::from_str(&contents).expect("Failed to parse config.toml");

    RwLock::new(config)
});

pub static TEST_CONFIG: Lazy<ConfigType> = Lazy::new(|| {
    ConfigType {
        verbose: true,
        // the rest are not going to be used
        codegen_filename: "test_default".to_string(),
        codegen_dir: "./test_output/codegen".to_string(),
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

pub fn set_default_config() {
    let contents = fs::read_to_string("config.toml").expect("Failed to read config.toml");

    let default_config = toml::from_str(&contents).expect("Failed to parse config.toml");

    *CONFIG.write().unwrap() = default_config;
}
