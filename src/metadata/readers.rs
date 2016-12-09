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

//! Miscellaneous functions to read fixed-size data from a file.

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::mem;

use ::error::NrgError;


/// Reads a String of `size` bytes from `fd`.
///
/// The string will be truncated at the first null byte encountered; therefore,
/// its length may be less than `size` characters.
pub fn read_sized_string(fd: &mut File, size: usize)
                         -> Result<String, NrgError> {
    // Read size bytes
    let mut bytes = vec!(0u8; size);
    try!(fd.read_exact(&mut bytes));

    // Truncate the vector at the first null byte
    let mut i: usize = 0;
    for b in bytes.iter() {
        if *b == 0 { break; }
        i += 1;
    }
    bytes.truncate(i);

    let cstring = CString::new(bytes)
        .expect("This Vec wasn't supposed to contain any null byte!");

    cstring.into_string().map_err(NrgError::String)
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


/// Reads a BCD-encoded byte from `fd`.
///
/// If the decoded value is more than 99, which is not a valid binary-coded
/// decimal value, the byte read is returned as is, without decoding.
pub fn read_u8_bcd(fd: &mut File) -> Result<u8, NrgError> {
    let byte = try!(read_u8(fd));
    let tens = (byte >> 4) * 10;
    let units = (byte << 4) >> 4;
    let value = tens + units;
    if value < 100 {
        return Ok(value);
    }
    Ok(byte)
}
