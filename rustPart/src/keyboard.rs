use crate::gui::*;
use serde::{Serialize, Deserialize};
use iced::{button, Column, Row, Element, Text, Button, Length};


use std::fs;

#[derive(Debug, Clone)]
pub enum KeyboardState {
    IsSelected {
        text_name: String,
    },
    NotSelected {
        select_button: button::State,
    }
}

#[derive(Debug)]
pub struct PathName {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct Keyboard {
    pub name: String,
    pub path: String,
    pub selected: KeyboardState,
    pub keys: Vec<GUIKey>,
}

#[derive(Debug, Clone)]
pub struct GUIKey {
    name: String,
    cmd: String,
    button: button::State,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonKeyboard {
    name: String,
    keys: Vec<JsonKey>,
}


#[derive(Serialize, Deserialize, Debug)]
struct JsonKey {
    name: String,
    keys: String,
    positionx: u32,
    positiony: u32,
}

impl GUIKey {
    pub fn new(name: String, cmd: String) -> GUIKey {
        GUIKey{ name, cmd, button: button::State::new()}
    }
    pub fn view(&mut self)  -> Element<KeyboardMessage> {
        Row::new()
        .spacing(20)
        .push(
            Button::new(&mut self.button,
                Text::new(self.name.to_string())
                ).on_press(KeyboardMessage::Do(self.cmd.to_string())),
            ).into()
    }
}

impl Default for Keyboard {
    fn default() -> Keyboard {
        let selected = KeyboardState::NotSelected{ select_button: button::State::new(), };
        let keys: Vec<GUIKey> = Vec::new();
        Keyboard{ name: "default".to_string(), path: "./keyboards/default.json".to_string(), selected, keys }
    }
}

impl Keyboard {
    pub fn new(path: String) -> Keyboard {
        let selected = KeyboardState::NotSelected{ select_button: button::State::new(), };
        let mut keys: Vec<GUIKey> = Vec::new();
        let json: JsonKeyboard = serde_json::from_str(&std::fs::read_to_string(path.to_string()).unwrap()).unwrap();
        for k in json.keys {
            keys.push(GUIKey::new(k.name, k.keys));
        }
        Keyboard{ name: json.name, path, selected, keys }

    }
    pub fn get_list() -> Vec<PathName> {
        let mut list = Vec::<PathName>::new();
        let paths = fs::read_dir("./keyboards/").unwrap();

        for pa in paths {
            let pat = pa.unwrap();
            let path = pat.path().display().to_string();
            let name = pat.file_name().to_str().unwrap().to_string();
            list.push(PathName { name, path });
        }
        list
    }

    pub fn menu_view(&mut self) -> Element<KeyboardMessage> {
        match &mut self.selected {
            KeyboardState::NotSelected{ select_button } => {
                Row::new()
                .spacing(20)
                .push(
                    Button::new(select_button,
                        Text::new(self.name.to_string())
                        ).on_press(KeyboardMessage::Select(Keyboard::new(self.path.to_string())),)
                    ).into()
            },
            KeyboardState::IsSelected{ text_name } => {
                Row::new()
                .spacing(10)
                .push(Text::new(text_name.to_string()))
                .into()
            },
        }
    }
    pub fn view(&mut self) -> Element<KeyboardMessage> {
        println!("{:?}", self.keys.len().to_string());
        let keylist: Element<_> = self.keys.iter_mut().fold(Column::new().spacing(20), |column, ikey| {
            column.push(ikey.view())
        }).into();
        Column::new()
        .push(keylist).width(Length::Fill).into()
    }
}