#![allow(non_camel_case_types)]

//! Module for fcntl.h
//! Includes interface for a subset of functions and constants declared in the Linux fcntl.h

use super::ffi;
use std::os::raw::*;
use std::os::unix::io::RawFd;
use std::io::Error;

pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const O_NONBLOCK: c_int = 2048;
pub const EOF: c_int = -1;

/// Wrapper for `fcntl` Linux libc function.
///
/// Returns either the output from `fcntl` or an Error if `-1` is returned. 
pub fn fcntl(fd: RawFd, cmd: c_int, flags: Option<c_int>) -> Result<c_int,Error> {
    unsafe { 
        let flags = flags.unwrap_or_default();
        match ffi::fcntl(fd, cmd, flags) {
            -1 => Err(Error::last_os_error()),
            other => Ok(other)
        }
    }
}
