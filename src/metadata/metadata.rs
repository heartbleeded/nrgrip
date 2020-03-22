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
use super::afnm::NrgAfnm;


#[derive(Debug)]
pub struct NrgMetadata {
    pub file_size: u64,
    pub nrg_version: u8,
    pub chunk_offset: u64,
    pub cuex_chunk: Option<NrgCuex>,
    pub daox_chunk: Option<NrgDaox>,
    pub sinf_chunk: Option<NrgSinf>,
    pub mtyp_chunk: Option<NrgMtyp>,
    pub afnm_chunk: Option<NrgAfnm>,
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
            afnm_chunk: None,
            skipped_chunks: Vec::new(),
        }
    }

    /// Returns the index1 of the first DAOX track in `metadata`, or 0 if there
    /// are no DAOX tracks.
    pub fn first_audio_byte(&self) -> u64 {
        if let Some(daox_chunk) = self.daox_chunk.as_ref() {
            if let Some(first_track) = daox_chunk.tracks.first() {
                return first_track.index1;
            }
        }
        0
    }

    /// Returns the number of the byte past the last audio byte in the image
    /// (i.e. `last audio byte + 1`).
    ///
    /// This byte is indicated by the `track_end` of the last DAOX track if at
    /// least one track is present in the DAOX chunk. If not, `chunk_offset` is
    /// returned.
    ///
    /// Note that the two values should always be identical anyway, but you
    /// never know.
    pub fn last_audio_byte(&self) -> u64 {
        if let Some(daox_chunk) = self.daox_chunk.as_ref() {
            if let Some(last_track) = daox_chunk.tracks.last() {
                return last_track.track_end;
            }
        }
        self.chunk_offset
    }

    /// Returns the sector size of this image.
    ///
    /// This information is retrieved from the first DAOX track only; it is
    /// assumed that every track has the same sector size.
    ///
    /// Returns 0 if there are no DAOX tracks.
    pub fn sector_size(&self) -> u16 {
        if let Some(daox_chunk) = self.daox_chunk.as_ref() {
            if let Some(first_track) = daox_chunk.tracks.first() {
                return first_track.sector_size;
            }
        }
        0
    }
}

impl fmt::Display for NrgMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Image size: {} Bytes\n\
                        NRG format version: {}\n\
                        First NRG chunk offset: {}",
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
        match self.afnm_chunk {
            None => {},
            Some(ref chunk) => try!(write!(f, "\n\n\
                                               {}", chunk)),
        }
        if !self.skipped_chunks.is_empty() {
            try!(write!(f, "\n\nUnhandled NRG chunks present in this image:"));
            for chunk_id in &self.skipped_chunks {
                try!(write!(f, " {}", chunk_id));
            }
        }
        Ok(())
    }
}
