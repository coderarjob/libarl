#![allow(non_camel_case_types)]

//! Module for termios.h
//! Includes interface for a subset of functions and constants declared in the Linux termios.h

use super::ffi;
use std::os::raw::*;
use std::os::unix::io::RawFd;
use std::io::Error;

pub type tcflag_t = u32;
pub type speed_t = u32;
pub type cc_t = u8;

pub const ECHO: tcflag_t = 0x8;
pub const ICANON: tcflag_t = 0x2;

pub const VTIME: usize = 0x5;
pub const VMIN: usize = 0x6;
pub const NCCS: usize = 32;

pub const TCSAFLUSH: c_int = 0x2;

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; NCCS],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

/// Wrapper for `tcgetattr` Linux libc function.
///
/// Returns Error if `-1` is returned. On success the `termios_p` structure is filled.
///
/// # Panics
/// If anything other than 0 or -1 is returned from `tcgetattr`.
pub fn tcgetattr(fd: RawFd, termios_p: &mut Termios) -> Result<(),Error> {
    unsafe {
        match ffi::tcgetattr (fd, termios_p) {
            -1 => Err(Error::last_os_error()),
            0 => Ok(()),
            _ => unreachable! ()
        }
    }
}

/// Wrapper for `tcsetattr` Linux libc function.
///
/// Returns Error if `-1` is returned. 
///
/// # Panics
/// If anything other than 0 or -1 is returned from `tcsetattr`.
pub fn tcsetattr(fd: RawFd, actions: c_int, termios_p: &Termios) -> Result<(),Error> {
    unsafe {
        match ffi::tcsetattr (fd, actions, termios_p) {
            -1 => Err(Error::last_os_error()),
            0 => Ok(()),
            _ => unreachable! ()
        }
    }
}
