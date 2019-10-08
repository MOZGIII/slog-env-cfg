//! This example demonstrates how to use `slog-env-cfg` and `slog-envlogger`
//! together.
//! You can configure both the log output format (with `slog-env-cfg`) and
//! the log verbosity (with `slog-envlogger`) via environment variables.
//!
//! Run this example with `RUST_LOG=debug` to see the output.

use slog::{info, o};

fn main() {
    let cfg = slog_env_cfg::config_from_env().expect("initialization error");
    let drain = cfg.build();
    let drain = slog_envlogger::new(drain);
    let drain = slog_env_cfg::util::drain_async_fuse(drain);
    let root = slog::Logger::root(drain, o!());
    info!(root, "formatted: {}", 1; "log-key" => true);
}
