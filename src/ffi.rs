//! Module for external libc calls
//! Includes interface for a subset of Linux libc functions.

use super::termios::Termios;
use std::os::raw::*;

extern "C" {
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;

    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    pub fn getchar() -> c_int;

    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    pub fn tcgetattr(fd: c_int, termios_p: *mut Termios) -> c_int;

    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    pub fn tcsetattr(fd: c_int, actions: c_int, termios_p: *const Termios) -> c_int;
}
