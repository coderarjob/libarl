//! ARL: Arjob's Rust Library
//!
//! This library contains various common functions that are not provided by the Rust standard 
//! library, but are also common enough for my use.
//! At this point this library contains mostly wrapper functions for libc counterparts.

mod fcntl;
mod termios;
mod stdio;
pub mod ffi;

pub use fcntl::*;
pub use termios::*;
pub use stdio::*;
