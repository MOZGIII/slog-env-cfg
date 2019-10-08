//! This mod provides the public APIs for this crate's user convenience.
//! Utility constructors with everything beyond the `config_from_env` would go
//! here.

use crate::*;
use slog::{Never, OwnedKV, SendSyncRefUnwindSafeDrain};
use std::sync::Arc;

/// Build slog `Drain` (more specificly `SendSyncRefUnwindSafeDrain`)
/// using the config obtained from env.
/// The resulting `Drain` is ready to be passed to the `slog::Logger::root`.
pub fn drain_from_env(
) -> Result<impl SendSyncRefUnwindSafeDrain<Ok = (), Err = Never>, ConfigFromEnvError> {
    Ok(config_from_env()?.build())
}

/// `Logger` is a convenience type alias for the `slog::Logger`.
/// You can use it to pass around the `Logger` built with this crate in your
/// app code.
pub type Logger = slog::Logger<Arc<dyn SendSyncRefUnwindSafeDrain<Ok = (), Err = Never>>>;

/// Build slog `Logger` using the config obtained from env.
pub fn logger_from_env<T>(values: OwnedKV<T>) -> Result<Logger, ConfigFromEnvError>
where
    T: slog::SendSyncRefUnwindSafeKV + 'static,
{
    let drain = drain_from_env()?;
    Ok(slog::Logger::root(drain, values))
}
