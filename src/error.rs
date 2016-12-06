//! Error type for NRGrip.

use std::error::Error;
use std::fmt;
use std::io;


#[derive(Debug)]
pub enum NrgError {
    Io(io::Error),
    NrgFormat,
    NrgFormatV1,
    NrgChunkId,
}

impl fmt::Display for NrgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NrgError::Io(ref err) => err.fmt(f),
            NrgError::NrgFormat => write!(f, "NRG format unknown."),
            NrgError::NrgFormatV1 => write!(f, "NRG v1 format is not handled."),
            NrgError::NrgChunkId => write!(f, "NRG chunk ID unknown."),
        }
    }
}

impl Error for NrgError {
    fn description(&self) -> &str {
        match *self {
            NrgError::Io(ref err) => err.description(),
            NrgError::NrgFormat => "NRG format",
            NrgError::NrgFormatV1 => "NRG format v1",
            NrgError::NrgChunkId => "NRG chunk ID",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            NrgError::Io(ref err) => Some(err),
            NrgError::NrgFormat => None,
            NrgError::NrgFormatV1 => None,
            NrgError::NrgChunkId => None,
        }
    }
}

impl From<io::Error> for NrgError {
    fn from(err: io::Error) -> NrgError {
        NrgError::Io(err)
    }
}
