use slog::{info, o};

fn main() {
    let root = slog_env_cfg::logger_from_env(o!()).expect("initialization error");
    info!(root, "formatted: {}", 1; "log-key" => true);
}
