use std::collections::HashMap;

use iced::Element;

use surface_dial_rs::events::TopLevelEvent;

use crate::Message;


pub trait ConfigurationPayload {

}

pub trait Printable {
    fn to_csv(&self) -> String;
}
pub enum ScreenCommand {
    None,
    NextScreen(Option<HashMap<String, String>>),
    PreviousScreen
}

pub struct ArcSettings {
    pub divisions: u16
}

pub trait ExperimentData {
    fn name(&self) -> String;
    fn headers(&self) -> String;
    fn data(&self) -> Box<&dyn Printable>;
}

pub trait DialView {
    fn init(&mut self);
    fn update(&mut self, msg: Option<TopLevelEvent>) -> ScreenCommand;
    fn iced_input(&mut self, msg: Message) -> ScreenCommand;
    fn view(&self) -> Element<Message>;
    fn show(&mut self);
    fn hide(&mut self);
    fn data(&self) -> Option<Box<&dyn ExperimentData>>;
    fn arc_settings(&self) -> Option<ArcSettings>;
}

pub mod arc_input_video_view;
pub mod arc_dichotomous_view;
pub mod participant_id_view;
pub mod info_view;
pub mod multichoice_view;
pub mod textinput_view;
pub mod image_view;
pub mod video_view;
pub mod lock_in_video_view;
pub mod arc_question_scale;