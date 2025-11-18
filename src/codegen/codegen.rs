use anyhow::Result;
use log::{error, info};

use crate::{codegen::daisy_dsl::generate_daisy_dsl, config::CONFIG, ir::program::Program};

pub fn generate_code(program: &Program) -> Result<()> {
    // create folder if not exist
    let folder = CONFIG.read().unwrap().codegen_dir.clone();
    std::fs::create_dir_all(folder).unwrap();

    match generate_daisy_dsl(&program.clone()) {
        Ok(_) => info!("Daisy DSL code generation succeeded."),
        Err(e) => error!("DaisyDSL code generation failed: {}", e),
    }

    Ok(())
}
