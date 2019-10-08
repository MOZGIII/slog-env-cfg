use crate::*;

mod error;
pub use error::*;

pub const LOG_FORMAT_ENV_KEY: &'static str = "LOG_FORMAT";
pub const DISABLE_ENVLOGGER_ENV_KEY: &'static str = "DISABLE_ENVLOGGER";
pub const ENVLOGGER_FILTERS_ENV_KEY: &'static str = "RUST_LOG";

/// Build `Config` using the env vars and opinionated defaults.
pub fn config_from_env() -> Result<Config, ConfigFromEnvError> {
    let format = parse_from_env(LOG_FORMAT_ENV_KEY)?.unwrap_or(LogFormat::Terminal);
    let disable_envlogger = parse_from_env(DISABLE_ENVLOGGER_ENV_KEY)?.unwrap_or(false);
    let envlogger_filters = parse_from_env(ENVLOGGER_FILTERS_ENV_KEY)?;
    let envlogger_override_default_filter = Some("debug".to_string());
    Ok(Config {
        format,
        disable_envlogger,
        envlogger_filters,
        envlogger_override_default_filter,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use matches::assert_matches;
    use serial_test_derive::serial;

    #[serial]
    #[test]
    fn log_format_term() {
        std::env::set_var(LOG_FORMAT_ENV_KEY, "term");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_envlogger: false,
                envlogger_filters: None,
                envlogger_override_default_filter: Some("trace".to_string()),
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_json() {
        std::env::set_var(LOG_FORMAT_ENV_KEY, "json");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Json,
                disable_envlogger: false,
                envlogger_filters: None,
                envlogger_override_default_filter: Some("trace".to_string()),
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_unset() {
        std::env::remove_var(LOG_FORMAT_ENV_KEY);
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_envlogger: false,
                envlogger_filters: None,
                envlogger_override_default_filter: Some("trace".to_string()),
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_empty() {
        std::env::set_var(LOG_FORMAT_ENV_KEY, "");
        assert_matches!(
            config_from_env(),
            Err(ConfigFromEnvError::InvalidLogFormat(
                ParseFromEnvError::ParseError{
                    ref string, ..
                }
            )) if string == ""
        );
    }

    #[serial]
    #[test]
    fn log_format_invalid() {
        std::env::set_var(LOG_FORMAT_ENV_KEY, "invalid");
        assert_matches!(
            config_from_env(),
            Err(ConfigFromEnvError::InvalidLogFormat(
                ParseFromEnvError::ParseError{
                    ref string, ..
                }
            )) if string == "invalid"
        );
    }
}
