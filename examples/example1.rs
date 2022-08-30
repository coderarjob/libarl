use libarl::*;
use std::os::unix::io::*;

const KEY_UP: &str = "\x1b[A";
const KEY_RIGHT: &str = "\x1b[C";
const KEY_DOWN: &str = "\x1b[B";
const KEY_LEFT: &str = "\x1b[D";

fn enter_raw_mode() {
    let stdin_fd = std::io::stdin().as_raw_fd();

    // Non Blocking read
    fcntl (stdin_fd, F_SETFL, Some(O_NONBLOCK)).unwrap();

    // Switch to raw mode (Non cannonical and no echo)
    let mut t:Termios = Default::default();
    tcgetattr (stdin_fd, &mut t).unwrap();

    t.c_lflag &= !(ECHO);
    tcsetattr (stdin_fd, TCSAFLUSH, &t).unwrap();
}

fn main() {
    enter_raw_mode();

    let mut key_bytes = String::default();
    loop {
        match getchar() {
            Err(_) => {
                match &key_bytes[..] {
                    "" => (),
                    "q" => break,
                    "\n" => println!("Return key was pressed."),
                    KEY_UP => println!("Up arrow was pressed."),
                    KEY_DOWN => println!("Down arrow was pressed."),
                    KEY_LEFT => println!("Left arrow was pressed."),
                    KEY_RIGHT => println!("Right arrow was pressed."),
                    string_val => println!("{}", string_val),
                }
                key_bytes.clear();
            },
            Ok(c) => key_bytes.push(c)
        }
    }
}
