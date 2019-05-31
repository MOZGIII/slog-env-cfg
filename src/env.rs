use crate::{Config, LogFormat, Result};
use std::env;

/// Build LogFormat from env vars.
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

/// Build Config from env vars.
pub fn config_from_env() -> Result<Config> {
    let format: LogFormat = log_format_from_env("LOG_FORMAT", LogFormat::Terminal)?;
    Ok(Config { format })
}

#[test]
fn env_test_term() {
    std::env::set_var("LOG_FORMAT", "term");
    assert_eq!(
        config_from_env().unwrap(),
        Config {
            format: LogFormat::Terminal
        }
    );
}

#[test]
fn env_test_json() {
    std::env::set_var("LOG_FORMAT", "json");
    assert_eq!(
        config_from_env().unwrap(),
        Config {
            format: LogFormat::Json
        }
    );
}

#[test]
fn env_test_unset() {
    std::env::remove_var("LOG_FORMAT");
    assert_eq!(
        config_from_env().unwrap(),
        Config {
            format: LogFormat::Terminal
        }
    );
}

#[test]
fn env_test_empty() {
    std::env::set_var("LOG_FORMAT", "");
    assert!(config_from_env().is_err());
}

#[test]
fn env_test_invalid() {
    std::env::set_var("LOG_FORMAT", "invalid");
    assert!(config_from_env().is_err());
}
