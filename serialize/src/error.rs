// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use serde::{de, ser};
use core::fmt;
use alloc::string::{String, ToString};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg(feature = "std")]
pub type IoError = std::io::Error;

#[cfg(not(feature = "std"))]
pub struct IoError(pub String);

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Eof,
    Io(String),
    ExceededMaxLen(usize),
    ExceededContainerDepthLimit(&'static str),
    ExpectedBoolean,
    ExpectedMapKey,
    ExpectedMapValue,
    NonCanonicalMap,
    ExpectedOption,
    Custom(String),
    MissingLen,
    NotSupported(&'static str),
    RemainingInput,
    Utf8,
    NonCanonicalUleb128Encoding,
    IntegerOverflowDuringUleb128Decoding,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[cfg(feature = "std")]
impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err.to_string())
    }
}

#[cfg(not(feature = "std"))]
impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err.0)
    }
}

#[cfg(not(feature = "std"))]
impl<'a> From<&'a str> for IoError {
    fn from(s: &'a str) -> IoError {
        IoError(s.into())
    }
}

impl ser::StdError for Error { }

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
