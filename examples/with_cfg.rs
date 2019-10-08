use slog::{info, o};

fn main() {
    let cfg = slog_env_cfg::config_from_env().expect("initialization error");
    let drain = cfg.build_ready();
    let root = slog::Logger::root(drain, o!());
    info!(root, "formatted: {}", 1; "log-key" => true);
}
