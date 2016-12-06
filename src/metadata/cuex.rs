//! NRG CUEX chunk data structure and associated functions.

use std::fmt;
use std::fs::File;

use ::error::NrgError;
use super::readers::*;


#[derive(Debug)]
pub struct NrgCuex {
    pub size: u32,
    pub tracks: Vec<NrgCuexTrack>,
}

impl NrgCuex {
    pub fn new() -> NrgCuex {
        NrgCuex {
            size: 0,
            tracks: Vec::new(),
        }
    }
}

impl fmt::Display for NrgCuex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Chunk ID: CUEX\n\
                        Chunk description: Cue Sheet\n\
                        Chunk size: {} Bytes", self.size));
        if self.tracks.is_empty() {
            try!(write!(f, "\nNo CUEX tracks!"));
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


#[derive(Debug)]
pub struct NrgCuexTrack {
    pub mode: u8,
    pub track_number: u8,
    pub index_number: u8,
    pub padding: u8,
    pub position_sectors: i32,
}

impl NrgCuexTrack {
    pub fn new() -> NrgCuexTrack {
        NrgCuexTrack {
            mode: 0,
            track_number: 0,
            index_number: 0,
            padding: 0,
            position_sectors: 0,
        }
    }
}

impl fmt::Display for NrgCuexTrack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "\tMode: 0x{:02X}", self.mode));

        try!(write!(f, "\tTrack number: "));
        if self.track_number == 0 {
            try!(writeln!(f, "0 (lead-in area)"));
        } else if self.track_number == 0xAA {
            try!(writeln!(f, "0xAA (lead-out area)"));
        } else {
            try!(writeln!(f, "{}", self.track_number));
        }

        try!(writeln!(f, "\tIndex number: {}", self.index_number));

        if self.padding != 0 {
            try!(writeln!(f, "\tPadding: {} (Warning: should be 0!)",
                          self.padding));
        }

        // Audio CDs are played at a 75 sectors per second rate:
        let position_seconds: f64 = (self.position_sectors as f64) / 75.0;
        write!(f, "\tPosition: {} sectors ({:.2} seconds)",
               self.position_sectors, position_seconds)
    }
}


/// Reads the NRG Cue Sheet chunk (CUEX) from `fd`.
///
/// The CUEX is constituted of the following data:
///
/// - 4 B: Chunk size (in bytes): size to be read *after* this chunk size
///        (should be a multiple of 8)
///
/// - one or more pairs of 8-byte blocks composed of:
///   + 1 B: Mode (values found: 0x01 for audio; 0x21 for non
///          copyright-protected audio; 0x41 for data)
///   + 1 B: Track number (BCD coded; 0xAA for the lead-out area)
///   + 1 B: Index number (probably BCD coded): 0 or 1
///   + 1 B: Unknown (padding?), always 0
///   + 4 B: Position in sectors (signed integer value)
///
/// - one last block like the ones above, for the lead-out area (optional?)
pub fn read_nrg_cuex(fd: &mut File) -> Result<NrgCuex, NrgError> {
    let mut chunk = NrgCuex::new();
    chunk.size = try!(read_u32(fd));
    let mut bytes_read = 0;

    // Read all the 8-byte track info
    while bytes_read < chunk.size {
        chunk.tracks.push(try!(read_nrg_cuex_track(fd)));
        bytes_read += 8;
    }

    assert_eq!(bytes_read, chunk.size);

    Ok(chunk)
}


/// Reads a track from the NRG cue sheet.
fn read_nrg_cuex_track(fd: &mut File) -> Result<NrgCuexTrack, NrgError> {
    let mut track = NrgCuexTrack::new();
    track.mode = try!(read_u8(fd));
    track.track_number = try!(read_u8(fd));
    track.index_number = try!(read_u8(fd));
    track.padding = try!(read_u8(fd));
    track.position_sectors = try!(read_u32(fd)) as i32;
    Ok(track)
}
