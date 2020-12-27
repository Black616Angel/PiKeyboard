use std::fs::OpenOptions;
use std::io::prelude::*;

pub mod constants;
use crate::constants::*;

fn main() {
	let hid_path = "/dev/hidg0";
	let mut file = OpenOptions::new().write(true).open(hid_path).unwrap();
	file.write(&Key::simple(A)[..]).expect("write failed");
	file.flush().unwrap();
	file.write(&ENDINPUT).expect("write failed");
	file.flush().unwrap();
}
