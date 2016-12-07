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

use std::env;
use std::fs::File;
use std::process;

extern crate nrgrip;


fn exit_usage(prog_name: &String) {
    println!("Usage:\n\t{} <image.nrg>", prog_name);
    process::exit(1);
}


fn main() {
    let mut argv = env::args();

    let prog_name = argv.next().unwrap_or("nrgrip".to_string());

    let img_name = argv.next().unwrap_or("".to_string());
    if img_name == "" {
        exit_usage(&prog_name);
    }

    println!("NRG image name: \"{}\"", img_name);

    // We don't support more than one input file
    if argv.next().is_some() {
        exit_usage(&prog_name);
    }

    // Open the image file
    let fd = File::open(&img_name);
    if fd.is_err() {
        println!("Can't open image file \"{}\": {}",
                 img_name, fd.unwrap_err().to_string());
        process::exit(1);
    }
    let mut fd = fd.unwrap();

    // Read and display the image's metadata
    let metadata = nrgrip::metadata::read_nrg_metadata(&mut fd);
    if metadata.is_err() {
        println!("Error reading \"{}\": {}",
                 img_name, metadata.unwrap_err().to_string());
        process::exit(1);
    }
    let metadata = metadata.unwrap();
    println!("\n{}", metadata);

    // Read and write the cue sheet
    println!("\nNow extracting cue sheet...");
    if let Err(err) = nrgrip::cue_sheet::write_cue_sheet(&img_name, &metadata) {
        println!("Error writing cue sheet: {}", err.to_string());
        process::exit(1);
    }

    // Extract raw audio data
    println!("Now extracting raw audio data...");
    if let Err(err) = nrgrip::raw_audio::extract_nrg_raw_audio(
        &mut fd, &img_name, &metadata) {
        println!("Error extracting raw audio data: {}", err.to_string());
    }
    println!("OK!");
}
