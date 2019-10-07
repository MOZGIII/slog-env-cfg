//! Example
//!
//! ```no_run
//! # #[macro_use]
//! # extern crate slog;
//! # fn main() -> Result<(), Box<std::error::Error>> {
//! // Read the configuration from environment and build slog drain.
//! let drain = slog_env_cfg::drain_from_env()?;
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
