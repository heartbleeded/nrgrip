// This file is part of the NRGrip project.
//
// Copyright (c) 2016 Matteo Cypriani <mcy@lm7.fr>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

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
    NoNrgCue,
    AudioReadError,
    AudioWriteError
}

impl fmt::Display for NrgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NrgError::Io(ref err) => err.fmt(f),
            NrgError::NrgFormat => write!(f, "NRG format unknown."),
            NrgError::NrgFormatV1 => write!(f, "NRG v1 format is not handled."),
            NrgError::NrgChunkId => write!(f, "NRG chunk ID unknown."),
            NrgError::NoNrgCue => write!(f, "NRG cue sheet chunk absent."),
            NrgError::AudioReadError => write!(f, "Error reading raw audio."),
            NrgError::AudioWriteError => write!(f, "Error writing raw audio."),
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
            NrgError::NoNrgCue => "No NRG cue",
            NrgError::AudioReadError => "Audio read error",
            NrgError::AudioWriteError => "Audio write error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            NrgError::Io(ref err) => Some(err),
            NrgError::NrgFormat => None,
            NrgError::NrgFormatV1 => None,
            NrgError::NrgChunkId => None,
            NrgError::NoNrgCue => None,
            NrgError::AudioReadError => None,
            NrgError::AudioWriteError => None,
        }
    }
}

impl From<io::Error> for NrgError {
    fn from(err: io::Error) -> NrgError {
        NrgError::Io(err)
    }
}
