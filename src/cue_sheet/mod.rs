//! Module to extract the cue sheet from the NRG metadata.

use std::io::Write;
use std::fs::File;

use ::error::NrgError;
use ::metadata::metadata::NrgMetadata;
use ::metadata::cuex::NrgCuexTrack;


/// Writes the cue sheet for `image_name` into a file.
///
/// `metadata` is the metadata extracted from `image_name` by nrgrip::metadata.
/// The output file's name is derived from `image_name`.
pub fn write_cue_sheet(image_name: &String, metadata: &NrgMetadata)
                       -> Result<(), NrgError> {
    if metadata.cuex_chunk.is_none() {
        return Err(NrgError::NoNrgCue);
    }
    let cuex_tracks = &metadata.cuex_chunk.as_ref().unwrap().tracks;

    let file_name = make_cue_sheet_name(image_name);
    let mut fd = try!(File::create(file_name));

    // Write cue sheet
    try!(writeln!(fd, "FILE \"{}\" RAW", image_name));
    try!(write_cue_tracks(&mut fd, cuex_tracks));

    Ok(())
}


/// Generates the cue sheet's name from the NRG image's name.
///
/// If `image_name`'s extension is `.nrg` (case-insensitive), the cue sheet's
/// name will be the same as `image_name` with a `.cue` extension instead of
/// `.nrg`. Otherwise, the cue sheet's name will be `image_name.cue`.
fn make_cue_sheet_name(image_name: &String) -> String {
    let mut name = image_name.clone();
    if name.to_lowercase().ends_with(".nrg") {
        let newlen = name.len() - 4;
        name.truncate(newlen);
    }
    name.push_str(".cue");
    name
}


/// Writes a list of cue tracks to `fd`.
fn write_cue_tracks(fd: &mut File, cuex_tracks: &Vec<NrgCuexTrack>)
                   -> Result<(), NrgError> {
    let mut index0_pos = -1; // position of the last index #0 encountered
    for track in cuex_tracks {
        try!(write_cue_track(fd, track, &mut index0_pos));
    }
    Ok(())
}


/// Writes a cue track's info to `fd`.
///
/// `index0_pos` should be negative when this function is first called.
fn write_cue_track(fd: &mut File, track: &NrgCuexTrack, index0_pos: &mut i32)
                   -> Result<(), NrgError> {
    // Ignore lead-in and lead-out areas
    if track.track_number == 0 || track.track_number == 0xAA {
        return Ok(());
    }

    // Ignore negative positions
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
