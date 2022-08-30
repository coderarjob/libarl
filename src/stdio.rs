#![allow(non_camel_case_types)]

//! Module for stdio.h
//! Includes interface for a subset of functions and constants declared in the Linux stdio.h

use super::ffi;

/// Wrapper for the `getchar` Linux libc function.
///
/// Returns either a character from `Stdin` or an Error if EOF was returned. Error can mean two
/// things:
/// 1. A genuine error has occurred. Read `last_os_error()` to get the error code.
/// 2. When using non blocking `Stdin`, error is returned if there are no characters to read.
pub fn getchar() -> Result<char, ()> {
    unsafe {
        match ffi::getchar() {
            super::fcntl::EOF => Err(()),
            other => Ok(char::from_u32(other as u32).unwrap())
        }
    }
}
