use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::mem;


#[derive(Debug)]
pub enum NrgError {
    Io(io::Error),
    NrgFormat,
    NrgFormatV1,
    NrgChunkId,
}

impl fmt::Display for NrgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NrgError::Io(ref err) => err.fmt(f),
            NrgError::NrgFormat => write!(f, "NRG format unknown."),
            NrgError::NrgFormatV1 => write!(f, "NRG v1 format is not handled."),
            NrgError::NrgChunkId => write!(f, "NRG chunk ID unknown."),
        }
    }
}

impl Error for NrgError {
    fn description(&self) -> &str {
        match *self {
            NrgError::Io(ref err) => err.description(),
            NrgError::NrgFormat => "NRG format",
            NrgError::NrgFormatV1 => "NRG format v1",
            NrgError::NrgChunkId => "NRG chunk ID",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            NrgError::Io(ref err) => Some(err),
            NrgError::NrgFormat => None,
            NrgError::NrgFormatV1 => None,
            NrgError::NrgChunkId => None,
        }
    }
}

impl From<io::Error> for NrgError {
    fn from(err: io::Error) -> NrgError {
        NrgError::Io(err)
    }
}


#[derive(Debug)]
pub struct NrgMetadata {
    pub file_size: u64,
    pub nrg_version: u8,
    pub chunk_offset: u64,
    pub cuex_chunk: Option<NrgCuex>,
}

impl NrgMetadata {
    fn new() -> NrgMetadata {
        NrgMetadata {
            file_size: 0,
            nrg_version: 0,
            chunk_offset: 0,
            cuex_chunk: None,
        }
    }
}

