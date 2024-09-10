use crate::error::BitBitError;
use std::io::Write;

pub const BASE: usize = 256;
#[cfg(target_pointer_width = "16")]
pub const P_WIDTH: usize = 2;
#[cfg(target_pointer_width = "32")]
pub const P_WIDTH: usize = 4;
#[cfg(target_pointer_width = "64")]
pub const P_WIDTH: usize = 8;

/// Shift pointer right with memory adjustment
#[inline]
pub fn shift_right(memory: &mut Vec<u8>, pointer: &mut usize, amount: usize) {
    // increment pointer
    *pointer += amount;
    // extend memory when needed
    adjust_size(memory, pointer);
}

/// Shift pointer left with bounds check
#[inline]
pub fn shift_left(pointer: &mut usize, amount: usize) -> Result<(), BitBitError> {
    if amount > *pointer {
        // case: left shift below 0
        Err(BitBitError::err("out of bounds shift left"))
    } else {
        // decrement pointer
        Ok(*pointer -= amount)
    }
}

/// Output character at pointer
#[inline]
pub fn output(memory: &Vec<u8>, pointer: &usize, amount: usize) -> Result<(), BitBitError> {
    // print ascii
    for _ in 0..amount {
        putch(memory[*pointer] as core::ffi::c_char);
    }

    // flush after print
    match std::io::stdout().flush() {
        Ok(_) => Ok(()),
        Err(e) => Err(BitBitError::hint("fail to flush stdout", e)),
    }
}

/// Store 1 byte input at pointer
#[inline]
pub fn input(memory: &mut Vec<u8>, pointer: &usize, amount: usize) {
    for _ in 0..amount {
        // get 1 character input
        (*memory)[*pointer] = getch() as u8;
    }
}

/// Set cell at pointer to 0
#[inline]
pub fn set_ptr_zero(memory: &mut Vec<u8>, pointer: &usize) {
    (*memory)[*pointer] = 0;
}

/// Expand memory to pointer
#[inline]
fn adjust_size(memory: &mut Vec<u8>, pointer: &usize) {
    while memory.len() <= *pointer {
        memory.push(0);
    }
}

/// Increment cell at pointer with looping
#[inline]
pub fn increment(cell: &mut u8, amount: usize) {
    *cell = ((amount + *cell as usize) % BASE) as u8;
}

/// Decrement value at pointer with looping
#[inline]
pub fn decrement(cell: &mut u8, amount: usize) {
    if amount > *cell as usize {
        *cell = (BASE - (amount % BASE) - *cell as usize) as u8;
    } else {
        *cell -= amount as u8;
    }
}

// I/O for brainfuck
extern "C" {
    fn _getch() -> core::ffi::c_char;
    fn _putch(c_char: core::ffi::c_char) -> core::ffi::c_void;
}

// Read 1 byte input
#[inline]
fn getch() -> core::ffi::c_char {
    // SAFETY: trust C
    unsafe { _getch() }
}

// Write 1 byte output
#[inline]
fn putch(c_char: core::ffi::c_char) -> core::ffi::c_void {
    // SAFETY: trust C
    unsafe { _putch(c_char) }
}
