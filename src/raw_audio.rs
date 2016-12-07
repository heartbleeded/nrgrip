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
    // Seek to the first audio byte
    let skip_bytes = get_daox_track1_index1(metadata);
    try!(in_fd.seek(SeekFrom::Start(skip_bytes)));

    // Open output file
    let audio_name = try!(make_output_file_name(img_path));
    let mut out_fd = try!(File::create(audio_name));

    // Read/write audio data
    let mut cur_offset = skip_bytes;
    while cur_offset < metadata.chunk_offset {
        let mut audio_buf = [0u8; 2352];

        let mut nbytes = try!(in_fd.read(&mut audio_buf));
        if nbytes != 2352 {
            return Err(NrgError::AudioReadError);
        }
        cur_offset += nbytes as u64;

        nbytes = try!(out_fd.write(&audio_buf));
        if nbytes != 2352 {
            return Err(NrgError::AudioWriteError);
        }
    }

    assert_eq!(cur_offset, metadata.chunk_offset);
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
