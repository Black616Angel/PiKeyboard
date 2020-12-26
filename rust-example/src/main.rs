use std::fs::File;
//use std::io::BufWriter;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
	let hid_path = "/dev/hidg0";
	let mut file = OpenOptions::new().write(true).open(hid_path).unwrap();
//	let mut file = BufWriter::new(File::open(hid_path).expect("HID not found"));
//	let mut file = File::open(hid_path).expect("HID not found");
	let byte_0: u8 = 0;
	let byte_a: u8 = 4;
	let testkey: Vec<u8> = [byte_0,byte_0,byte_a,byte_0,byte_0,byte_0,byte_0,byte_0].to_vec();
	file.write(&testkey).expect("write failed");
	file.flush().unwrap();
	let endinput: Vec<u8> = [byte_0,byte_0,byte_0,byte_0,byte_0,byte_0,byte_0,byte_0].to_vec();
	file.write(&endinput).expect("write failed");
	file.flush().unwrap();
//	println!("Hello, world!");
}
