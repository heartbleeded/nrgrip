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

//! Module to extract the raw audio data from an NRG image file.

use std::fs::File;
use std::io::{Seek, SeekFrom, Read, Write};
use std::path::PathBuf;

use ::error::NrgError;
use ::metadata::metadata::NrgMetadata;

const RAW_SEC_SIZE: u16 = 2352;
const RAW96_SEC_SIZE: u16 = 2448;


/// Extracts the raw audio data from an NRG image.
///
/// - `in_fd` is the handler to the NRG image file.
/// - `img_path` is the name of the input NRG file.
/// - `metadata` is the metadata extracted from `img_path` by nrgrip::metadata.
///
/// The output file's name is derived from `img_path`.
pub fn extract_nrg_raw_audio(in_fd: &mut File,
                             img_path: &String,
                             metadata: &NrgMetadata)
                             -> Result<(), NrgError> {
    // Seek to the first audio byte
    let first_audio_byte = metadata.first_audio_byte();
    try!(in_fd.seek(SeekFrom::Start(first_audio_byte)));

    // Open output file
    let audio_name = try!(make_output_file_name(img_path));
    let mut out_fd = try!(File::create(audio_name));

    // Copy the audio data
    let count = metadata.last_audio_byte() - first_audio_byte;
    let bytes_read = match metadata.sector_size() {
        RAW96_SEC_SIZE => try!(copy_raw96_audio(in_fd, &mut out_fd, count)),
        0              => return Err(NrgError::AudioReadError),
        _              => try!(copy_raw_audio(in_fd, &mut out_fd, count)),
    };

    assert_eq!(count, bytes_read);
    Ok(())
}


/// Reads `count` bytes from `in_fd` and write them to `out_fd`.
///
/// The offsets of `in_fd` and `out_fd` are not reset prior to reading and
/// writing.
///
/// Returns the number of bytes read/written.
fn copy_raw_audio(in_fd: &mut File, out_fd: &mut File, count: u64)
                  -> Result<u64, NrgError> {
    // The buffer size (~4,6 MiB) is a multiple of the standard audio CD sector
    // size, i.e. 2352 bytes (it doesn't have to be, though).
    const BUF_SIZE: usize = RAW_SEC_SIZE as usize * 1024 * 2;

    // Read/write audio data
    let mut bytes_read = 0;
    while bytes_read + BUF_SIZE as u64 <= count {
        let mut audio_buf = [0u8; BUF_SIZE];

        let mut nbytes = try!(in_fd.read(&mut audio_buf));
        if nbytes != BUF_SIZE {
            return Err(NrgError::AudioReadError);
        }
        bytes_read += nbytes as u64;

        nbytes = try!(out_fd.write(&audio_buf));
        if nbytes != BUF_SIZE {
            return Err(NrgError::AudioWriteError);
        }
    }

    // Read/write the last bytes
    let remaining: usize = (count - bytes_read) as usize;
    let mut audio_buf = vec![0u8; remaining];
    let mut nbytes = try!(in_fd.read(&mut audio_buf));
    if nbytes != remaining {
        return Err(NrgError::AudioReadError);
    }
    bytes_read += nbytes as u64;
    nbytes = try!(out_fd.write(&audio_buf));
    if nbytes != remaining {
        return Err(NrgError::AudioWriteError);
    }

    Ok(bytes_read)
}


/// Reads `count` bytes from `in_fd` and write them to `out_fd` after stripping
/// the sub-channel bytes.
///
/// `in_fd` is read by chunks of 2448 bytes, then the first 2352 bytes are
/// written to `out_fd`, leaving out the 96 sub-channel bytes.
///
/// The offsets of `in_fd` and `out_fd` are not reset prior to reading and
/// writing.
///
/// Returns the number of bytes read (not written).
fn copy_raw96_audio(in_fd: &mut File, out_fd: &mut File, count: u64)
                    -> Result<u64, NrgError> {
    const IN_BUF_SIZE: usize = RAW96_SEC_SIZE as usize;
    const OUT_BUF_SIZE: usize = RAW_SEC_SIZE as usize;

    // Read/write audio data
    let mut bytes_read = 0;
    while bytes_read < count {
        let mut audio_buf = vec![0u8; IN_BUF_SIZE];

        let mut nbytes = try!(in_fd.read(&mut audio_buf));
        if nbytes != IN_BUF_SIZE {
            return Err(NrgError::AudioReadError);
        }
        bytes_read += nbytes as u64;

        audio_buf.truncate(OUT_BUF_SIZE);
        nbytes = try!(out_fd.write(&audio_buf));
        if nbytes != OUT_BUF_SIZE {
            return Err(NrgError::AudioWriteError);
        }
    }

    Ok(bytes_read)
}


/// Generates the output file's name from the NRG image's name.
///
/// The output file's name will be `img_path`'s base name stripped for its
/// extension (if any), with a ".raw" extension.
fn make_output_file_name(img_path: &String) -> Result<String, NrgError> {
    let mut name = PathBuf::from(img_path);
    name.set_extension("raw");
    let name = try!(name.file_name().ok_or(
        NrgError::FileName(name.to_string_lossy().into_owned())));

    // Make sure the new name and the original name are different
    if name == img_path.as_str() {
        return Err(NrgError::FileName("Input and output file are identical"
                                      .to_string()));
    }

    Ok(name.to_string_lossy().into_owned())
}
