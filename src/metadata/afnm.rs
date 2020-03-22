// This file is part of the NRGrip project.
//
// Copyright (c) 2020 heartbleeded
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

//! NRG AFNM chunk data structure and associated functions.

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use ::error::NrgError;
use super::readers::*;


#[derive( Debug)]
pub struct NrgAfnm {
    pub size: u32,
    pub tracks: Vec<NrgAfnmTrack>,
}

impl NrgAfnm {
    pub fn new() -> NrgAfnm {
        NrgAfnm {
            size: 0,
            tracks: Vec::new(),
        }
    }
}

impl fmt::Display for NrgAfnm {
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Chunk ID: AFNM\n\
                        Chunk size: {} Bytes", self.size));
        if self.tracks.is_empty() {
            try!(write!(f, "\nNo AFNM tracks!"));
        } else {
            for track in &self.tracks {
                try!(write!(f, "\n\
                                Track:\n\
                                {}", track));
            }
        }
        Ok(())
    }
}

#[derive( Clone, Debug)]
pub struct NrgAfnmTrack {
    pub name: String,
}

impl NrgAfnmTrack {
    pub fn new() -> NrgAfnmTrack {
        NrgAfnmTrack {
            name: String::new(),
        }
    }
}

impl fmt::Display for NrgAfnmTrack {
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Chunk name: {} \n", self.name));
        Ok(())
    }
}

/// Reads the Media Type (?) chunk (AFNM).
pub fn read_nrg_afnm(fd: &mut File) -> Result<NrgAfnm, NrgError> {
    let mut chunk = NrgAfnm::new();
    chunk.size = try!(read_u32(fd));
    let mut bytes_read = 0;
    let mut name = String::new();;
    let mut track = NrgAfnmTrack::new();
    while bytes_read < chunk.size{
        let mut buffer = [0; 1];
        try!(fd.read_exact(&mut buffer));
        if buffer[0] == 0 {
            println!("{:?}", name);
            track.name = name;
            chunk.tracks.push(track);
            track = NrgAfnmTrack::new();
            name = String::new();
        }else{
            name.push(buffer[0] as char);
        }
        bytes_read+=1;
    }
    assert_eq!(bytes_read, chunk.size);
    Ok(chunk)
}
