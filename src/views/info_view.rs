use iced::Alignment;
use iced::Column;
use iced::Element;

use iced::Length;
use iced::Text;
use surface_dial_rs::events::{DialEvent, DialDirection, TopLevelEvent};

use crate::Message;

use crate::views::ScreenCommand;
use crate::views::DialView;

pub struct InfoView {
    title: String,
    message: String
}

impl InfoView {
    pub fn new(title: String, message: String) -> InfoView {
        InfoView {
            title,
            message
        }
    }
}

impl DialView for InfoView {
    fn init(&mut self) {
        
    }

    fn update(&mut self, msg: Option<TopLevelEvent>) -> ScreenCommand {
        match msg {
            Some(e) => {
                if let TopLevelEvent::DialEvent(DialEvent::Button { pressed }) = &e {
                    if *pressed {
                        return ScreenCommand::NextScreen;
                    }
                }
            },
            _ => {}
        }

        ScreenCommand::None
    }

    fn view(&mut self) -> Element<Message> {
        let title = self.title.clone();
        let message = self.message.clone();

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .align_items(Alignment::Center)
            .push(Text::new(title).size(30))
            .push(Text::new(message).size(18))
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
        Some(super::ArcSettings {
            divisions: 0
        })
    }
}
