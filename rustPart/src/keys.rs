
use std::fs::OpenOptions;
use std::io::prelude::*;



pub const LEFT_CTRL: u8 = 0x01;
pub const RIGHT_CTRL: u8 = 0x10;
pub const LEFT_SHIFT: u8 = 0x02;
pub const RIGHT_SHIFT: u8 = 0x20;
pub const LEFT_ALT: u8 = 0x04;
pub const RIGHT_ALT: u8 = 0x40;
pub const LEFT_META: u8 = 0x08;
pub const RIGHT_META: u8 = 0x80;

pub const NULL: u8 = 0x00;
pub const A: u8 = 0x04;
pub const B: u8 = 0x05;
pub const C: u8 = 0x06;
pub const D: u8 = 0x07;
pub const E: u8 = 0x08;
pub const F: u8 = 0x09;
pub const G: u8 = 0x0a;
pub const H: u8 = 0x0b;
pub const I: u8 = 0x0c;
pub const J: u8 = 0x0d;
pub const K: u8 = 0x0e;
pub const L: u8 = 0x0f;
pub const M: u8 = 0x10;
pub const N: u8 = 0x11;
pub const O: u8 = 0x12;
pub const P: u8 = 0x13;
pub const Q: u8 = 0x14;
pub const R: u8 = 0x15;
pub const S: u8 = 0x16;
pub const T: u8 = 0x17;
pub const U: u8 = 0x18;
pub const V: u8 = 0x19;
pub const W: u8 = 0x1a;
pub const X: u8 = 0x1b;
pub const Y: u8 = 0x1c;
pub const Z: u8 = 0x1d;
pub const ONE: u8 = 0x1e;
pub const TWO: u8 = 0x1f;
pub const THREE: u8 = 0x20;
pub const FOUR: u8 = 0x21;
pub const FIVE: u8 = 0x22;
pub const SIX: u8 = 0x23;
pub const SEVEN: u8 = 0x24;
pub const EIGHT: u8 = 0x25;
pub const NINE: u8 = 0x26;
pub const ZERO: u8 = 0x27;

pub const RETURN: u8 = 0x28;
pub const ESC: u8 = 0x29;
pub const BACKSPACE: u8 = 0x2a;
pub const TAB: u8 = 0x2b;
pub const SPACE: u8 = 0x2c;
pub const MINUS: u8 = 0x2d;
pub const EQUALS: u8 = 0x2e;
pub const LBRACKET: u8 = 0x2f;
pub const RBRACKET: u8 = 0x30;
pub const BACKSLASH: u8 = 0x31;
pub const HASH: u8 = 0x32;
pub const SEMICOLON: u8 = 0x33;
pub const QUOTE: u8 = 0x34;
pub const TILDE: u8 = 0x35;
pub const COMMA: u8 = 0x36;
pub const PERIOD: u8 = 0x37;
pub const SLASH: u8 = 0x38;
pub const CAPS_LOCK: u8 = 0x39;
pub const F1: u8 = 0x3a;
pub const F2: u8 = 0x3b;
pub const F3: u8 = 0x3c;
pub const F4: u8 = 0x3d;
pub const F5: u8 = 0x3e;
pub const F6: u8 = 0x3f;
pub const F7: u8 = 0x40;
pub const F8: u8 = 0x41;
pub const F9: u8 = 0x42;
pub const F10: u8 = 0x43;
pub const F11: u8 = 0x44;
pub const F12: u8 = 0x45;
pub const PRINT: u8 = 0x46;
pub const SCROLL_LOCK: u8 = 0x47;
pub const PAUSE: u8 = 0x48;
pub const INSERT: u8 = 0x49;
pub const HOME: u8 = 0x4a;
pub const PAGEUP: u8 = 0x4b;
pub const DELETE: u8 = 0x4c;
pub const END: u8 = 0x4d;
pub const PAGEDOWN: u8 = 0x4e;
pub const RIGHT: u8 = 0x4f;
pub const LEFT: u8 = 0x50;
pub const DOWN: u8 = 0x51;
pub const UP: u8 = 0x52;
pub const NUM_LOCK: u8 = 0x53;
pub const KP_DIVIDE: u8 = 0x54;
pub const KP_MULTIPLY: u8 = 0x55;
pub const KP_MINUS: u8 = 0x56;
pub const KP_PLUS: u8 = 0x57;
pub const KP_ENTER: u8 = 0x58;
pub const KP1: u8 = 0x59;
pub const KP2: u8 = 0x5a;
pub const KP3: u8 = 0x5b;
pub const KP4: u8 = 0x5c;
pub const KP5: u8 = 0x5d;
pub const KP6: u8 = 0x5e;
pub const KP7: u8 = 0x5f;
pub const KP8: u8 = 0x60;
pub const KP9: u8 = 0x61;
pub const KP0: u8 = 0x62;
pub const KP_PERIOD: u8 = 0x63;
pub const APPLICATION: u8 = 0x65;
pub const POWER: u8 = 0x66;
pub const KP_EQUALS: u8 = 0x67;
pub const F13: u8 = 0x68;
pub const F14: u8 = 0x69;
pub const F15: u8 = 0x6a;
pub const F16: u8 = 0x6b;
pub const F17: u8 = 0x6c;
pub const F18: u8 = 0x6d;
pub const F19: u8 = 0x6e;
pub const F20: u8 = 0x6f;
pub const F21: u8 = 0x70;
pub const F22: u8 = 0x71;
pub const F23: u8 = 0x72;
pub const F24: u8 = 0x73;
pub const EXECUTE: u8 = 0x74;
pub const HELP: u8 = 0x75;
pub const MENU: u8 = 0x76;
pub const SELECT: u8 = 0x77;
pub const CANCEL: u8 = 0x78;
pub const REDO: u8 = 0x79;
pub const UNDO: u8 = 0x7a;
pub const CUT: u8 = 0x7b;
pub const COPY: u8 = 0x7c;
pub const PASTE: u8 = 0x7d;
pub const FIND: u8 = 0x7e;
pub const MUTE: u8 = 0x7f;
// These are multimedia keys, they will not work on standard keyboard; they need a different USB descriptor
pub const VOLUME_UP: u8 = 0x80;
pub const VOLUME_DOWN: u8 = 0x81;


