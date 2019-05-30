#![warn(rust_2018_idioms)]

mod env;
pub use env::*;

mod config;
pub use config::*;

pub use slog::Drain;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
