//! Primary error structures for babybatch.
use std::error::Error;
use std::fmt;
use std::io;
use std::sync::mpsc;

use crate::comm::*;

/// Error types for babybatch.
#[derive(Debug)]
pub enum BabyBatchError {
    Io(std::io::Error),
    SendError(mpsc::SendError<Message>),
    RewqestError(reqwest::Error),
    NoNextPageToken,
    NoResource,
}

impl Error for BabyBatchError {}

/// Error conversions From<_> for `BabyBatchError`s.
impl From<io::Error> for BabyBatchError {
    fn from(err: io::Error) -> BabyBatchError {
        BabyBatchError::Io(err)
    }
}

impl From<reqwest::Error> for BabyBatchError {
    fn from(err: reqwest::Error) -> BabyBatchError {
        BabyBatchError::RewqestError(err)
    }
}

impl From<mpsc::SendError<Message>> for BabyBatchError {
    fn from(err: mpsc::SendError<Message>) -> BabyBatchError {
        BabyBatchError::SendError(err)
    }
}

impl fmt::Display for BabyBatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BabyBatchError::Io(e) => write!(f, "{}", e),
            BabyBatchError::NoNextPageToken => {
                write!(f, "{}", stringify!(BabyBatchError::NoNextPageToken))
            }
            BabyBatchError::RewqestError(e) => write!(f, "{}", e),
            BabyBatchError::SendError(e) => write!(f, "{}", e),
            BabyBatchError::NoResource => write!(f, "{}", stringify!(BabyBatchError::NoResource)),
        }
    }
}

/// Custom result type for babybatch.
pub type Result<T> = std::result::Result<T, BabyBatchError>;
