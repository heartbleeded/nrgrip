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

//! NRG SINF chunk data structure and associated functions.

use std::fmt;
use std::fs::File;

use ::error::NrgError;
use super::readers::read_u32;


#[derive(Debug)]
pub struct NrgSinf {
    pub size: u32,
    pub nb_tracks: u32,
}

impl NrgSinf {
    pub fn new() -> NrgSinf {
        NrgSinf {
            size: 0,
            nb_tracks: 0,
        }
    }
}

impl fmt::Display for NrgSinf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chunk ID: SINF\n\
                   Chunk description: Session Information\n\
                   Chunk size: {} Bytes\n\
                   Number of tracks in the session: {}",
               self.size,
               self.nb_tracks)
    }
}


/// Reads the NRG Session Information chunk (SINF).
pub fn read_nrg_sinf(fd: &mut File) -> Result<NrgSinf, NrgError> {
    let mut chunk = NrgSinf::new();
    chunk.size = try!(read_u32(fd));
    chunk.nb_tracks = try!(read_u32(fd));
    Ok(chunk)
}
