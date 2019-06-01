# slog-env-cfg

[![crates.io](https://img.shields.io/crates/v/slog-env-cfg.svg)](https://crates.io/crates/slog-env-cfg)
[![docs.rs](https://docs.rs/slog-env-cfg/badge.svg)](https://docs.rs/slog-env-cfg)
[![Build Status](https://travis-ci.org/MOZGIII/slog-env-cfg.svg?branch=master)](https://travis-ci.org/MOZGIII/slog-env-cfg)

Opinionated slog drains builder, configurable via env vars.

## Description

This crate builds an [slog `Drain`](https://docs.rs/slog/2/slog/trait.Drain.html) that writes to stdout. It's designed to be easy to use and with sane defaults. The format of the log records is configurable at runtime via `LOG_FORMAT` environment variable.

Possible values for `LOG_FORMAT` are:

- `terminal` (default)
- `json`

The idea is that in development environment, where you just run the executable manually and in the absense of centralized logging,
you would rely on human-readable log output. At deployment, on the contrary, cenralized logging system can make use of JSON-formatted
logs, so the `LOG_FORMAT=json` can be set. The configuration is at runtime, so you only need to manage a single build artifact for both
use cases.

This crate aims to be an opinionated solution, and no other configuration is available so far.
Yet, I'm open to adding other tweaks if they're justified.

## Usage

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
