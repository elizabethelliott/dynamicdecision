use std::time::Duration;

use iced::Alignment;
use iced::Column;
use iced::Element;

use iced::Length;
use iced::Text;
use iced_native::image::Data;
use iced_video_player::VideoPlayer;
use url::Url;

use surface_dial_rs::events::{DialEvent, DialDirection, TopLevelEvent};

use crate::Message;
use crate::arc_input::ArcInput;

use crate::views::ScreenCommand;
use crate::views::DialView;

const MIN_VALUE: i32 = -10;
const MAX_VALUE: i32 = 10;

struct DataStructure {
    final_decision: f32,
    data_points: Vec<DataPoint>
}

struct DataPoint {
    timestamp: Duration,
    value: f32
}

pub struct ArcInputVideoView {
    arc_input: ArcInput,
    value: i32,
    min_value: i32,
    max_value: i32,
    interim_decision: i32,
    last_decision: i32,
    video: VideoPlayer,
    data: DataStructure,
}

impl DataStructure {
    pub fn new() -> DataStructure {
        DataStructure { 
            final_decision: 0.0,
            data_points: Vec::new() 
        }
    }
}

impl ArcInputVideoView {
    pub fn new() -> ArcInputVideoView {
        let mut arc_input = ArcInput::new(MIN_VALUE, MAX_VALUE, 0, 90.0);
        arc_input.set_left_label("".to_string());
        arc_input.set_right_label("".to_string());

        ArcInputVideoView {
            arc_input,
            value: 0,
            min_value: MIN_VALUE,
            max_value: MAX_VALUE,
            interim_decision: 0,
            last_decision: 0,
            data: DataStructure::new(),
            video: VideoPlayer::new(&Url::from_file_path(std::path::PathBuf::from(file!()).parent().unwrap().join("../../videos/bad-guy.mp4").canonicalize().unwrap()).unwrap(), false).unwrap(), }
    }
}

impl DialView for ArcInputVideoView {
    fn init(&mut self) {
        self.video.set_paused(true);
        self.value = 0;
        self.arc_input.set_value(0);
    }

    fn update(&mut self, msg: Option<TopLevelEvent>) -> ScreenCommand {
        match msg {
            Some(e) => {
                if let TopLevelEvent::DialEvent(DialEvent::Rotate { direction, velocity: _ }) = &e {
                    match direction {
                        DialDirection::Clockwise => {
                            if self.value + 1 <= self.max_value { 
                                self.value += 1 
                            }
                        },
                        DialDirection::Counterclockwise => {
                            if self.value > self.min_value { 
                                self.value -= 1 
                            }
                        }
                    }

                    self.arc_input.set_value(self.value);
                }

                if let TopLevelEvent::DialEvent(DialEvent::Button { pressed }) = &e {
                    if *pressed {
                        return ScreenCommand::NextScreen;
                    }
                }

                if let TopLevelEvent::ConnectionEvent(c) = e {
                    println!("Connection event: {:?}", c);
                }
            },
            _ => {}
        }

        ScreenCommand::None
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

    fn show(&mut self) {
        self.video.set_paused(false);
    }

    fn hide(&mut self) {
        self.video.set_paused(true);
    }
}