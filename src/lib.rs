//! Example
//!
//! ```no_run
//! # #[macro_use]
//! # extern crate slog;
//! # fn main() -> Result<(), Box<std::error::Error>> {
//! // Read configuration from environment.
//! let cfg = slog_env_cfg::config_from_env()?;
//! // Build a slog drain using the provided configuration.
//! let drain = cfg.build();
//! // Use the darin to create a logger as usual.
//! let root = slog::Logger::root(drain, o!());
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

pub use slog::Drain;

pub type Logger = slog::Logger<dyn slog::Drain<Ok = (), Err = slog::Never>>;
