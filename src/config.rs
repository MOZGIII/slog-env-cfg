use crate::util::drain_async_fuse;
use crate::InvalidLogFormat;
use std::str::FromStr;

// Re-export the `slog` API parts that appear in our API.
pub use slog::{Drain, SendSyncRefUnwindSafeDrain};

/// Supported log formats.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogFormat {
    Terminal,
    Json,
}

impl FromStr for LogFormat {
    type Err = InvalidLogFormat;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        match item.to_lowercase().as_ref() {
            "terminal" | "term" => Ok(LogFormat::Terminal),
            "json" => Ok(LogFormat::Json),
            _ => Err(InvalidLogFormat),
        }
    }
}

/// Holds the configuration parameters.
/// Used to build `Drain`s.
#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub format: LogFormat,
}

impl Config {
    /// Build a `Drain` according to the specified parameters.
    /// The resulting `Drain` can be wrapped with `slog_async::Async` and
    /// `fuse`'d to be ready for passing it to the `slog::Logger::root`.
    /// For convenience, we also provide the `build_ready` method that takes
    /// care of that.
    pub fn build(&self) -> Box<dyn Drain<Ok = (), Err = slog::Never> + Send> {
        match self.format {
            LogFormat::Terminal => Box::new(self.build_terminal()),
            LogFormat::Json => Box::new(self.build_json()),
        }
    }

    /// Build a `Drain` according to the specified parameters and wrap it to
    /// prepare for use with `slog::Logger::root`.
    /// Use this function for convenience, or `build` for a lower level.
    pub fn build_ready(&self) -> impl SendSyncRefUnwindSafeDrain<Ok = (), Err = slog::Never> {
        drain_async_fuse(self.build())
    }

    fn build_terminal(&self) -> impl Drain<Ok = (), Err = slog::Never> {
        let decorator = slog_term::TermDecorator::new().stdout().build();
        slog_term::CompactFormat::new(decorator).build().fuse()
    }

    fn build_json(&self) -> impl Drain<Ok = (), Err = slog::Never> {
        slog_json::Json::default(std::io::stdout()).fuse()
    }
}

#[test]
fn build_test_terminal() {
    let cfg = Config {
        format: LogFormat::Terminal,
    };
    cfg.build();
}

#[test]
fn build_test_json() {
    let cfg = Config {
        format: LogFormat::Json,
    };
    cfg.build();
}