pub const ENDINPUT: [u8; 8] = [NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL];

#[derive(Debug)]
pub struct Key {
	file: std::fs::File,
}

impl Key {
	fn simplem(key: u8, modi: u8) -> Vec<u8> {
		vec![modi, NULL, key, NULL, NULL, NULL, NULL, NULL ]
	}
	fn multim(keys: Vec<u8>, modi: u8) -> Vec<u8> {
		let mut ret: Vec<u8> = vec![modi, NULL];
		for i in 0..3 {
			if i <= keys.len()-1 {
				ret.push(keys[i]);
			} else {
				ret.push(NULL);
			}
		}
		ret
	}

	fn get_key(key: String) -> Result<u8,std::io::Error> {
		let mut ret: u8 = NULL;
		if key.len() == 1 {
			let chars: Vec<char> = key.chars().collect();
			let keyb = chars[0] as u8;
			ret = match keyb {
				32		=> SPACE,
				// 33		=> ??
				58      => ZERO,
				49..=57 => keyb-19,
				65..=90 => keyb-61, //capital letters
				97..=122 => keyb-93, //small letters
				_ => NULL,
			};
		}
		Ok(ret)
	}

	fn get_modi(key: String) -> Result<u8,std::io::Error> {
		let mut ret: u8 = NULL;
		if key.len() == 1 {
			let chars: Vec<char> = key.chars().collect();
			let keyb = chars[0] as u8;
			ret = match keyb {
				65..=90 => RIGHT_SHIFT, //capital letters
				_ => NULL,
			};
		}
		Ok(ret)
	}
	pub fn std_write(keys: String) {
		let k = Key::new( "/dev/hidg0".to_string());
		if k.is_err() {
			println!("{:?}", keys);
		} else {
			let res = k.unwrap().write(&keys);
			if res.is_err() {
				println!("{:?}", keys);
			}
		}
	}

	pub fn new(device: String) -> Result<Key, std::io::Error> {
		Ok(Key{ file: OpenOptions::new().write(true).open(device)? })
	}
	fn flush(&mut self) -> Result<(),std::io::Error> {
		self.file.flush()?;
		self.file.write(&ENDINPUT)?;
		self.file.flush()?;
		Ok(())
	}
	fn write_simplem(&mut self, key: u8, modi: u8) -> Result<(),std::io::Error> {
		self.file.write(&Key::simplem(key, modi)[..])?;
		self.flush()?;
		Ok(())
	}
	fn write_multim(&mut self, keys: Vec<u8>, modi: u8) -> Result<(),std::io::Error> {
		self.file.write(&Key::multim(keys, modi)[..])?;
		self.flush()?;
		Ok(())
	}
	pub fn write(&mut self, all_keys: &str) -> Result<(),std::io::Error> {
		for keys in all_keys.split(" ") {
			if !keys.contains("\\") && !keys.contains("+") {
				self.write_simplem(Key::get_key(keys.to_string())?, Key::get_modi(keys.to_string())?)?;
			} else if !keys.contains("\\") {
				let mut keyvec: Vec<u8> = Vec::new();
				let mut modivec: Vec<u8> = Vec::new();
				let mut modi: u8 = NULL;
				for s in keys.split("+") {
					keyvec.push(Key::get_key(s.to_string())?);
					modivec.push(Key::get_modi(s.to_string())?);
				}
				for m in modivec {
					modi = m;
				}

				self.write_multim(keyvec,modi)?;
			}
		}
		Ok(())
	}
}