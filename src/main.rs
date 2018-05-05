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

extern crate getopts;
use getopts::Options;

extern crate nrgrip;
use nrgrip::metadata;
use nrgrip::cue_sheet;
use nrgrip::raw_audio;

const PRETTY_PROGNAME: &'static str = "NRGrip";
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn print_version() {
    println!("{} v{}", PRETTY_PROGNAME, VERSION.unwrap_or("X.Y.Z"));
}

fn print_usage(prog_name: &str, opts: &Options) {
    let brief = format!("{prettyprog} - rip Nero Burning ROM audio images

Usage:
    {prog} [-icrx] [options] <image.nrg>
    {prog} [-h | -V]", prettyprog = PRETTY_PROGNAME, prog = prog_name);

    print!("{}", opts.usage(&brief));
}

fn main() {
    process::exit(main_main());
}

fn main_main() -> i32 {
    let args: Vec<String> = env::args().collect();
    let prog_name = &args.first().expect("Can't retrieve program's name");

    let mut opts = Options::new();
    opts.optflag("i", "info",
                 "display the image's metadata (default action)");
    opts.optflag("x", "extract",
                 "same as --extract-cue --extract-raw");
    opts.optflag("c", "extract-cue",
                 "extract cue sheet from the NRG metadata");
    opts.optflag("r", "extract-raw",
                 "extract the raw audio tracks");
    opts.optflag("S", "no-strip-subchannel",
                 "don't strip the 96-bit subchannel if present");
    opts.optflag("h", "help",
                 "print this help message");
    opts.optflag("V", "version",
                 "print program version");

    let options = match opts.parse(&args[1..]) {
        Ok(options) => options,
        Err(_) => {
            print_usage(&prog_name, &opts);
            return 1;
        },
    };

    if options.opt_present("help") {
        print_usage(&prog_name, &opts);
        return 0;
    }

    if options.opt_present("version") {
        print_version();
        return 0;
    }

    let strip_subchannel = !options.opt_present("no-strip-subchannel");

    // Get input NRG image name
    if options.free.len() != 1 {
        // We need exactly one input file!
        print_usage(&prog_name, &opts);
        return 1;
    }
    let img_path = &options.free[0];
    println!("NRG image path: \"{}\"", img_path);

    // See what actions are to be taken on that file
    let action_cue =
        options.opt_present("extract-cue") || options.opt_present("extract");
    let action_raw =
        options.opt_present("extract-raw") || options.opt_present("extract");
    let action_info =
        options.opt_present("info") || !(action_cue || action_raw);

    // Open the image file
    let mut fd = match File::open(&img_path) {
        Ok(fd) => fd,
        Err(err) => {
            println!("Can't open image file \"{}\": {}", img_path, err);
            return 1;
        },
    };

    // Read the image's metadata
    let metadata = match metadata::read_nrg_metadata(&mut fd) {
        Ok(metadata) => metadata,
        Err(err) => {
            println!("Error reading \"{}\": {}", img_path, err);
            return 1;
        },
    };

    // Display metadata if requested
    if action_info {
        println!("\n{}", metadata);
    }

    // Read and write the cue sheet
    if action_cue {
        println!("\nExtracting cue sheet...");
        if let Err(err) = cue_sheet::write_cue_sheet(&img_path, &metadata) {
            println!("Error writing cue sheet: {}", err);
            return 1;
        }
        println!("OK!");
    }

    // Extract raw audio data
    if action_raw {
        println!("\nExtracting raw audio data...");
        if let Err(err) =
            raw_audio::extract_nrg_raw_audio(&mut fd, &img_path,
                                             &metadata, strip_subchannel) {
            println!("Error extracting raw audio data: {}", err);
        }
        println!("OK!");
    }

    0
}
