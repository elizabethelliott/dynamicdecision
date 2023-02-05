use iced::Alignment;
use iced::widget::{Button, Column, Row, Text, TextInput};
use iced::Element;

use iced::Length;

use iced_native::Widget;
use surface_dial_rs::events::TopLevelEvent;

use crate::Message;

use crate::views::ScreenCommand;
use crate::views::DialView;

use super::ExperimentData;
use super::Printable;

pub enum TextInputType {
    Alphanumeric,
    Number,
    Characters,
    All
}

struct TextData {
    name: String,
    text: String,
}

impl TextData {
    pub fn new(name: String) -> TextData {
        TextData {
            name,
            text: "".to_string(),
        }
    }
}

impl ExperimentData for TextData {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn headers(&self) -> String {
        "text".to_string()
    }

    fn data(&self) -> Box<&dyn super::Printable> {
        Box::new(self)
    }
}

impl Printable for TextData {
    fn to_csv(&self) -> String {
        self.text.clone()
    }
}

pub struct TextInputView {
    input_type: TextInputType,
    title: String,
    label: String,
    hint: String,
    text_value: String,
    data: TextData,
}

impl TextInputView {
    pub fn new(input_type: TextInputType, name: String, title: String, label: String, hint: String) -> TextInputView {
        TextInputView {
            input_type,
            title,
            label,
            hint,
            text_value: "".to_string(),
            data: TextData::new(name.clone())
        }
    }
}

impl DialView for TextInputView {
    fn init(&mut self) {
        self.text_value = "".to_string();
        self.data = TextData::new(self.data.name.clone());
    }

    fn update(&mut self, _msg: Option<TopLevelEvent>) -> ScreenCommand {
        ScreenCommand::None
    }

    fn view(&self) -> Element<Message> {
        let mut column = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(32)
            .align_items(Alignment::Center)
            .push(Text::new(self.title.clone()).size(30))
            .push(Text::new("\n").size(22).height(Length::Units(10)))
            .push(Text::new(self.label.clone()).size(18).height(Length::Shrink))
            .push(Text::new("\n").size(22).height(Length::Units(10)))
            .push(TextInput::new(self.hint.clone().as_str(), &self.text_value, Message::TextInputChanged).padding(7).width(Length::Units(380)));

        column = column.push(Text::new("\n").size(22).height(Length::Fill));

        let mut next_button = Button::new(Text::new("Next"));

        if !self.text_value.is_empty() {
            next_button = next_button.on_press(Message::ButtonPressed);
        }

        column = column.push(Column::new()
            .align_items(Alignment::End)
            .width(Length::Fill)
            .height(Length::Shrink)
            .push(next_button)
        );

        column.into()
    }

    fn show(&mut self) {

    }

    fn hide(&mut self) {
        
    }

    fn data(&self) -> Option<Box<&dyn super::ExperimentData>> {
        Some(Box::new(&self.data))
    }

    fn arc_settings(&self) -> Option<super::ArcSettings> {
        None
    }

    fn iced_input(&mut self, msg: Message) -> ScreenCommand {
        match msg {
            Message::TextInputChanged(s) => {
                let mut valid_input = false;

                match self.input_type {
                    TextInputType::Alphanumeric => {
                        valid_input = s.chars().all(char::is_alphanumeric);
                    },
                    TextInputType::Number => {
                        valid_input = s.chars().all(char::is_numeric);
                    },
                    TextInputType::Characters => {
                        valid_input = s.chars().all(char::is_alphabetic);
                    },
                    TextInputType::All => {
                        valid_input = true;
                    }
                    _ => { }
                }

                if valid_input {
                    self.text_value = s.clone();
                    self.data.text = s.clone();
                }
            },
            Message::ButtonPressed => {
                return ScreenCommand::NextScreen(None);
            },
            _ => {

            }
        }

        ScreenCommand::None
    }
}