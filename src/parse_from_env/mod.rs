use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::str::FromStr;

mod error;
pub use error::*;

/// Parse a value from an env var.
pub fn parse_from_env<K, V>(key: K) -> Result<Option<V>, ParseFromEnvError<V::Err>>
where
    K: AsRef<OsStr>,
    V: FromStr,
    <V as FromStr>::Err: Error,
{
    match env::var(key) {
        Ok(val) => match val.parse() {
            Ok(format) => Ok(Some(format)),
            Err(err) => Err(ParseFromEnvError::ParseError {
                string: val,
                source: err,
            }),
        },
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(val)) => Err(ParseFromEnvError::NotUnicode(val)),
    }
}
