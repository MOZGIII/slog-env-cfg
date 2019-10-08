use crate::*;
use std::convert::Infallible;
use std::str::ParseBoolError;

#[derive(Debug)]
pub enum ConfigFromEnvError {
    InvalidLogFormat(ParseFromEnvError<InvalidLogFormat>),
    InvalidBool(ParseFromEnvError<ParseBoolError>),
    InvalidInfallible(ParseFromEnvError<Infallible>),
}

impl From<ParseFromEnvError<InvalidLogFormat>> for ConfigFromEnvError {
    fn from(err: ParseFromEnvError<InvalidLogFormat>) -> Self {
        ConfigFromEnvError::InvalidLogFormat(err)
    }
}

impl From<ParseFromEnvError<ParseBoolError>> for ConfigFromEnvError {
    fn from(err: ParseFromEnvError<ParseBoolError>) -> Self {
        ConfigFromEnvError::InvalidBool(err)
    }
}

impl From<ParseFromEnvError<Infallible>> for ConfigFromEnvError {
    fn from(err: ParseFromEnvError<Infallible>) -> Self {
        ConfigFromEnvError::InvalidInfallible(err)
    }
}

impl std::fmt::Display for ConfigFromEnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigFromEnvError::InvalidLogFormat(err) => err.fmt(f),
            ConfigFromEnvError::InvalidBool(err) => err.fmt(f),
            ConfigFromEnvError::InvalidInfallible(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ConfigFromEnvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigFromEnvError::InvalidLogFormat(err) => Some(err),
            ConfigFromEnvError::InvalidBool(err) => Some(err),
            ConfigFromEnvError::InvalidInfallible(err) => Some(err),
        }
    }
}
