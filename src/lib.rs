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

mod error;
pub use error::*;

mod env;
pub use env::*;

mod config;
pub use config::*;

mod standard;
pub use standard::*;
