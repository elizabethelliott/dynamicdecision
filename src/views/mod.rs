use iced::Element;

use surface_dial_rs::events::TopLevelEvent;

use crate::Message;

pub enum ScreenCommand {
    None,
    NextScreen,
    PreviousScreen
}

pub trait DialView {
    fn init(&mut self);
    fn update(&mut self, msg: Option<TopLevelEvent>) -> ScreenCommand;
    fn view(&mut self) -> Element<Message>;
    fn show(&mut self);
    fn hide(&mut self);
}

pub mod arc_input_video_view;
pub mod info_view;