use iced::Alignment;
use iced::widget::{Button, Column, Row, Radio, Text};
use iced::Element;

use iced::Length;
use iced::alignment::Vertical;
use surface_dial_rs::events::{DialEvent, DialDirection, TopLevelEvent};

use crate::Message;

use crate::views::ScreenCommand;
use crate::views::DialView;

use super::ExperimentData;
use super::Printable;

struct ChoiceData {
    name: String,
    selection: u32,
    label: String,
}

impl ChoiceData {
    pub fn new(name: String) -> ChoiceData {
        ChoiceData {
            name,
            selection: 0,
            label: String::default()
        }
    }
}

impl ExperimentData for ChoiceData {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn headers(&self) -> String {
        "index,label".to_string()
    }

    fn data(&self) -> Box<&dyn super::Printable> {
        Box::new(self)
    }
}

impl Printable for ChoiceData {
    fn to_csv(&self) -> String {
        format!("{}, {}", self.selection, self.label).to_string()
    }
}

pub struct MultiChoiceView {
    title: String,
    message: String,
    choices: Vec<(u32, String)>,
    current_choice: Option<u32>,
    data: ChoiceData,
}

impl MultiChoiceView {
    pub fn new(name: String, title: String, message: String, choices: Vec<(u32, String)>) -> MultiChoiceView {
        MultiChoiceView {
            title,
            message,
            choices,
            current_choice: None,
            data: ChoiceData::new(name),
        }
    }
}

impl DialView for MultiChoiceView {
    fn init(&mut self) {
        self.current_choice = None;
        self.data = ChoiceData::new(self.data.name.clone());
    }

    fn update(&mut self, msg: Option<TopLevelEvent>) -> ScreenCommand {
        ScreenCommand::None
    }

    fn view(&self) -> Element<Message> {
        let title = self.title.clone();
        let message = self.message.clone();

        let mut column = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(32)
            .align_items(Alignment::Center)
            .push(Text::new(title).size(30))
            .push(Text::new("\n").size(22).height(Length::Units(10)))
            .push(Text::new(message).size(18).height(Length::Shrink))
            .push(Text::new("\n").size(22).height(Length::Units(10)));

        for c in self.choices.iter() {
            column = column.push(Radio::new(c.0, c.1.clone(), self.current_choice, Message::RadioSelected).text_size(16).width(Length::Units(480)));
        }

        column = column.push(Text::new("\n").size(22).height(Length::Fill));

        let mut next_button = Button::new(Text::new("Next"));

        if let Some(_c) = self.current_choice {
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
        Some(super::ArcSettings {
            divisions: 0
        })
    }

    fn iced_input(&mut self, msg: Message) -> ScreenCommand {
        match msg {
            Message::RadioSelected(c) => {
                self.current_choice = Some(c);

                self.data.selection = c;
                self.data.label = "Unknown".to_string();

                for ch in self.choices.iter() {
                    if ch.0 == c {
                        self.data.label = ch.1.clone().replace(",", ";");
                        break;
                    }
                }
            },
            Message::ButtonPressed => {
                return ScreenCommand::NextScreen(None);
            }
            _ => { }
        }

        ScreenCommand::None
    }
}
