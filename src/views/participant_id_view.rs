use iced::Alignment;
use iced::Button;
use iced::Column;
use iced::Element;

use iced::Length;
use iced::Row;
use iced::Text;

use iced::TextInput;
use surface_dial_rs::events::TopLevelEvent;

use crate::Message;

use crate::views::ScreenCommand;
use crate::views::DialView;

pub struct ParticipantIdView {
    text_value: String,
    text_state: iced::text_input::State,
    button_state: iced::button::State,
}

impl ParticipantIdView {
    pub fn new() -> ParticipantIdView {
        ParticipantIdView {
            text_value: "".to_string(),
            text_state: iced::text_input::State::new(),
            button_state: iced::button::State::new(),
        }
    }
}

impl DialView for ParticipantIdView {
    fn init(&mut self) {

    }

    fn update(&mut self, _msg: Option<TopLevelEvent>) -> ScreenCommand {
        ScreenCommand::None
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40)
            .align_items(Alignment::Center)
            .push(Text::new("Enter the participant ID to begin").size(30))
            .push(Row::new()
                .width(Length::Fill)
                .padding(40)
                .push(TextInput::new(&mut self.text_state, "Enter participant id...", &self.text_value, Message::TextInputChanged) .padding(7))
                .push(Button::new(&mut self.button_state, Text::new("Submit")).on_press(Message::ButtonPressed)))
            .into()
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
                self.text_value = s;
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