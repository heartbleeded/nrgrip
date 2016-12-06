//! Module to read and store the metadata from an NRG image file.

use std::fs::File;
use std::io::{Seek, SeekFrom};

use ::error::NrgError;

mod metadata;
mod cuex;
mod daox;
mod sinf;
mod mtyp;
mod readers;

use self::metadata::NrgMetadata;
use self::readers::*;


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
            "CUEX" => { nm.cuex_chunk = Some(try!(cuex::read_nrg_cuex(fd))); },
            "DAOX" => { nm.daox_chunk = Some(try!(daox::read_nrg_daox(fd))); },
            "CDTX" => { try!(skip_unhandled_chunk(fd, &chunk_id)); },
            "ETN2" => { try!(skip_unhandled_chunk(fd, &chunk_id)); },
            "SINF" => { nm.sinf_chunk = Some(try!(sinf::read_nrg_sinf(fd))); },
            "MTYP" => { nm.mtyp_chunk = Some(try!(mtyp::read_nrg_mtyp(fd))); },
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
