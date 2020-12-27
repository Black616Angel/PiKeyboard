use iced::{Column, Row, TextInput, Element, Sandbox, text_input, Container, Length, Scrollable, scrollable};

use crate::keyboard::*;
use crate::keys::*;

#[derive(Default)]
pub struct PiKeyboardGUI {
    search:         text_input::State,
    scroll_content: scrollable::State,
    search_text:    String,
    content:        Keyboard,
    keyboards:      Vec::<Keyboard>,
}

#[derive(Debug, Clone)]
pub enum KeyboardMessage {
    Search(String),
    Select(Keyboard),
    Do(String),
}

impl PiKeyboardGUI {
    pub fn push_list(&mut self, path: String) {
        self.keyboards.push( Keyboard::new(path) );
    }
}

impl Sandbox for PiKeyboardGUI {
    type Message = KeyboardMessage;

    fn new() -> Self {
        let mut ret = Self::default();
        for path in Keyboard::get_list() {
            ret.push_list(path.path);
        }
        ret
    }

    fn title(&self) -> String {
        String::from("PiKeyboard")
    }

    fn update(&mut self, message: KeyboardMessage) {
        match message {
            KeyboardMessage::Search(search_text) => {
                self.search_text = search_text;
            }
            KeyboardMessage::Select(kb) => {
                self.content = kb;
            }
            KeyboardMessage::Do(cmd) => {
                Key::std_write(cmd);
            }
        }
    }

    fn view(&mut self) -> Element<KeyboardMessage> {
        let kblist: Element<_> = self.keyboards.iter_mut().fold(Column::new().spacing(20), |column, ikb| {
                            column.push(ikb.menu_view())
                        }).into();
        let menu = Container::new(Column::new().push(TextInput::new(&mut self.search, "Search", &self.search_text.to_string(), KeyboardMessage::Search,)
                    .on_submit(KeyboardMessage::Search(self.search_text.to_string())))
                .push(kblist)).width(Length::Fill);
        let details = Container::new(
                    Column::new().push(Scrollable::new(&mut self.scroll_content)
                                        .push(self.content.view()).padding(20)
                                      )
                                 ).width(Length::FillPortion(5));
        Row::new().push(
                menu
            ).push(
                details
            ).into()
    }
}
