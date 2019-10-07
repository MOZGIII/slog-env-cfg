use crate::{InvalidLogFormat, LogFormat, LogFormatFromEnvError, LogFormatFromEnvWithDefaultError};
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
