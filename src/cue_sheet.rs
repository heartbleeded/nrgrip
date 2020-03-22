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

//! Module to extract the cue sheet from the NRG metadata.

use std::io::Write;
use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;

use ::error::NrgError;
use ::metadata::metadata::NrgMetadata;
use ::metadata::cuex::NrgCuexTrack;
use ::metadata::afnm::NrgAfnmTrack;


/// Writes the cue sheet for `img_path` into a file.
///
/// - `img_path` is the name of the input NRG file.
/// - `metadata` is the metadata extracted from `img_path` by nrgrip::metadata.
///
/// The output file's name will be `img_path`'s base name stripped for its
/// extension (if any), with a ".cue" extension.
pub fn write_cue_sheet(img_path: &str, metadata: &NrgMetadata)
                       -> Result<(), NrgError> {
    // Make sure we have a cue sheet in the metadata
    let cuex_tracks = match metadata.cuex_chunk {
        None => return Err(NrgError::NoNrgCue),
        Some(ref chunk) => &chunk.tracks,
    };
     let cuex_titles = match metadata.afnm_chunk {
        None => return Err(NrgError::NoNrgCue),
        Some(ref chunk) => &chunk.tracks,
    };
    // Get the image's base name
    let img_name = PathBuf::from(img_path);
    let img_name = match img_name.file_name() {
        Some(name) => name,
        None => return Err(NrgError::FileName(img_path.to_string())),
    };

    // Set the cue sheet file's name
    let mut cue_name = PathBuf::from(img_name);
    if cue_name.extension().unwrap_or(OsStr::new("")) == "cue" {
        // img_path's extension was already .cue: problem!
        return Err(NrgError::FileName("Input and output file are identical"
                                      .to_string()));

    }
    cue_name.set_extension("cue");

    // Set the raw audio file's name
    let mut raw_name = cue_name.clone();
    raw_name.set_extension("raw");

    // Write cue sheet
    let mut fd = try!(File::create(cue_name));
    try!(writeln!(fd, "FILE \"{}\" BINARY", raw_name.to_string_lossy()));
    try!(write_cue_tracks(&mut fd, cuex_tracks, cuex_titles));

    Ok(())
}


/// Writes a list of cue tracks to `fd`.
fn write_cue_tracks(fd: &mut File, cuex_tracks: &Vec<NrgCuexTrack>, afnm_tracks: &Vec<NrgAfnmTrack>)
                   -> Result<(), NrgError> {
    let mut index0_pos = -1; // position of the last index #0 encountered
    for track in cuex_tracks {
        try!(write_cue_track(fd, track, &mut index0_pos, afnm_tracks));
    }
    Ok(())
}


/// Writes a cue track's info to `fd`.
///
/// `index0_pos` should be negative when this function is first called.
fn write_cue_track(fd: &mut File, track: &NrgCuexTrack, index0_pos: &mut i32, afnm_tracks: &Vec<NrgAfnmTrack>)
                   -> Result<(), NrgError> {
    // Ignore lead-in and lead-out areas
    if track.track_number == 0 || track.track_number == 0xAA {
        return Ok(());
    }

    // Ignore negative positions. This should happen only for track 1, index 0
    // and for the lead-in area (which we already skipped).
    if track.position_sectors < 0 {
        return Ok(());
    }

    // Store/skip index0
    if track.index_number == 0 {
        *index0_pos = track.position_sectors;
        return Ok(());
    }

    // Write track info
    try!(writeln!(fd, "  TRACK {:02} AUDIO", track.track_number));
    try!(writeln!(fd, "    TITLE {:?}", afnm_tracks[ (track.track_number -1) as usize].name.replace(".wav", "")));
    
    // Write index0 if we stored it and it's before the current index's
    // position (i.e., it indicates a pre-gap)
    if *index0_pos >= 0 && *index0_pos < track.position_sectors {
        try!(write_cue_index(fd, 0, *index0_pos));
    }

    // Reset index0 (even if we didn't write it, because it only applies to the
    // current track)
    *index0_pos = -1;

    // Write current index
    write_cue_index(fd, track.index_number, track.position_sectors)
}


/// Writes a cue index's info to `fd`.
fn write_cue_index(fd: &mut File, index: u8, position_sectors: i32)
                   -> Result<(), NrgError> {
    assert!(position_sectors >= 0);

    // Audio CDs are played at a 75 sectors per second rate:
    let mut seconds: u32 = position_sectors as u32 / 75;
    let remaining_sectors = position_sectors % 75;
    let minutes = seconds / 60;
    seconds %= 60;

    try!(writeln!(fd, "    INDEX {:02} {:02}:{:02}:{:02}",
                  index, minutes, seconds, remaining_sectors));

    Ok(())
}
