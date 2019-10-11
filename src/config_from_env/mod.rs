use crate::parse_from_env::parse_from_env;
use crate::*;

mod error;
pub use error::*;

pub const LOG_FORMAT_ENV_KEY: &'static str = "LOG_FORMAT";
pub const DISABLE_ENVLOGGER_ENV_KEY: &'static str = "DISABLE_ENVLOGGER";
pub const ENVLOGGER_FILTERS_ENV_KEY: &'static str = "RUST_LOG";

pub const ENVLOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT: &'static str = "debug";

/// Build `Config` using the env vars and opinionated defaults.
pub fn config_from_env() -> Result<Config, ConfigFromEnvError> {
    let format = parse_from_env(LOG_FORMAT_ENV_KEY)?.unwrap_or(LogFormat::Terminal);
    let disable_envlogger = parse_from_env(DISABLE_ENVLOGGER_ENV_KEY)?.unwrap_or(false);
    let envlogger_filters = parse_from_env(ENVLOGGER_FILTERS_ENV_KEY)?;
    let envlogger_override_default_filter =
        Some(ENVLOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string());
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
    use crate::parse_from_env::ParseFromEnvError;
    use matches::assert_matches;
    use serial_test_derive::serial;

    fn reset_env() {
        std::env::remove_var(LOG_FORMAT_ENV_KEY);
        std::env::remove_var(DISABLE_ENVLOGGER_ENV_KEY);
        std::env::remove_var(ENVLOGGER_FILTERS_ENV_KEY);
    }

    #[serial]
    #[test]
    fn defaults() {
        reset_env();
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_envlogger: false,
                envlogger_filters: None,
                envlogger_override_default_filter: Some(
                    ENVLOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()
                ),
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_term() {
        reset_env();
        std::env::set_var(LOG_FORMAT_ENV_KEY, "term");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_envlogger: false,
                envlogger_filters: None,
                envlogger_override_default_filter: Some(
                    ENVLOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()
                ),
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_json() {
        reset_env();
        std::env::set_var(LOG_FORMAT_ENV_KEY, "json");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Json,
                disable_envlogger: false,
                envlogger_filters: None,
                envlogger_override_default_filter: Some(
                    ENVLOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()
                ),
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_unset() {
        reset_env();
        std::env::remove_var(LOG_FORMAT_ENV_KEY);
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_envlogger: false,
                envlogger_filters: None,
                envlogger_override_default_filter: Some(
                    ENVLOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()
                ),
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_empty() {
        reset_env();
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
        reset_env();
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

    #[serial]
    #[test]
    fn envlogger_filters_empty() {
        reset_env();
        std::env::set_var(ENVLOGGER_FILTERS_ENV_KEY, "");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_envlogger: false,
                envlogger_filters: Some("".to_string()),
                envlogger_override_default_filter: Some(ENVLOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()),
            }
        );
    }

    #[serial]
    #[test]
    fn envlogger_filters_debug() {
        reset_env();
        std::env::set_var(ENVLOGGER_FILTERS_ENV_KEY, "debug");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_envlogger: false,
                envlogger_filters: Some("debug".to_string()),
                envlogger_override_default_filter: Some(
                    ENVLOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()
                ),
            }
        );
    }
}
