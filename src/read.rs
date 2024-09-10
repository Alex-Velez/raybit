use crate::{error::BitBitError, memory};
use memchr;
use raylib_ffi::Color;
use std::ffi::CString;

/// Read N pointers at `index` from memory
#[inline]
pub fn ptrs<const N: usize>(memory: &Vec<u8>, index: usize) -> [usize; N] {
    let mut ptrs: [usize; N] = [0; N];
    for i in 0..N {
        ptrs[i] = usize(memory, index + (memory::P_WIDTH * i));
    }
    ptrs
}

/// Read C string at `index` in memory ([A-z]...[0])
#[inline]
pub fn c_string(memory: &Vec<u8>, index: usize) -> Result<CString, BitBitError> {
    if let Some(last_char_index) = memchr::memchr(b'\0', &memory[index..]) {
        match CString::new(&memory[index..(index + last_char_index)]) {
            Ok(c_str) => Ok(c_str),
            Err(e) => Err(BitBitError::hint("convert to CString fail", e)),
        }
    } else {
        Err(BitBitError::err(
            "memchr fail when attempting to read string",
        ))
    }
}

/// Read little-endian array of cells at `index` in memory as unsigned target-bit integer
/// usize (u16): [0][0]
/// usize (u32): [0][0][0][0]
/// usize (u64): [0][0][0][0][0][0][0][0]
#[inline]
pub fn usize(memory: &Vec<u8>, index: usize) -> usize {
    #[cfg(target_pointer_width = "16")]
    return usize::from_le_bytes([memory[index], memory[index + 1]]);
    #[cfg(target_pointer_width = "32")]
    return usize::from_le_bytes([
        memory[index],
        memory[index + 1],
        memory[index + 2],
        memory[index + 3],
    ]);
    #[cfg(target_pointer_width = "64")]
    return usize::from_le_bytes([
        memory[index],
        memory[index + 1],
        memory[index + 2],
        memory[index + 3],
        memory[index + 4],
        memory[index + 5],
        memory[index + 6],
        memory[index + 7],
    ]);
}

/// Read little-endian array of cells at `index` in memory as unsigned target-bit integer
/// isize (i16): [0][0]
/// isize (i32): [0][0][0][0]
/// isize (i64): [0][0][0][0][0][0][0][0]
#[inline]
pub fn _isize(memory: &Vec<u8>, index: usize) -> isize {
    #[cfg(target_pointer_width = "16")]
    return isize::from_le_bytes([memory[index], memory[index + 1]]);
    #[cfg(target_pointer_width = "32")]
    return isize::from_le_bytes([
        memory[index],
        memory[index + 1],
        memory[index + 2],
        memory[index + 3],
    ]);
    #[cfg(target_pointer_width = "64")]
    return isize::from_le_bytes([
        memory[index],
        memory[index + 1],
        memory[index + 2],
        memory[index + 3],
        memory[index + 4],
        memory[index + 5],
        memory[index + 6],
        memory[index + 7],
    ]);
}

/// Read little-endian array of cells at `index` in memory as unsigned 16-bit integer
/// u16: [0][0]
#[inline]
pub fn _u16(memory: &Vec<u8>, index: usize) -> u16 {
    u16::from_le_bytes([memory[index], memory[index + 1]])
}

/// Read little-endian array of cells at `index` in memory as unsigned 32-bit integer
/// u32: [0][0][0][0]
#[inline]
pub fn u32(memory: &Vec<u8>, index: usize) -> u32 {
    u32::from_le_bytes([
        memory[index],
        memory[index + 1],
        memory[index + 2],
        memory[index + 3],
    ])
}

/// Read little-endian array of cells at `index` in memory as unsigned 64-bit integer
/// u64: [0][0][0][0][0][0][0][0]
#[inline]
pub fn _u64(memory: &Vec<u8>, index: usize) -> u64 {
    u64::from_le_bytes([
        memory[index],
        memory[index + 1],
        memory[index + 2],
        memory[index + 3],
        memory[index + 4],
        memory[index + 5],
        memory[index + 6],
        memory[index + 7],
    ])
}

/// Read little-endian array of cells at `index` in memory as unsigned 128-bit integer
/// u128: [0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0]
#[inline]
pub fn _u128(memory: &Vec<u8>, index: usize) -> u128 {
    u128::from_le_bytes([
        memory[index],
        memory[index + 1],
        memory[index + 2],
        memory[index + 3],
        memory[index + 4],
        memory[index + 5],
        memory[index + 6],
        memory[index + 7],
        memory[index + 8],
        memory[index + 9],
        memory[index + 10],
        memory[index + 11],
        memory[index + 12],
        memory[index + 13],
        memory[index + 14],
        memory[index + 15],
    ])
}

/// Read little-endian array of cells at `index` in memory as signed 16-bit integer
/// i16: [0][0]
#[inline]
pub fn _i16(memory: &Vec<u8>, index: usize) -> i16 {
    i16::from_le_bytes([memory[index], memory[index + 1]])
}

/// Read little-endian array of cells at `index` in memory as signed 32-bit integer
/// i32: [0][0][0][0]
#[inline]
pub fn i32(memory: &Vec<u8>, index: usize) -> i32 {
    i32::from_le_bytes([
        memory[index],
        memory[index + 1],
        memory[index + 2],
        memory[index + 3],
    ])
}

/// Read little-endian array of cells at `index` in memory as signed 64-bit integer
/// i64: [0][0][0][0][0][0][0][0]
#[inline]
pub fn _i64(memory: &Vec<u8>, index: usize) -> i64 {
    i64::from_le_bytes([
        memory[index],
        memory[index + 1],
        memory[index + 2],
        memory[index + 3],
        memory[index + 4],
        memory[index + 5],
        memory[index + 6],
        memory[index + 7],
    ])
}

/// Read little-endian array of cells at `index` in memory as signed 128-bit integer
/// i128: [0][0][0][0][0][0][0][0][0][0][0][0][0][0][0][0]
#[inline]
pub fn _i128(memory: &Vec<u8>, index: usize) -> i128 {
    i128::from_le_bytes([
        memory[index],
        memory[index + 1],
        memory[index + 2],
        memory[index + 3],
        memory[index + 4],
        memory[index + 5],
        memory[index + 6],
        memory[index + 7],
        memory[index + 8],
        memory[index + 9],
        memory[index + 10],
        memory[index + 11],
        memory[index + 12],
        memory[index + 13],
        memory[index + 14],
        memory[index + 15],
    ])
}

/// Read Color at `index` in memory (R, G, B, A)
/// Color: [0][0][0][0]
#[inline]
pub fn color(memory: &Vec<u8>, index: usize) -> Color {
    Color {
        r: memory[index],
        g: memory[index + 1],
        b: memory[index + 2],
        a: memory[index + 3],
    }
}
