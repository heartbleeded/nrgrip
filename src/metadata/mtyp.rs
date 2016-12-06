//! NRG MTYP chunk data structure and associated functions.

use std::fmt;
use std::fs::File;

use ::error::NrgError;
use super::readers::read_u32;


#[derive(Debug)]
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
