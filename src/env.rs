use crate::{Config, LogFormat, Result};
use std::env;

pub fn log_format_from_env(key: &str, default: LogFormat) -> Result<LogFormat> {
    match env::var(key) {
        Ok(val) => {
            let parsed: LogFormat = val.parse()?;
            Ok(parsed)
        }
        Err(env::VarError::NotPresent) => Ok(default),
        Err(val) => Err(val)?,
    }
}

pub fn config_from_env() -> Result<Config> {
    let format: LogFormat = log_format_from_env("LOG_FORMAT", LogFormat::Terminal)?;
    Ok(Config { format })
}
