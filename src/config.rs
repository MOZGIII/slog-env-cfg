use crate::drain::EitherDrain;
use crate::util::drain_async_fuse;
use crate::*;
use slog::{Drain, Never, SendSyncRefUnwindSafeDrain};

/// Holds the configuration parameters.
/// Used to build `Drain`s.
#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    /// Log format to provide.
    pub format: LogFormat,

    /// Disable envlogger.
    /// Yuo might want this for performance reasons or just for convenience:
    /// envlogger hides log values by default, and turning the whole envlogger
    /// off is a simple wordaround for that behavior.
    pub disable_envlogger: bool,

    /// Envlogger configuration to use (usually passed via `RUST_LOG` env var).
    pub envlogger_filters: Option<String>,

    /// If no configuration is passed to envlogger, it adds the "error" filter
    /// by default. This allows for overriding this default with something
    /// more suitable.
    pub envlogger_override_default_filter: Option<String>,
}

impl Config {
    /// Build a `Drain` according to the specified parameters.
    /// The resulting `Drain` is ready to use with `slog::Logger::root`.
    #[inline]
    pub fn build(&self) -> impl SendSyncRefUnwindSafeDrain<Ok = (), Err = Never> {
        drain_async_fuse(self.wrap_with_envlogger(self.build_format_drain()))
    }

    #[inline]
    fn wrap_with_envlogger<D>(&self, drain: D) -> EitherDrain<D, slog_envlogger::EnvLogger<D>>
    where
        D: Drain<Ok = (), Err = Never>,
    {
        if self.disable_envlogger {
            return EitherDrain::Left(drain);
        }

        let mut builder = slog_envlogger::LogBuilder::new(drain);
        match self.envlogger_filters {
            Some(ref val) if val != "" => builder = builder.parse(val),
            _ => {
                if let Some(ref val) = self.envlogger_override_default_filter {
                    builder = builder.parse(val)
                }
            }
        }

        EitherDrain::Right(builder.build())
    }

    #[inline]
    fn build_format_drain(
        &self,
    ) -> EitherDrain<impl Drain<Ok = (), Err = Never>, impl Drain<Ok = (), Err = Never>> {
        match self.format {
            LogFormat::Terminal => EitherDrain::Left(self.build_terminal()),
            LogFormat::Json => EitherDrain::Right(self.build_json()),
        }
    }

    #[inline]
    fn build_terminal(&self) -> impl Drain<Ok = (), Err = Never> {
        let decorator = slog_term::TermDecorator::new().stdout().build();
        slog_term::CompactFormat::new(decorator).build().fuse()
    }

    #[inline]
    fn build_json(&self) -> impl Drain<Ok = (), Err = Never> {
        slog_json::Json::default(std::io::stdout()).fuse()
    }
}

#[test]
fn build_test_terminal() {
    let cfg = Config {
        format: LogFormat::Terminal,
        disable_envlogger: false,
        envlogger_filters: None,
        envlogger_override_default_filter: None,
    };
    cfg.build();
}

#[test]
fn build_test_json() {
    let cfg = Config {
        format: LogFormat::Json,
        disable_envlogger: false,
        envlogger_filters: None,
        envlogger_override_default_filter: None,
    };
    cfg.build();
}
