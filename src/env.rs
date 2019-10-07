use crate::{
    Config, InvalidLogFormat, LogFormat, LogFormatFromEnvError, LogFormatFromEnvWithDefaultError,
};
use std::env;

/// Build LogFormat from env vars.
pub fn log_format_from_env(key: &str) -> Result<LogFormat, LogFormatFromEnvError> {
    match env::var(key) {
        Ok(val) => match val.parse() {
            Ok(format) => Ok(format),
            Err(_err @ InvalidLogFormat) => Err(LogFormatFromEnvError::InvalidFormat(val)),
        },
        Err(env::VarError::NotPresent) => Err(LogFormatFromEnvError::NotPresent),
        Err(env::VarError::NotUnicode(val)) => Err(LogFormatFromEnvError::NotUnicode(val)),
    }
}

/// Build LogFormat from env, or return default if the env value is not set.
pub fn log_format_from_env_with_default(
    key: &str,
    default: LogFormat,
) -> Result<LogFormat, LogFormatFromEnvWithDefaultError> {
    match log_format_from_env(key) {
        Ok(val) => Ok(val),
        Err(LogFormatFromEnvError::NotPresent) => Ok(default),
        Err(LogFormatFromEnvError::NotUnicode(val)) => {
            Err(LogFormatFromEnvWithDefaultError::NotUnicode(val))
        }
        Err(LogFormatFromEnvError::InvalidFormat(val)) => {
            Err(LogFormatFromEnvWithDefaultError::InvalidFormat(val))
        }
    }
}

/// Build Config using the `LOG_FORMAT` env var.
pub fn config_from_env() -> Result<Config, LogFormatFromEnvWithDefaultError> {
    let format: LogFormat = log_format_from_env_with_default("LOG_FORMAT", LogFormat::Terminal)?;
    Ok(Config { format })
}

#[cfg(test)]
mod test {
    use super::*;
    use matches::assert_matches;
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
        assert_matches!(
            config_from_env(),
            Err(LogFormatFromEnvWithDefaultError::InvalidFormat(ref s)) if s == ""
        );
    }

    #[serial]
    #[test]
    fn log_format_invalid() {
        std::env::set_var("LOG_FORMAT", "invalid");
        assert_matches!(
            config_from_env(),
            Err(LogFormatFromEnvWithDefaultError::InvalidFormat(ref s)) if s == "invalid"
        );
    }
}
