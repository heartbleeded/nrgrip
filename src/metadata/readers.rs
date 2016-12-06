//! Miscellaneous functions to read fixed-size data from a file.

use std::fs::File;
use std::io::Read;
use std::mem;

use ::error::NrgError;


/// Reads a String of size `size` from `fd`.
pub fn read_sized_string(fd: &File, size: u64) -> Result<String, NrgError> {
    let mut handle = fd.take(size);
    let mut string = String::new();
    try!(handle.read_to_string(&mut string));
    Ok(string)
}


/// Reads a 64-bit unsigned integer from `fd`.
pub fn read_u64(fd: &mut File) -> Result<u64, NrgError> {
    let mut buf = [0u8; 8];
    try!(fd.read_exact(&mut buf));
    let i: u64;
    unsafe {
        i = mem::transmute(buf);
    }
    Ok(u64::from_be(i))
}


/// Reads a 32-bit unsigned integer from `fd`.
pub fn read_u32(fd: &mut File) -> Result<u32, NrgError> {
    let mut buf = [0u8; 4];
    try!(fd.read_exact(&mut buf));
    let i: u32;
    unsafe {
        i = mem::transmute(buf);
    }
    Ok(u32::from_be(i))
}


/// Reads a 16-bit unsigned integer from `fd`.
pub fn read_u16(fd: &mut File) -> Result<u16, NrgError> {
    let mut buf = [0u8; 2];
    try!(fd.read_exact(&mut buf));
    let i: u16;
    unsafe {
        i = mem::transmute(buf);
    }
    Ok(u16::from_be(i))
}


/// Reads an unsigned byte from `fd`.
pub fn read_u8(fd: &mut File) -> Result<u8, NrgError> {
    let mut buf = [0u8; 1];
    try!(fd.read_exact(&mut buf));
    Ok(buf[0])
}
