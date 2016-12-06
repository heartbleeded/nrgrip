use std::fmt;


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
