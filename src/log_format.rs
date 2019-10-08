use std::str::FromStr;

/// Supported log formats.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogFormat {
    /// Output logs in human-readable form.
    Terminal,

    /// Output logs in JSON format (aka JSON Lines).
    Json,
}

impl FromStr for LogFormat {
    type Err = InvalidLogFormat;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        match item.to_lowercase().as_ref() {
            "terminal" | "term" => Ok(LogFormat::Terminal),
            "json" => Ok(LogFormat::Json),
            _ => Err(InvalidLogFormat),
        }
    }
}

/// `InvalidLogFormat` is an error returned on a `LogFormat` parse attempt when
/// an invalid logger format name is passed.
#[derive(Debug)]
pub struct InvalidLogFormat;

impl std::fmt::Display for InvalidLogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid log format")
    }
}

impl std::error::Error for InvalidLogFormat {}
