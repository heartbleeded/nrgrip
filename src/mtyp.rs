use std::fmt;


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
