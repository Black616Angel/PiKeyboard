pub mod keys;
pub mod gui;
pub mod keyboard;

use iced::Settings;
use iced::Sandbox;

use crate::gui::*;

fn main() {
	PiKeyboardGUI::run(Settings::default()).unwrap();
}
