use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ConfigurationError {
    MissingVar(String),
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::MissingVar(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl Error for ConfigurationError {}
