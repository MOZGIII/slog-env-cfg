#[macro_use]
extern crate slog;

fn main() {
    let drain = slog_env_cfg::drain_from_env().expect("initialization error");
    let root = slog::Logger::root(drain, o!());
    info!(root, "formatted: {}", 1; "log-key" => true);
}
