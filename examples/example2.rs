use libarl::*;
use std::os::unix::io::*;

fn main() {

    // Non Blocking read
    let stdin_fd = std::io::stdin().as_raw_fd();
    fcntl (stdin_fd, F_SETFL, Some(O_NONBLOCK)).unwrap();

    println! ("{:?}", getchar());
}
