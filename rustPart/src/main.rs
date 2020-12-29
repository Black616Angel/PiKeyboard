pub mod keys;
use crate::keys::*;

fn main() {
	Key::std_write(std::env::args().nth(1).expect("no string given"));
}
