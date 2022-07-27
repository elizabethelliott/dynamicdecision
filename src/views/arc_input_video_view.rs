use std::time::Instant;
use std::time::SystemTime;

use iced::Alignment;
use iced::Column;
use iced::Element;

use iced::Length;
use iced::Text;
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
    final_decision: i32,
    data_points: Vec<DataPoint>
}

struct DataPoint {
    timestamp: SystemTime,
    value: i32
}

pub struct ArcInputVideoView {
    arc_input: ArcInput,
    value: i32,
    min_value: i32,
    max_value: i32,
    interim_decision: i32,
    video: VideoPlayer,
    data: DataStructure,
    timer: Option<Instant>,
    finished: bool
}

impl DataStructure {
    pub fn new() -> DataStructure {
        DataStructure { 
            final_decision: 0,
            data_points: Vec::new() 
        }
    }
}

impl ArcInputVideoView {
    pub fn new() -> ArcInputVideoView {
        let mut arc_input = ArcInput::new(MIN_VALUE, MAX_VALUE, 0, 90.0);
        arc_input.set_left_label("Lie".to_string());
        arc_input.set_right_label("Truth".to_string());

        ArcInputVideoView {
            arc_input,
            value: 0,
            min_value: MIN_VALUE,
            max_value: MAX_VALUE,
            interim_decision: 0,
            data: DataStructure::new(),
            timer: None,
            finished: false,
            video: VideoPlayer::new(&Url::from_file_path(std::path::PathBuf::from(file!()).parent().unwrap().join("../../videos/bad-guy.mp4").canonicalize().unwrap()).unwrap(), false).unwrap(), }
    }
}

impl DialView for ArcInputVideoView {
    fn init(&mut self) {
        self.video.set_paused(true);
        self.value = 0;
        self.arc_input.set_value(0);
        self.data.data_points.clear();
        self.data.final_decision = 0;
    }

    fn update(&mut self, msg: Option<TopLevelEvent>) -> ScreenCommand {
        match msg {
            Some(e) => {
                if let TopLevelEvent::DialEvent(DialEvent::Rotate { direction, velocity: _ }) = &e {
                    match direction {
                        DialDirection::Clockwise => {
                            if !self.arc_input.is_disabled() && self.value + 1 <= self.max_value { 
                                self.value += 1;
                            }
                        },
                        DialDirection::Counterclockwise => {
                            if !self.arc_input.is_disabled() && self.value > self.min_value { 
                                self.value -= 1;
                            }
                        }
                    }

                    if self.interim_decision != self.value {
                        self.interim_decision = self.value;
                        self.timer = Some(Instant::now());
                    }

                    self.arc_input.set_value(self.value);
                }

                if let TopLevelEvent::DialEvent(DialEvent::Button { pressed }) = &e {
                    if *pressed {
                        if !self.finished {
                            self.data.final_decision = self.value;
                            self.timer = None;

                            self.arc_input.set_disabled(true);
                            self.video.set_paused(true);

                            self.finished = true;

                            println!("The user made a decision! {}", self.data.final_decision);
                        } else {
                            return ScreenCommand::NextScreen;
                        }
                    }
                }

                if let TopLevelEvent::ConnectionEvent(c) = e {
                    println!("Connection event: {:?}", c);
                }
            },
            _ => {}
        }

        // Wait to record a point if the user doesn't move the dial for 500ms
        if let Some(timer) = self.timer {
            if timer.elapsed().as_millis() > 500 {
                self.data.data_points.push(DataPoint {
                    timestamp: SystemTime::now(),
                    value: self.value,
                });
                self.timer = None;

                println!("Stored a point! {}", self.value);
            }
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

    fn data(&self) -> Option<Box<dyn super::ExperimentData>> {
        None
    }
}