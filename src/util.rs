use slog::{Drain, Fuse};
use slog_async::Async;

/// Wraps drain with `slog_async::Async` and fuses it.
#[inline]
pub fn drain_async_fuse<D>(drain: D) -> Fuse<Async>
where
    D: slog::Drain<Err = slog::Never, Ok = ()> + Send + 'static,
{
    Async::new(drain).build().fuse()
}
