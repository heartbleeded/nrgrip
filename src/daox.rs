use std::fmt;


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
