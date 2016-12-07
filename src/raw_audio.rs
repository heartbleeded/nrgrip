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

use ::error::NrgError;
use ::metadata::metadata::NrgMetadata;


/// Extracts the raw audio data from an NRG image.
///
/// - `in_fd` is the handler to the NRG image file.
/// - `image_name` is the name of the input NRG file.
/// - `metadata` is the metadata extracted from `image_name` by
///    nrgrip::metadata.
///
/// The output file's name is derived from `image_name`.
pub fn extract_nrg_raw_audio(in_fd: &mut File,
                             image_name: &String,
                             metadata: &NrgMetadata)
                             -> Result<(), NrgError> {
    let skip_bytes = get_daox_track1_index1(metadata);

    try!(in_fd.seek(SeekFrom::Start(skip_bytes)));

    let file_name = make_output_file_name(image_name);
    let mut out_fd = try!(File::create(file_name));

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
/// If `image_name`'s extension is `.nrg` (case-insensitive), the name will be
/// the same as `image_name` with a `.raw` extension instead of `.nrg`.
/// Otherwise, it will be `image_name.raw`.
fn make_output_file_name(image_name: &String) -> String {
    let mut name = image_name.clone();
    if name.to_lowercase().ends_with(".nrg") {
        let newlen = name.len() - 4;
        name.truncate(newlen);
    }
    name.push_str(".raw");
    name
}
