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

//! NRG DAOX chunk data structure and associated functions.

use std::fmt;
use std::fs::File;

use ::error::NrgError;
use super::readers::*;


#[derive(Debug)]
pub struct NrgDaox {
    pub size: u32,
    pub size2: u32,
    pub upc: String,
    pub padding: u8,
    pub toc_type: u16,
    pub first_track: u8,
    pub last_track: u8,
    pub tracks: Vec<NrgDaoxTrack>,
}

impl NrgDaox {
    pub fn new() -> NrgDaox {
        NrgDaox {
            size: 0,
            size2: 0,
            upc: String::new(),
            padding: 0,
            toc_type: 0,
            first_track: 0,
            last_track: 0,
            tracks: Vec::new(),
        }
    }
}

impl fmt::Display for NrgDaox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Chunk ID: DAOX\n\
                          Chunk description: DAO (Disc At Once) Information\n\
                          Chunk size: {} Bytes\n\
                          Chunk size 2: {}\n\
                          UPC: \"{}\"",
                      self.size,
                      self.size2,
                      self.upc));

        if self.padding != 0 {
            try!(writeln!(f, "Padding: {} (Warning: should be 0!)",
                          self.padding));
        }

        try!(write!(f, "TOC type: 0x{:04X}\n\
                        First track in the session: {}\n\
                        Last track in the session: {}",
                    self.toc_type,
                    self.first_track,
                    self.last_track));

        if self.tracks.is_empty() {
            try!(write!(f, "\nNo DAOX tracks!"));
        } else {
            let mut i = 1;
            for track in &self.tracks {
                try!(write!(f, "\n\
                                Track {:02}:\n\
                                {}", i, track));
                i += 1;
            }
        }

        Ok(())
    }
}


#[derive(Debug)]
pub struct NrgDaoxTrack {
    pub isrc: String,
    pub sector_size: u16,
    pub data_mode: u16,
    pub unknown: u16,
    pub index0: u64,
    pub index1: u64,
    pub track_end: u64,
}

impl NrgDaoxTrack {
    pub fn new() -> NrgDaoxTrack {
        NrgDaoxTrack {
            isrc: String::new(),
            sector_size: 0,
            data_mode: 0,
            unknown: 0,
            index0: 0,
            index1: 0,
            track_end: 0,
        }
    }
}

impl fmt::Display for NrgDaoxTrack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "\tISRC: \"{}\"\n\
                          \tSector size in the image file: {} Bytes\n\
                          \tMode of the data in the image file: 0x{:04X}",
                      self.isrc,
                      self.sector_size,
                      self.data_mode));

        if self.unknown != 0x0001 {
            try!(writeln!(f, "\tUnknown field: 0x{:04X} \
                              (Warning: should be 0x0001!)",
                          self.unknown));
        }

        write!(f, "\tIndex0 (Pre-gap): {} Bytes\n\
                   \tIndex1 (Start of track): {} Bytes\n\
                   \tEnd of track + 1: {} Bytes",
               self.index0,
               self.index1,
               self.track_end)
    }
}


/// Reads the NRG Disc-At-Once Information chunk (DAOX).
///
/// The DAOX is constituted of the following data:
///
/// - 4 B: Chunk size (in bytes)
/// - 4 B: Chunk size again, sometimes little endian
/// - 13 B: UPC (text) or null bytes
/// - 1 B: Unknown (padding?), always 0
/// - 2 B: TOC type
/// - 1 B: First track in the session
/// - 1 B: Last track in the session
///
/// Followed by one or more groups of 42-byte track blocks composed of:
/// - 12 B: ISRC (text) or null bytes
/// - 2 B: Sector size in the image file (bytes)
/// - 2 B: Mode of the data in the image file
/// - 2 B: Unknown (should always be 0x0001)
/// - 8 B: Index0 (Pre-gap) (bytes)
/// - 8 B: Index1 (Start of track) (bytes)
/// - 8 B: End of track + 1 (bytes)
pub fn read_nrg_daox(fd: &mut File) -> Result<NrgDaox, NrgError> {
    let mut chunk = NrgDaox::new();
    chunk.size = try!(read_u32(fd));
    let mut bytes_read = 0;

    chunk.size2 = try!(read_u32(fd));
    bytes_read += 4; // 32 bits

    chunk.upc = try!(read_sized_string(fd, 13));
    bytes_read += 13;

    chunk.padding = try!(read_u8(fd));
    bytes_read += 1;

    chunk.toc_type = try!(read_u16(fd));
    bytes_read += 2;

    chunk.first_track = try!(read_u8(fd));
    chunk.last_track = try!(read_u8(fd));
    bytes_read += 2;

    // Read all the 42-byte track info
    while bytes_read < chunk.size {
        chunk.tracks.push(try!(read_nrg_daox_track(fd)));
        bytes_read += 42;
    }

    assert_eq!(bytes_read, chunk.size);

    Ok(chunk)
}


/// Reads a 42-byte track block from the NRG DAO Information.
///
/// See the documentation for read_nrg_daox() for the format of the track
/// blocks.
fn read_nrg_daox_track(fd: &mut File) -> Result<NrgDaoxTrack, NrgError> {
    let mut track = NrgDaoxTrack::new();
    track.isrc = try!(read_sized_string(fd, 12));
    track.sector_size = try!(read_u16(fd));
    track.data_mode = try!(read_u16(fd));
    track.unknown = try!(read_u16(fd));
    track.index0 = try!(read_u64(fd));
    track.index1 = try!(read_u64(fd));
    track.track_end = try!(read_u64(fd));
    Ok(track)
}
