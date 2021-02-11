
use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Hash,Clone)]
pub enum Error {
    Message(String),
    InvalidUtf8 {
        valid_up_to: usize,
        error_len: Option<usize>
    },
    IoError(String)
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::InvalidUtf8 { valid_up_to, error_len } => formatter.write_str(
                &format!("Invalid UTF-8; valid up to: {}, error length: {}",
                         valid_up_to,
                         error_len.map(|x| x.to_string()).unwrap_or("EOF".to_string()))),
            Error::IoError(msg) => formatter.write_str(msg)
        }
    }
}

impl std::error::Error for Error {}

impl std::convert::From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Error {
        Error::InvalidUtf8 { valid_up_to: error.valid_up_to(), error_len: error.error_len() }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IoError(error.to_string())
    }
}
