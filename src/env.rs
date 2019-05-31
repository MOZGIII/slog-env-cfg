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


#[cfg(test)]
mod test {
    use super::*;
    use serial_test_derive::serial;

    #[serial]
    #[test]
    fn log_format_term() {
        std::env::set_var("LOG_FORMAT", "term");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_json() {
        std::env::set_var("LOG_FORMAT", "json");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Json
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_unset() {
        std::env::remove_var("LOG_FORMAT");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_empty() {
        std::env::set_var("LOG_FORMAT", "");
        assert!(config_from_env().is_err());
    }

    #[serial]
    #[test]
    fn log_format_invalid() {
        std::env::set_var("LOG_FORMAT", "invalid");
        assert!(config_from_env().is_err());
    }
}