impl fmt::Display for NrgMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Image size: {}\n\
                        NRG format version: {}\n\
                        First chunk offset: {}",
                    self.file_size,
                    self.nrg_version,
                    self.chunk_offset,
        ));
        match self.cuex_chunk {
            None => {},
            Some(ref chunk) => try!(write!(f, "\n\
                                               {}", chunk)),
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct NrgCuex {
    pub size: u32,
    pub tracks: Vec<NrgCuexTrack>,
}

impl NrgCuex {
    fn new() -> NrgCuex {
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
                        Chunk size: {}", self.size));
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
    mode: u8,
    track_number: u8,
    index_number: u8,
    padding: u8,
    position_sectors: i32,
}

impl NrgCuexTrack {
    fn new() -> NrgCuexTrack {
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


pub fn parse_nrg_metadata(img_name: String) -> Result<NrgMetadata, NrgError> {
    let mut nm = NrgMetadata::new();

    // Open the image file
    let mut fd = try!(File::open(img_name));

    // Get the file size
    nm.file_size = try!(fd.seek(SeekFrom::End(0)));

    // Get the NRG format from the footer
    nm.nrg_version = try!(read_nrg_version(&mut fd, nm.file_size));
    if nm.nrg_version != 2 {
        // We handle only NRG v2
        return Err(NrgError::NrgFormatV1);
    }

    // Read the first chunk offset
    nm.chunk_offset = try!(read_u64(&mut fd));

    // Read all the chunks
    try!(fd.seek(SeekFrom::Start(nm.chunk_offset)));
    try!(read_nrg_chunks(&mut fd, &mut nm));

    Ok(nm)
}


/// Determines the NRG format of an open NRG image `fd` of file `file_size`.
/// 
/// The offset is left after the main chunk ID, therefore the calling function
/// can read the first data chunk's offset (32 bits for NRG v1 or 64 bits for
/// NRG v2) directly without seeking.
fn read_nrg_version(fd: &mut File, file_size: u64) -> Result<u8, NrgError> {
    if file_size < 12 {
        // Input file too small
        return Err(NrgError::NrgFormat);
    }

    // In NRG v2, the main footer is on the last 12 bytes
    try!(fd.seek(SeekFrom::End(-12)));
    let chunk_id = try!(read_nrg_chunk_id(&fd));
    if chunk_id == "NER5" {
        return Ok(2); // NRG v2
    }

    // In NRG v1, the main footer is on the last 8 bytes; since we just read 4
    // bytes after seeking 12 bytes before the end, the offset is right
    let chunk_id = try!(read_nrg_chunk_id(&fd));
    if chunk_id == "NERO" {
        return Ok(1); // NRG v1
    }

    Err(NrgError::NrgFormat) // Unknown format
}


/// Reads all the available NRG chunks.
///
/// Returns the number of chunks read.
fn read_nrg_chunks(mut fd: &mut File, nm: &mut NrgMetadata) -> Result<u16, NrgError> {
    let mut nread = 0u16;

    loop {
        let chunk_id = try!(read_nrg_chunk_id(&fd));
        nread += 1;
        match chunk_id.as_ref() {
            "END!" => break,
            "CUEX" => { nm.cuex_chunk = Some(try!(read_nrg_cuex(&mut fd))); },
            "DAOX" => { try!(skip_unhandled_chunk(&mut fd, &chunk_id)); },
            //"DAOX" => { try!(read_nrg_daox(&mut fd)); },
            "CDTX" => { try!(skip_unhandled_chunk(&mut fd, &chunk_id)); },
            "ETN2" => { try!(skip_unhandled_chunk(&mut fd, &chunk_id)); },
            "SINF" => { try!(skip_unhandled_chunk(&mut fd, &chunk_id)); },
            "MTYP" => { try!(skip_unhandled_chunk(&mut fd, &chunk_id)); },
            // "SINF" => { try!(read_nrg_sinf(&mut fd)); },
            // "MTYP" => { try!(read_nrg_mytp(&mut fd)); },
            "DINF" => { try!(skip_unhandled_chunk(&mut fd, &chunk_id)); },
            "TOCT" => { try!(skip_unhandled_chunk(&mut fd, &chunk_id)); },
            "RELO" => { try!(skip_unhandled_chunk(&mut fd, &chunk_id)); },
            _      => { println!("{}", chunk_id); return Err(NrgError::NrgChunkId); }, //fixme
        }
    }

    Ok(nread)
}


/// Reads an NRG chunk ID (i.e. a 4-byte string) from `fd`.
fn read_nrg_chunk_id(fd: &File) -> Result<String, NrgError> {
    read_sized_string(fd, 4)
}


/// Reads a String of size `size` from `fd`.
fn read_sized_string(fd: &File, size: u64) -> Result<String, NrgError> {
    let mut handle = fd.take(size);
    let mut string = String::new();
    try!(handle.read_to_string(&mut string));
    Ok(string)
}


/// Reads a 64-bit unsigned integer from `fd`.
fn read_u64(fd: &mut File) -> Result<u64, NrgError> {
    let mut buf = [0u8; 8];
    try!(fd.read_exact(&mut buf));
    let i: u64;
    unsafe {
        i = mem::transmute(buf);
    }
    Ok(u64::from_be(i))
}


/// Reads a 32-bit unsigned integer from `fd`.
fn read_u32(fd: &mut File) -> Result<u32, NrgError> {
    let mut buf = [0u8; 4];
    try!(fd.read_exact(&mut buf));
    let i: u32;
    unsafe {
        i = mem::transmute(buf);
    }
    Ok(u32::from_be(i))
}


/// Reads a 16-bit unsigned integer from `fd`.
#[cfg(dead_code)]
fn read_u16(fd: &mut File) -> Result<u16, NrgError> {
    let mut buf = [0u8; 2];
    try!(fd.read_exact(&mut buf));
    let i: u16;
    unsafe {
        i = mem::transmute(buf);
    }
    Ok(u16::from_be(i))
}


/// Reads an unsigned byte from `fd`.
fn read_u8(fd: &mut File) -> Result<u8, NrgError> {
    let mut buf = [0u8; 1];
    try!(fd.read_exact(&mut buf));
    Ok(buf[0])
}


fn skip_unhandled_chunk(mut fd: &mut File, chunk_id: &str) -> Result<(), NrgError> {
    let chunk_size = try!(read_u32(&mut fd));
    try!(fd.seek(SeekFrom::Current(chunk_size as i64)));
    // fixme: lib should'nt print!
    println!("Skipping unhandled chunk: {} ({} bytes)", chunk_id, chunk_size);
    Ok(())
}


/// Reads the NRG Cue Sheet chunk (CUEX).
///
/// The CUEX is constituted of the following data:
///
/// - 4 B: Chunk size (bytes): size to be read *after* this (should be a
///        multiple of 8)
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
fn read_nrg_cuex(mut fd: &mut File) -> Result<NrgCuex, NrgError> {
    let mut chunk = NrgCuex::new();
    chunk.size = try!(read_u32(&mut fd));
    let mut bytes_read = 0;

    // Read all the 8-byte track info
    while bytes_read < chunk.size {
        chunk.tracks.push(try!(read_nrg_cuex_track(&mut fd)));
        bytes_read += 8;
    }

    assert_eq!(bytes_read, chunk.size);

    Ok(chunk)
}


/// Reads a track from the NRG cue sheet.
fn read_nrg_cuex_track(mut fd: &mut File) -> Result<NrgCuexTrack, NrgError> {
    let mut track = NrgCuexTrack::new();
    track.mode = try!(read_u8(&mut fd));
    track.track_number = try!(read_u8(&mut fd));
    track.index_number = try!(read_u8(&mut fd));
    track.padding = try!(read_u8(&mut fd));
    track.position_sectors = try!(read_u32(&mut fd)) as i32;
    Ok(track)
}


#[cfg(dead_code)]
fn read_nrg_daox(fd: &mut File) -> Result<i32, NrgError> {
    unimplemented!();
}


#[cfg(dead_code)]
fn read_nrg_sinf(fd: &mut File) -> Result<i32, NrgError> {
    unimplemented!();
}


#[cfg(dead_code)]
fn read_nrg_mytp(fd: &mut File) -> Result<i32, NrgError> {
    unimplemented!();
}
