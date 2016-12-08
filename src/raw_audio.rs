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
    const BUF_SIZE: usize = 1024 * 1024 * 4; // 4 MiB

    // Seek to the first audio byte
    let first_audio_byte = get_daox_track1_index1(metadata);
    try!(in_fd.seek(SeekFrom::Start(first_audio_byte)));

    // Get the last audio byte
    let last_audio_byte = get_last_audio_byte(metadata);

    // Open output file
    let audio_name = try!(make_output_file_name(img_path));
    let mut out_fd = try!(File::create(audio_name));

    // Read/write audio data
    let mut cur_offset = first_audio_byte;
    while cur_offset + BUF_SIZE as u64 <= last_audio_byte {
        let mut audio_buf = [0u8; BUF_SIZE];

        let mut nbytes = try!(in_fd.read(&mut audio_buf));
        if nbytes != BUF_SIZE {
            return Err(NrgError::AudioReadError);
        }
        cur_offset += nbytes as u64;

        nbytes = try!(out_fd.write(&audio_buf));
        if nbytes != BUF_SIZE {
            return Err(NrgError::AudioWriteError);
        }
    }

    // Read/write the last bytes
    let remaining: usize = (last_audio_byte - cur_offset) as usize;
    let mut audio_buf = vec![0u8; remaining];
    let mut nbytes = try!(in_fd.read(&mut audio_buf));
    if nbytes != remaining {
        return Err(NrgError::AudioReadError);
    }
    cur_offset += nbytes as u64;
    nbytes = try!(out_fd.write(&audio_buf));
    if nbytes != remaining {
        return Err(NrgError::AudioWriteError);
    }

    assert_eq!(cur_offset, last_audio_byte);
    Ok(())
}


/// Returns the index1 of the first DAOX track in `metadata`, or 0 if there are
/// no DAOX tracks.
fn get_daox_track1_index1(metadata: &NrgMetadata) -> u64 {
    if metadata.daox_chunk.is_none() {
        return 0;
    }
    let daox_tracks = &metadata.daox_chunk.as_ref().unwrap().tracks;
    if daox_tracks.is_empty() {
        return 0;
    }
    return daox_tracks[0].index1;
}


/// Returns the number of the byte past the last audio byte in the image.
///
/// This byte is indicated by the `track_end` of the last DAOX track in
/// `metadata`, if at least one track is present in the DAOX chunk.
/// If not, `metadata.chunk_offset` is returned.
///
/// Note that the two values should always be identical, but you never know.
fn get_last_audio_byte(metadata: &NrgMetadata) -> u64 {
    if let Some(daox_chunk) = metadata.daox_chunk.as_ref() {
        if let Some(last_track) = daox_chunk.tracks.last() {
            return last_track.track_end;
        }
    }
    metadata.chunk_offset
}


/// Generates the output file's name from the NRG image's name.
///
/// The output file's name will be `img_path`'s base name stripped for its
/// extension (if any), with a ".raw" extension.
fn make_output_file_name(img_path: &String) -> Result<String, NrgError> {
    let mut name = PathBuf::from(img_path);
    name.set_extension("raw");
    let name = PathBuf::from(name);
    let name = try!(name.file_name().ok_or(NrgError::FileName));

    // Make sure the new name and the original name are different
    if name == img_path.as_str() {
        return Err(NrgError::FileName);
    }

    Ok(name.to_string_lossy().into_owned())
}
