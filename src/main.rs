use std::env;
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

    println!("NRG image name: {}", img_name);

    // We don't support more than one input file
    if argv.next().is_some() {
        exit_usage(&prog_name);
    }

    match nrgrip::metadata::parse_nrg_metadata(img_name) {
        Err(err) => println!("{}", err.to_string()),
        Ok(metadata) => println!("\n\
                                  *** Metadata ***\n\
                                  {}", metadata),
    };
}
