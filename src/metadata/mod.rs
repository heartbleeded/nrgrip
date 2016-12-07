// This file is part of the NRGrip project.
//
// Copyright (c) 2016 Matteo Cypriani <mcy@lm7.fr>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

//! Module to read and store the metadata from an NRG image file.

use std::fs::File;
use std::io::{Seek, SeekFrom};

use ::error::NrgError;

pub mod metadata;
pub mod cuex;
mod daox;
mod sinf;
mod mtyp;
mod readers;

use self::metadata::NrgMetadata;
use self::readers::*;


/// Reads the metadata chunks from an open NRG image file `fd`.
///
/// `fd`'s offset can be anywhere when this function is called: it will be reset
/// before anything is read.
///
/// In case of success, `fd`'s offset will be left after the "END!" string of
/// the NRG footer. Otherwise, the offset is undefined and should be reset by
/// the caller if any additional reading operations are to be done.
pub fn read_nrg_metadata(fd: &mut File) -> Result<NrgMetadata, NrgError> {
    let mut nm = NrgMetadata::new();

    // Get the file size
    nm.file_size = try!(fd.seek(SeekFrom::End(0)));

    // Get the NRG format from the footer
    nm.nrg_version = try!(read_nrg_version(fd, nm.file_size));
    if nm.nrg_version != 2 {
        // We handle only NRG v2
        return Err(NrgError::NrgFormatV1);
    }

    // Read the first chunk offset
    nm.chunk_offset = try!(read_u64(fd));

    // Read all the chunks
    try!(fd.seek(SeekFrom::Start(nm.chunk_offset)));
    try!(read_nrg_chunks(fd, &mut nm));

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
fn read_nrg_chunks(fd: &mut File, nm: &mut NrgMetadata) -> Result<(), NrgError> {
    loop {
        let chunk_id = try!(read_nrg_chunk_id(fd));
        match chunk_id.as_ref() {
            "END!" => break,
            "CUEX" => { nm.cuex_chunk = Some(try!(cuex::read_nrg_cuex(fd))); },
            "DAOX" => { nm.daox_chunk = Some(try!(daox::read_nrg_daox(fd))); },
            "CDTX" => {
                try!(skip_chunk(fd));
                nm.skipped_chunks.push(chunk_id);
            },
            "ETN2" => {
                try!(skip_chunk(fd));
                nm.skipped_chunks.push(chunk_id);
            },
            "SINF" => { nm.sinf_chunk = Some(try!(sinf::read_nrg_sinf(fd))); },
            "MTYP" => { nm.mtyp_chunk = Some(try!(mtyp::read_nrg_mtyp(fd))); },
            "DINF" => {
                try!(skip_chunk(fd));
                nm.skipped_chunks.push(chunk_id);
            },
            "TOCT" => {
                try!(skip_chunk(fd));
                nm.skipped_chunks.push(chunk_id);
            },
            "RELO" => {
                try!(skip_chunk(fd));
                nm.skipped_chunks.push(chunk_id);
            },
            _      => { println!("{}", chunk_id); return Err(NrgError::NrgChunkId); }, //fixme
        }
    }
    Ok(())
}


/// Reads an NRG chunk ID (i.e. a 4-byte string) from `fd`.
fn read_nrg_chunk_id(fd: &File) -> Result<String, NrgError> {
    read_sized_string(fd, 4)
}


/// Skips a chunk.
fn skip_chunk(fd: &mut File) -> Result<(), NrgError> {
    let chunk_size = try!(read_u32(fd));
    try!(fd.seek(SeekFrom::Current(chunk_size as i64)));
    Ok(())
}
