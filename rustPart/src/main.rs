pub mod keys;
use crate::keys::*;

fn main() -> Result<(),std::io::Error> {
	let hid_path = "/dev/hidg0".to_string();
	let mut lc_key = Key::new(hid_path);
	lc_key.write("A")?;
	Ok(())
}
