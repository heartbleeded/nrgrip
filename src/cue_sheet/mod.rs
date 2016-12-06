//! Module to extract the cue sheet from the NRG metadata.

use ::error::NrgError;
use ::metadata::metadata::NrgMetadata;


pub fn write_cue_sheet(image_name: &String, nm: &NrgMetadata)
                       -> Result<(), NrgError> {
    if nm.cuex_chunk.is_none() {
        return Err(NrgError::NoNrgCue);
    }
    let cuex_tracks = &nm.cuex_chunk.as_ref().unwrap().tracks;

    let file_name = try!(make_cue_sheet_name(image_name));

    println!("FILE \"{}\" RAW", image_name);

    let mut index0_pos = -1;
    for track in cuex_tracks {
        // Ignore lead-in and lead-out areas
        if track.track_number == 0 || track.track_number == 0xAA {
            continue;
        }

        // Ignore negative positions
        if track.position_sectors < 0 {
            continue;
        }

        // Store/skip index0
        if track.index_number == 0 {
            index0_pos = track.position_sectors;
            continue;
        }

        // Write track info
        println!("  TRACK {:02} AUDIO", track.track_number);

        // Write index0 if we stored it and it's before the current index's
        // position (i.e., it indicates a pre-gap)
        if index0_pos >= 0 && index0_pos < track.position_sectors {
            try!(write_cue_index(0, index0_pos));
            index0_pos = -1;
        }

        // Write current index
        try!(write_cue_index(track.index_number, track.position_sectors));
    }

    Ok(())
}


/// Generates the cue sheet's name from the NRG image's name.
///
/// If `image_name`'s extension is `.nrg` (case-insensitive), the cue sheet's
/// name will be the same as `image_name` with a `.cue` extension instead of
/// `.nrg`. Otherwise, the cue sheet's name will be `image_name.cue`.
fn make_cue_sheet_name(image_name: &String) -> Result<String, NrgError> {
    Ok("image.cue".to_string())
}


fn write_cue_index(index: u8, position_sectors: i32)
                   -> Result<(), NrgError> {
    assert!(position_sectors >= 0);

    // Audio CDs are played at a 75 sectors per second rate:
    let mut seconds: u32 = position_sectors as u32 / 75;
    let remaining_sectors = position_sectors % 75;
    let minutes = seconds / 60;
    seconds %= 60;

    println!("    INDEX {:02} {:02}:{:02}:{:02}",
             index, minutes, seconds, remaining_sectors);

    Ok(())
}
