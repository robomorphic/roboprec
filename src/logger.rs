use chrono::Local;
use log::LevelFilter;
use once_cell::sync::OnceCell;
use std::path::PathBuf;

static LOG_FILE_PATH: OnceCell<PathBuf> = OnceCell::new();

/// Configures the logger to write to a timestamped file in the `.log` directory and returns the file path.
pub fn setup_logger() -> Result<PathBuf, fern::InitError> {
    if let Some(existing) = LOG_FILE_PATH.get() {
        return Ok(existing.clone());
    }

    // Create the .log directory if it doesn't exist
    std::fs::create_dir_all(".log")?;

    // Generate a timestamped filename
    let log_file_name = if cfg!(test) {
        format!(
            ".log/{}.log",
            Local::now().format("testlog_%Y-%m-%d_%H-%M-%S")
        )
    } else {
        format!(".log/{}.log", Local::now().format("%Y-%m-%d_%H-%M-%S"))
    };

    let log_level = LevelFilter::Info;

    // Base logging configuration
    let base_config = fern::Dispatch::new()
        .format(|out, message, record| match record.level() {
            log::Level::Error | log::Level::Warn => {
                out.finish(format_args!("{}: {}", record.level(), message))
            }
            _ => out.finish(format_args!("{}", message)),
        })
        .level(log_level); // Set the minimum log level

    // Separate configurations for file and console
    let file_config = fern::Dispatch::new().chain(fern::log_file(&log_file_name)?);

    // Apply both configurations
    base_config.chain(file_config).apply()?;

    let log_path = PathBuf::from(&log_file_name);
    let _ = LOG_FILE_PATH.set(log_path.clone());

    Ok(log_path)
}
