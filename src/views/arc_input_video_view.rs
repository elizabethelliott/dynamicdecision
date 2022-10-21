use std::borrow::BorrowMut;
use std::env::current_exe;
use std::time::Instant;

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

use super::ExperimentData;
use super::Printable;

const MIN_VALUE: i32 = -10;
const MAX_VALUE: i32 = 10;

struct DataStructure {
    id: usize,
    path: String,
    counterbalance: bool,
    final_decision: i32,
    final_decision_timestamp: u128,
    data_points: Vec<DataPoint>
}

struct DataPoint {
    timestamp: u128,
    value: i32
}

impl ExperimentData for DataStructure {
    fn name(&self) -> String {
        format!("lie_truth_dynamic_{}", self.id).to_string()
    }

    fn headers(&self) -> String {
        "type,timestamp,value".to_string()
    }

    fn data(&self) -> Box<&dyn Printable> {
        Box::new(self)
    }
}

impl Printable for DataStructure {
    fn to_csv(&self) -> String {
        let mut final_string: String = "".to_string();
        let multiplier = if self.counterbalance {
            -1
        } else {
            1
        };

        final_string.push_str(format!("counterbalance,0,{}\n", self.counterbalance).as_str());
        final_string.push_str(format!("path,0,{}\n", self.path).as_str());

        for point in self.data_points.iter() {
            final_string.push_str(format!("decision,{},{}\n", point.timestamp, point.value * multiplier).as_str());
        }
        final_string.push_str(format!("final,{},{}\n", self.final_decision_timestamp, self.final_decision * multiplier).as_str());
        
        final_string
    }
}

pub struct ArcInputVideoView {
    id: usize,
    path: String,
    arc_input: ArcInput,
    value: i32,
    min_value: i32,
    max_value: i32,
    interim_decision: i32,
    video: Option<VideoPlayer>,
    data: DataStructure,
    timer: Option<Instant>,
    finished: bool,
    allow_lockin: bool
}

impl DataStructure {
    pub fn new(id: usize, path: String, counterbalance: bool) -> DataStructure {
        DataStructure {
            id,
            counterbalance,
            path,
            final_decision: 0,
            final_decision_timestamp: 0,
            data_points: Vec::new() 
        }
    }
}

impl ArcInputVideoView {
    pub fn new(id: usize, path: String, counterbalance: bool, allow_lockin: bool) -> ArcInputVideoView {
        let mut arc_input = ArcInput::new(MIN_VALUE, MAX_VALUE, 0, 90.0);
        if counterbalance {
            arc_input.set_right_label("Lie".to_string());
            arc_input.set_left_label("Truth".to_string());
        } else {
            arc_input.set_left_label("Lie".to_string());
            arc_input.set_right_label("Truth".to_string());
        }
        arc_input.scale(1.4);

        ArcInputVideoView {
            id,
            path: path.clone(),
            arc_input,
            value: 0,
            min_value: MIN_VALUE,
            max_value: MAX_VALUE,
            interim_decision: 0,
            data: DataStructure::new(id, path.clone(), counterbalance),
            timer: None,
            finished: false,
            allow_lockin,
            video: None }
    }
}

impl DialView for ArcInputVideoView {
    fn init(&mut self) {
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
                            if self.allow_lockin {
                                self.data.final_decision = self.value;
                                self.data.final_decision_timestamp = self.video.as_mut().expect("No video is playing").position().as_millis();
                                self.timer = None;

                                self.arc_input.set_disabled(true);
                                self.video.as_mut().expect("No video is playing").set_paused(true);

                                self.finished = true;

                                //println!("The user made a decision! {}", self.data.final_decision);
                            }
                        } else {
                            return ScreenCommand::NextScreen(None);
                        }
                    }
                }

                if let TopLevelEvent::ConnectionEvent(c) = e {
                    println!("Connection event: {:?}", c);
                }
            },
            _ => {}
        }

        // Check to see if we've reached the end of the video (with some buffer)
        if self.video.as_ref().expect("No video is playing").position().as_millis() + 25 >= self.video.as_ref().expect("No video is playing").duration().as_millis() {
            self.data.final_decision = self.value;
            self.data.final_decision_timestamp = self.video.as_mut().expect("No video is playing").position().as_millis();
            self.timer = None;

            self.arc_input.set_disabled(true);
            self.video.as_mut().expect("No video is playing").set_paused(true);

            self.finished = true;
        }

        // Wait to record a point if the user doesn't move the dial for 500ms
        if let Some(timer) = self.timer {
            if timer.elapsed().as_millis() > 500 {
                self.data.data_points.push(DataPoint {
                    timestamp: self.video.as_mut().expect("No video is playing").position().as_millis(),
                    value: self.value,
                });
                self.timer = None;

                //println!("Stored a point! {}", self.value);
            }
        }

        ScreenCommand::None
    }

    fn view(&mut self) -> Element<Message> {
        let mut column = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40)
            .align_items(Alignment::Center)
            .push(Text::new("Is the person lying or telling the truth?").size(30));

            if let Some(v) = self.video.borrow_mut() { 
                column = column.push(v.frame_view().width(Length::Units(640)).height(Length::Units(360)));
            } else { 
                column = column.push(Text::new("Video is loading"));
            }

            column = column.push(self.arc_input.view())
                .push(Text::new("\n\n0").size(22).height(Length::Shrink))
                .push(Text::new("Your last decision will be made final at the end of the video").size(16))
                .push(Text::new("\n\n\n0").size(22).height(Length::Shrink))
                .push(Text::new(if self.finished { "Press down on the dial to continue" } else { "" }).size(25));
            
            column.into()
    }

    fn show(&mut self) {
        let path = std::path::PathBuf::from(current_exe().unwrap());
        let root_path = path.parent().unwrap().parent().unwrap().parent().unwrap();
        let uri = Url::from_file_path(root_path.join(self.path.clone()).canonicalize().unwrap()).unwrap();

        self.video = Some(VideoPlayer::new(&uri, false).unwrap(),);
        self.video.as_mut().expect("No video is loaded").set_paused(false);
        self.value = 0;
        self.arc_input.set_value(0);
        self.data.data_points.clear();
        self.data.final_decision = 0;
    }

    fn hide(&mut self) {
        if let Some(v) = self.video.as_mut() {
            v.set_paused(true);
        }
        self.video = None;
    }

    fn data(&self) -> Option<Box<&dyn super::ExperimentData>> {
        Some(Box::new(&self.data))
    }

    fn arc_settings(&self) -> Option<super::ArcSettings> {
        Some(super::ArcSettings {
            divisions: 60
        })
    }

    fn iced_input(&mut self, msg: Message) -> ScreenCommand {
        ScreenCommand::None
    }
}