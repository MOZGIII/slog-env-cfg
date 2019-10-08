use crate::*;
use std::sync::Arc;

// Re-export the `slog` API parts that appear in our API.
pub use slog::SendSyncRefUnwindSafeDrain;

pub const STANDARD_LOG_FORMAT_ENV_KEY: &'static str = "LOG_FORMAT";

/// Build `Config` using the `STANDARD_LOG_FORMAT_ENV_KEY` env var.
pub fn config_from_env() -> Result<Config, LogFormatFromEnvWithDefaultError> {
    let format =
        log_format_from_env_with_default(STANDARD_LOG_FORMAT_ENV_KEY, LogFormat::Terminal)?;
    Ok(Config { format })
}

/// Build slog `Drain` (more specificly `SendSyncRefUnwindSafeDrain`)
/// using the `STANDARD_LOG_FORMAT_ENV_KEY` env var.
/// The resulting `Drain` is ready to be passed to the logger (i.e. it is
/// wrapped with `slog_async::Async`). To build a raw, unwrapped `Drain` use
/// `config_from_env` and manually drive the build process from there.
pub fn drain_from_env() -> Result<
    impl SendSyncRefUnwindSafeDrain<Ok = (), Err = slog::Never>,
    LogFormatFromEnvWithDefaultError,
> {
    Ok(config_from_env()?.build_ready())
}

/// `Logger` is a convenience type alias for the `slog::Logger`.
/// You can use it to pass around the `Logger` built with this crate in your
/// app code.
pub type Logger =
    slog::Logger<Arc<dyn slog::SendSyncRefUnwindSafeDrain<Ok = (), Err = slog::Never>>>;

/// Build slog `Logger` using the `STANDARD_LOG_FORMAT_ENV_KEY` env var.
pub fn logger_from_env<T>(
    values: slog::OwnedKV<T>,
) -> Result<Logger, LogFormatFromEnvWithDefaultError>
where
    T: slog::SendSyncRefUnwindSafeKV + 'static,
{
    let drain = drain_from_env()?;
    Ok(slog::Logger::root(drain, values))
}

#[cfg(test)]
mod test {
    use super::*;
    use matches::assert_matches;
    use serial_test_derive::serial;

    #[serial]
    #[test]
    fn log_format_term() {
        std::env::set_var(STANDARD_LOG_FORMAT_ENV_KEY, "term");
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
        std::env::set_var(STANDARD_LOG_FORMAT_ENV_KEY, "json");
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
        std::env::remove_var(STANDARD_LOG_FORMAT_ENV_KEY);
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
        std::env::set_var(STANDARD_LOG_FORMAT_ENV_KEY, "");
        assert_matches!(
            config_from_env(),
            Err(LogFormatFromEnvWithDefaultError::InvalidFormat(ref s)) if s == ""
        );
    }

    #[serial]
    #[test]
    fn log_format_invalid() {
        std::env::set_var(STANDARD_LOG_FORMAT_ENV_KEY, "invalid");
        assert_matches!(
            config_from_env(),
            Err(LogFormatFromEnvWithDefaultError::InvalidFormat(ref s)) if s == "invalid"
        );
    }
}
