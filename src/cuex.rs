use std::fmt;


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
