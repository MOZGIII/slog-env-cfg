use crate::parse_from_env::parse_from_env;
use crate::*;

mod error;
pub use error::*;

pub mod env_key {
    pub const LOG_FORMAT: &'static str = "LOG_FORMAT";
    pub const DISABLE_ENV_LOGGER: &'static str = "DISABLE_ENV_LOGGER";
    pub const ENV_LOGGER_FILTERS: &'static str = "RUST_LOG";
}

pub const ENV_LOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT: &'static str = "debug";

/// Build `Config` using the env vars and opinionated defaults.
pub fn config_from_env() -> Result<Config, ConfigFromEnvError> {
    let format = parse_from_env(env_key::LOG_FORMAT)?.unwrap_or(LogFormat::Terminal);
    let disable_env_logger = parse_from_env(env_key::DISABLE_ENV_LOGGER)?.unwrap_or(false);
    let env_logger_filters = parse_from_env(env_key::ENV_LOGGER_FILTERS)?;
    let env_logger_override_default_filter =
        Some(ENV_LOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string());
    Ok(Config {
        format,
        disable_env_logger,
        env_logger_filters,
        env_logger_override_default_filter,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_from_env::ParseFromEnvError;
    use matches::assert_matches;
    use serial_test_derive::serial;

    fn reset_env() {
        std::env::remove_var(env_key::LOG_FORMAT);
        std::env::remove_var(env_key::DISABLE_ENV_LOGGER);
        std::env::remove_var(env_key::ENV_LOGGER_FILTERS);
    }

    #[serial]
    #[test]
    fn defaults() {
        reset_env();
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_env_logger: false,
                env_logger_filters: None,
                env_logger_override_default_filter: Some(
                    ENV_LOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()
                ),
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_term() {
        reset_env();
        std::env::set_var(env_key::LOG_FORMAT, "term");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_env_logger: false,
                env_logger_filters: None,
                env_logger_override_default_filter: Some(
                    ENV_LOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()
                ),
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_json() {
        reset_env();
        std::env::set_var(env_key::LOG_FORMAT, "json");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Json,
                disable_env_logger: false,
                env_logger_filters: None,
                env_logger_override_default_filter: Some(
                    ENV_LOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()
                ),
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_unset() {
        reset_env();
        std::env::remove_var(env_key::LOG_FORMAT);
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_env_logger: false,
                env_logger_filters: None,
                env_logger_override_default_filter: Some(
                    ENV_LOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()
                ),
            }
        );
    }

    #[serial]
    #[test]
    fn log_format_empty() {
        reset_env();
        std::env::set_var(env_key::LOG_FORMAT, "");
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
        std::env::set_var(env_key::LOG_FORMAT, "invalid");
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
    fn env_logger_filters_empty() {
        reset_env();
        std::env::set_var(env_key::ENV_LOGGER_FILTERS, "");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_env_logger: false,
                env_logger_filters: Some("".to_string()),
                env_logger_override_default_filter: Some(ENV_LOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()),
            }
        );
    }

    #[serial]
    #[test]
    fn env_logger_filters_debug() {
        reset_env();
        std::env::set_var(env_key::ENV_LOGGER_FILTERS, "debug");
        assert_eq!(
            config_from_env().unwrap(),
            Config {
                format: LogFormat::Terminal,
                disable_env_logger: false,
                env_logger_filters: Some("debug".to_string()),
                env_logger_override_default_filter: Some(
                    ENV_LOGGER_OVERRIDE_DEFAULT_FILTER_DEFAULT.to_string()
                ),
            }
        );
    }
}
