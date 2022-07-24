use iced::Alignment;
use iced::Column;
use iced::Element;

use iced::Length;
use iced::Text;
use iced_video_player::VideoPlayer;
use url::Url;

use crate::Message;
use crate::arc_input;

const MIN_VALUE: i32 = -10;
const MAX_VALUE: i32 = 10;

pub enum ScreenCommand {
    None,
    NextScreen,
    PreviousScreen
}

pub trait DialView {
    fn init(&mut self);
    fn update(&mut self, msg: Message) -> ScreenCommand;
    fn view(&mut self) -> Element<Message>;
}

pub struct ArcInputVideoView {
    arc_input: arc_input::ArcInput,
    value: i32,
    min_value: i32,
    max_value: i32,
    video: VideoPlayer,
}

impl ArcInputVideoView {
    pub fn new() -> ArcInputVideoView {
        let mut arc_input = arc_input::ArcInput::new(MIN_VALUE, MAX_VALUE, 0, 90.0);
        arc_input.set_left_label("Left".to_string());
        arc_input.set_right_label("Right".to_string());

        ArcInputVideoView {
            arc_input,
            value: 0,
            min_value: MIN_VALUE,
            max_value: MAX_VALUE,
            video: VideoPlayer::new(&Url::from_file_path(std::path::PathBuf::from(file!()).parent().unwrap().join("../videos/bad-guy.mp4").canonicalize().unwrap()).unwrap(), false).unwrap(), }
    }
}

impl DialView for ArcInputVideoView {
    fn init(&mut self) {
        todo!()
    }

    fn update(&mut self, msg: Message) -> ScreenCommand {
        todo!()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .align_items(Alignment::Center)
            .push(Text::new("This is a test").size(30))
            .push(self.video.frame_view())
            .push(self.arc_input.view())
            .into()
    }
}

