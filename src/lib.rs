use std::fs::File;
use std::io::{Seek, SeekFrom};

mod error;
use error::NrgError;

mod metadata;
use metadata::NrgMetadata;

mod cuex;
use cuex::{NrgCuex, NrgCuexTrack};

mod daox;
use daox::{NrgDaox, NrgDaoxTrack};

mod sinf;
use sinf::{NrgSinf};

mod mtyp;
use mtyp::{NrgMtyp};

mod readers;
use readers::*;


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
    let chunk_id = try!(read_nrg_chunk_id(fd));
    if chunk_id == "NER5" {
        return Ok(2); // NRG v2
    }

    // In NRG v1, the main footer is on the last 8 bytes; since we just read 4
    // bytes after seeking 12 bytes before the end, the offset is right
    let chunk_id = try!(read_nrg_chunk_id(fd));
    if chunk_id == "NERO" {
        return Ok(1); // NRG v1
    }

    Err(NrgError::NrgFormat) // Unknown format
}


/// Reads all the available NRG chunks.
///
/// Returns the number of chunks read.
fn read_nrg_chunks(fd: &mut File, nm: &mut NrgMetadata) -> Result<u16, NrgError> {
    let mut nread = 0u16;

    loop {
        let chunk_id = try!(read_nrg_chunk_id(fd));
        nread += 1;
        match chunk_id.as_ref() {
            "END!" => break,
            "CUEX" => { nm.cuex_chunk = Some(try!(read_nrg_cuex(fd))); },
            "DAOX" => { nm.daox_chunk = Some(try!(read_nrg_daox(fd))); },
            "CDTX" => { try!(skip_unhandled_chunk(fd, &chunk_id)); },
            "ETN2" => { try!(skip_unhandled_chunk(fd, &chunk_id)); },
            "SINF" => { nm.sinf_chunk = Some(try!(read_nrg_sinf(fd))); },
            "MTYP" => { nm.mtyp_chunk = Some(try!(read_nrg_mtyp(fd))); },
            "DINF" => { try!(skip_unhandled_chunk(fd, &chunk_id)); },
            "TOCT" => { try!(skip_unhandled_chunk(fd, &chunk_id)); },
            "RELO" => { try!(skip_unhandled_chunk(fd, &chunk_id)); },
            _      => { println!("{}", chunk_id); return Err(NrgError::NrgChunkId); }, //fixme
        }
    }

    Ok(nread)
}


/// Reads an NRG chunk ID (i.e. a 4-byte string) from `fd`.
fn read_nrg_chunk_id(fd: &File) -> Result<String, NrgError> {
    read_sized_string(fd, 4)
}


fn skip_unhandled_chunk(fd: &mut File, chunk_id: &str) -> Result<(), NrgError> {
    let chunk_size = try!(read_u32(fd));
    try!(fd.seek(SeekFrom::Current(chunk_size as i64)));
    // fixme: lib should'nt print!
    println!("Skipping unhandled chunk: {} ({} bytes)", chunk_id, chunk_size);
    Ok(())
}


/// Reads the NRG Cue Sheet chunk (CUEX).
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
fn read_nrg_cuex(fd: &mut File) -> Result<NrgCuex, NrgError> {
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
/// Followed by one or more groups of 42-byte blocks composed of:
/// - 12 B: ISRC (text) or null bytes
/// - 2 B: Sector size in the image file (bytes)
/// - 2 B: Mode of the data in the image file
/// - 2 B: Unknown (should always be 0x0001)
/// - 8 B: Index0 (Pre-gap) (bytes)
/// - 8 B: Index1 (Start of track) (bytes)
/// - 8 B: End of track + 1 (bytes)
fn read_nrg_daox(fd: &mut File) -> Result<NrgDaox, NrgError> {
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


/// Reads a track from the NRG DAO Information.
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


/// Reads the NRG Session Information chunk (SINF).
fn read_nrg_sinf(fd: &mut File) -> Result<NrgSinf, NrgError> {
    let mut chunk = NrgSinf::new();
    chunk.size = try!(read_u32(fd));
    chunk.nb_tracks = try!(read_u32(fd));
    Ok(chunk)
}


/// Reads the Media Type (?) chunk (MTYP).
fn read_nrg_mtyp(fd: &mut File) -> Result<NrgMtyp, NrgError> {
    let mut chunk = NrgMtyp::new();
    chunk.size = try!(read_u32(fd));
    chunk.unknown = try!(read_u32(fd));
    Ok(chunk)
}
