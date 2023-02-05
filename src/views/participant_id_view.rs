use std::collections::HashMap;

use iced::Alignment;
use iced::widget::{Button, Column, Text, Row};
use iced::Element;

use iced::Length;

use iced::widget::TextInput;
use surface_dial_rs::events::TopLevelEvent;

use crate::Message;

use crate::views::ScreenCommand;
use crate::views::DialView;

pub struct ParticipantIdView {
    text_value: String,
}

impl ParticipantIdView {
    pub fn new() -> ParticipantIdView {
        ParticipantIdView {
            text_value: "".to_string(),
        }
    }
}

impl DialView for ParticipantIdView {
    fn init(&mut self) {
        self.text_value = String::default();
    }

    fn update(&mut self, _msg: Option<TopLevelEvent>) -> ScreenCommand {
        ScreenCommand::None
    }

    fn view(&self) -> Element<Message> {

        let mut column = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40)
            .align_items(Alignment::Center)
            .push(Text::new("Enter the participant ID to begin").size(30));

        let mut submit_button = Button::new(Text::new("Submit"));

        if self.text_value.len() > 0 {
            submit_button = submit_button.on_press(Message::ButtonPressed);
        }

        column = column.push(Row::new()
            .width(Length::Fill)
            .padding(40)
            .push(TextInput::new("Enter participant id...", &self.text_value, Message::TextInputChanged) .padding(7))
            .push(submit_button));
            
        column.into()
    }

    fn show(&mut self) {
        
    }

    fn hide(&mut self) {
        
    }

    fn data(&self) -> Option<Box<&dyn super::ExperimentData>> {
        None
    }

    fn arc_settings(&self) -> Option<super::ArcSettings> {
        None
    }

    fn iced_input(&mut self, msg: Message) -> ScreenCommand {
        match msg {
            Message::TextInputChanged(s) => {
                if s.chars().all(char::is_numeric) {
                    self.text_value = s;
                }
            },
            Message::ButtonPressed => {
                return ScreenCommand::NextScreen(Some(HashMap::from([
                    ("id".to_string(), self.text_value.clone())
                ])));
            },
            _ => {

            }
        }

        ScreenCommand::None
    }
}