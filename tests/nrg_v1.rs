extern crate nrgrip;
use nrgrip::metadata;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;

#[test]
fn nrg_format() {
    let mut img = PathBuf::new();
    img.push("tests");
    img.push("minimal_v1.nrg");
    let mut fd = File::open(img)
        .expect("File::open()");
    let size = fd.seek(SeekFrom::End(0))
        .expect("fd.seek()");
    let ver = metadata::read_nrg_version(&mut fd, size)
        .expect("read_nrg_version()");
    assert_eq!(ver, 1);
}
