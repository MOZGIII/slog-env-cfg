//! Example
//!
//! ```no_run
//! # use slog::{info, o};
//! # fn main() -> Result<(), Box<std::error::Error>> {
//! // Read the configuration from environment and build a logger.
//! let root = slog_env_cfg::logger_from_env(o!())?;
//! // Log something!
//! info!(root, "formatted: {}", 1; "log-key" => true);
//! # Ok(())
//! # }
//! ```

#![warn(rust_2018_idioms)]

mod log_format;
pub use log_format::*;

mod config;
pub use config::*;

mod config_from_env;
pub use config_from_env::*;

mod convenience;
pub use convenience::*;

mod drain;
mod parse_from_env;
