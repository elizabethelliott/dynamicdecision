use iced::Alignment;
use iced::Column;
use iced::Element;

use iced::Image;
use iced::Length;
use iced::Text;
use iced_native::image;
use surface_dial_rs::events::{DialEvent, TopLevelEvent};

use crate::Message;

use crate::views::ScreenCommand;
use crate::views::DialView;

pub struct ImageView {
    title: String,
    path: String
}

impl ImageView {
    pub fn new(title: String, path: String) -> ImageView {
        ImageView {
            title,
            path
        }
    }
}

impl DialView for ImageView {
    fn init(&mut self) {
        
    }

    fn update(&mut self, msg: Option<TopLevelEvent>) -> ScreenCommand {
        match msg {
            Some(e) => {
                if let TopLevelEvent::DialEvent(DialEvent::Button { pressed }) = &e {
                    if *pressed {
                        return ScreenCommand::NextScreen(None);
                    }
                }
            },
            _ => {}
        }

        ScreenCommand::None
    }

    fn view(&mut self) -> Element<Message> {
        let title = self.title.clone();
        let path = self.path.clone();

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .align_items(Alignment::Center)
            .push(Text::new(title).size(30))
            .push(Text::new("\n\n\n0").size(22).height(Length::Shrink))
            .push(Image::new(image::Handle::from_path(path)).width(Length::Units(800)))
            .push(Text::new("\n\n\n0").size(22).height(Length::Shrink))
            .push(Text::new("Press down on the dial to continue").size(25))
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

    fn iced_input(&mut self, msg: Message) -> ScreenCommand {
        ScreenCommand::None
    }
}
