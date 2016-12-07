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

//! NRG metadata structure, storing the contents of all of the NRG chunks.

use std::fmt;

use super::cuex::NrgCuex;
use super::daox::NrgDaox;
use super::sinf::NrgSinf;
use super::mtyp::NrgMtyp;


#[derive(Debug)]
pub struct NrgMetadata {
    pub file_size: u64,
    pub nrg_version: u8,
    pub chunk_offset: u64,
    pub cuex_chunk: Option<NrgCuex>,
    pub daox_chunk: Option<NrgDaox>,
    pub sinf_chunk: Option<NrgSinf>,
    pub mtyp_chunk: Option<NrgMtyp>,
    pub skipped_chunks: Vec<String>,
}

impl NrgMetadata {
    pub fn new() -> NrgMetadata {
        NrgMetadata {
            file_size: 0,
            nrg_version: 0,
            chunk_offset: 0,
            cuex_chunk: None,
            daox_chunk: None,
            sinf_chunk: None,
            mtyp_chunk: None,
            skipped_chunks: Vec::new(),
        }
    }
}

impl fmt::Display for NrgMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Image size: {} Bytes\n\
                        NRG format version: {}\n\
                        First chunk offset: {}",
                    self.file_size,
                    self.nrg_version,
                    self.chunk_offset,
        ));
        match self.cuex_chunk {
            None => {},
            Some(ref chunk) => try!(write!(f, "\n\n\
                                               {}", chunk)),
        }
        match self.daox_chunk {
            None => {},
            Some(ref chunk) => try!(write!(f, "\n\n\
                                               {}", chunk)),
        }
        match self.sinf_chunk {
            None => {},
            Some(ref chunk) => try!(write!(f, "\n\n\
                                               {}", chunk)),
        }
        match self.mtyp_chunk {
            None => {},
            Some(ref chunk) => try!(write!(f, "\n\n\
                                               {}", chunk)),
        }
        if !self.skipped_chunks.is_empty() {
            try!(write!(f, "\n\nUnhandled chunks present in this image:"));
            for chunk_id in &self.skipped_chunks {
                try!(write!(f, " {}", chunk_id));
            }
        }
        Ok(())
    }
}
