# slog-env-cfg

![https://crates.io/crates/slog-env-cfg](https://img.shields.io/crates/v/slog-env-cfg.svg)
![https://docs.rs/slog-env-cfg](https://docs.rs/slog-env-cfg/badge.svg)

Opinionated slog drains builder, configurable via env vars.

## Usage

Logging is done to stdout, and configure at runtime via `LOG_FORMAT` environment variable.

Possible values for `LOG_FORMAT` are:

- `json`
- `terminal` (default)

This crate aims to be an opinionated solution, and no other configuration is available so far.
Yet, I'm open to adding other tweaks if they're justified.

Here's a minimal complete `main.rs` example:

```rust
#[macro_use]
extern crate slog;

fn main() {
    let cfg = slog_env_cfg::config_from_env().expect("initialization error");
    let drain = cfg.build();
    let root = slog::Logger::root(drain, o!());
    info!(root, "formatted: {}", 1; "log-key" => true);
}
```
