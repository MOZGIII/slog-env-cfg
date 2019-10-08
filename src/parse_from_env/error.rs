use std::error::Error;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter, Result};

/// `ParseFromEnvError` captures all possible errors that can occur when
/// value is constructred from the system environment variables.
#[derive(Debug)]
pub enum ParseFromEnvError<T>
where
    T: Debug,
{
    NotUnicode(OsString),
    ParseError { string: String, source: T },
}

impl<T> Display for ParseFromEnvError<T>
where
    T: Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ParseFromEnvError::NotUnicode(s) => {
                write!(f, "environment variable was not valid unicode: {:?}", s)
            }
            ParseFromEnvError::ParseError { source, string } => write!(
                f,
                "environment variable value could not be parsed: {} (value was  {:?})",
                source, string,
            ),
        }
    }
}

impl<T> Error for ParseFromEnvError<T>
where
    T: Debug + Display + Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseFromEnvError::ParseError { source, .. } => Some(source),
            ParseFromEnvError::NotUnicode(_) => None,
        }
    }
}
