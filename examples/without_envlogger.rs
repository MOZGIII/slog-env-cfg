//! This example demonstrates how to force-disable envlogger.
//! You may require this for various reasons.
//!
//! To test the behavior, run this example with `RUST_LOG=error`.
//! You should see that envlogger does no longer filter the logs.

use slog::{info, o};

fn main() {
    let mut cfg = slog_env_cfg::config_from_env().expect("initialization error");
    cfg.disable_env_logger = true;
    let drain = cfg.build();
    let root = slog::Logger::root(drain, o!());
    info!(root, "formatted: {}", 1; "log-key" => true);
}
