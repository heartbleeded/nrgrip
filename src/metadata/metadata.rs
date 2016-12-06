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
        Ok(())
    }
}
