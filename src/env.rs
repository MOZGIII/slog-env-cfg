use crate::{Config, InvalidLogFormat, LogFormat, LogFormatFromEnvError};
use std::env;

/// Build LogFormat from env vars.
pub fn log_format_from_env(
    key: &str,
    default: LogFormat,
) -> Result<LogFormat, LogFormatFromEnvError> {
    match env::var(key) {
        Ok(val) => match val.parse() {
            Ok(format) => Ok(format),
            Err(_err @ InvalidLogFormat) => Err(LogFormatFromEnvError::InvalidFormat(val)),
        },
        Err(env::VarError::NotPresent) => Ok(default),
        Err(env::VarError::NotUnicode(val)) => Err(LogFormatFromEnvError::NotUnicode(val)),
    }
}

/// Build Config from env vars.
pub fn config_from_env() -> Result<Config, LogFormatFromEnvError> {
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
