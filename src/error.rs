use std::ffi::OsString;

/// InvalidLogFormat is an error returned on a LogFormat parse attempt when an
/// invalid logger format name is passed.
#[derive(Debug)]
pub struct InvalidLogFormat;

impl std::fmt::Display for InvalidLogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid log format")
    }
}

impl std::error::Error for InvalidLogFormat {}

/// LogFormatFromEnvError captures all possible errors that can occur when
/// log format is constructred from the system environment variables.
#[derive(Debug)]
pub enum LogFormatFromEnvError {
    NotUnicode(OsString),
    InvalidFormat(String),
}

impl std::fmt::Display for LogFormatFromEnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogFormatFromEnvError::NotUnicode(s) => {
                write!(f, "environment variable was not valid unicode: {:?}", s)
            }
            LogFormatFromEnvError::InvalidFormat(s) => {
                write!(f, "environment variable was not a valid format name: {}", s)
            }
        }
    }
}

impl std::error::Error for LogFormatFromEnvError {}
