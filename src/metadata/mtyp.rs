// This file is part of the NRGrip project.
//
// Copyright (c) 2016, 2018 Matteo Cypriani <mcy@lm7.fr>
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

//! NRG MTYP chunk data structure and associated functions.

use std::fmt;
use std::fs::File;

use ::error::NrgError;
use super::readers::read_u32;


#[derive(Copy, Clone, Debug)]
pub struct NrgMtyp {
    pub size: u32,
    pub unknown: u32,
}

impl NrgMtyp {
    pub fn new() -> NrgMtyp {
        NrgMtyp {
            size: 0,
            unknown: 0,
        }
    }
}

impl fmt::Display for NrgMtyp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chunk ID: MTYP\n\
                   Chunk description: Media Type (?)\n\
                   Chunk size: {} Bytes\n\
                   Unknown field: 0x{:04X}",
               self.size,
               self.unknown)
    }
}


/// Reads the Media Type (?) chunk (MTYP).
pub fn read_nrg_mtyp(fd: &mut File) -> Result<NrgMtyp, NrgError> {
    let mut chunk = NrgMtyp::new();
    chunk.size = try!(read_u32(fd));
    chunk.unknown = try!(read_u32(fd));
    Ok(chunk)
}
