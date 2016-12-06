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
