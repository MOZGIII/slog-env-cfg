# slog-env-cfg

[![crates.io](https://img.shields.io/crates/v/slog-env-cfg.svg)](https://crates.io/crates/slog-env-cfg)
[![docs.rs](https://docs.rs/slog-env-cfg/badge.svg)](https://docs.rs/slog-env-cfg)

Opinionated slog drains builder, configurable via env vars.

## Usage

This crate builds an [slog `Drain`](https://docs.rs/slog/2/slog/trait.Drain.html) that writes to stdout. It's designed to be easy to use and with sane defaults. The format of the log records is configurable at runtime via `LOG_FORMAT` environment variable.

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
