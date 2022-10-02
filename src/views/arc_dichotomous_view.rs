use std::alloc::System;
use std::fmt::Debug;
use std::time::Instant;
use std::time::SystemTime;

use iced::Alignment;
use iced::Column;
use iced::Element;

use iced::Length;
use iced::Text;

use surface_dial_rs::events::{DialEvent, DialDirection, TopLevelEvent};

use crate::Message;
use crate::arc_input::ArcInput;

use crate::views::ScreenCommand;
use crate::views::DialView;

use super::ExperimentData;
use super::Printable;

const MIN_VALUE: i32 = -1;
const MAX_VALUE: i32 = 1;

struct DataStructure {
    id: usize,
    control: bool,
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
        format!("lie_truth_dichotomous_{}", self.id).to_string()
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
        let multiplier = if self.control {
            -1
        } else {
            1
        };

        for point in self.data_points.iter() {
            final_string.push_str(format!("decision,{},{}\n", point.timestamp, point.value * multiplier).as_str());
        }
        final_string.push_str(format!("final,{},{}\n", self.final_decision_timestamp, self.final_decision).as_str());
        
        final_string
    }
}

pub struct ArcDichotomousView {
    id: usize,
    control: bool,
    arc_input: ArcInput,
    value: i32,
    min_value: i32,
    max_value: i32,
    interim_decision: i32,
    data: DataStructure,
    timer: Option<Instant>,
    finished: bool,
    show_time: SystemTime
}

impl DataStructure {
    pub fn new(id: usize, control: bool) -> DataStructure {
        DataStructure { 
            id,
            control,
            final_decision: 0,
            final_decision_timestamp: 0,
            data_points: Vec::new() 
        }
    }
}

impl ArcDichotomousView {
    pub fn new(id: usize, control: bool) -> ArcDichotomousView {
        let mut arc_input = ArcInput::new(MIN_VALUE, MAX_VALUE, 0, 90.0);
        if control {
            arc_input.set_right_label("Lie".to_string());
            arc_input.set_left_label("Truth".to_string());
        } else {
            arc_input.set_left_label("Lie".to_string());
            arc_input.set_right_label("Truth".to_string());
        }
        
        arc_input.scale(2.0);

        ArcDichotomousView {
            id,
            control,
            arc_input,
            value: 0,
            min_value: MIN_VALUE,
            max_value: MAX_VALUE,
            interim_decision: 0,
            data: DataStructure::new(id, control),
            timer: None,
            finished: false,
            show_time: SystemTime::now()
        }
    }
}

impl DialView for ArcDichotomousView {
    fn init(&mut self) {
        self.value = 0;
        self.arc_input.set_value(0);
        self.data.data_points.clear();
        self.data.final_decision = 0;
        self.data.final_decision_timestamp = 0;
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
                        if !self.finished && self.value != 0 {
                            let end_timestamp = SystemTime::now().duration_since(self.show_time).expect("Could not get timestamp for final decision");
                            
                            self.data.final_decision = self.value;
                            self.data.final_decision_timestamp = end_timestamp.as_millis();
                            self.timer = None;

                            self.arc_input.set_disabled(true);

                            self.finished = true;

                            //println!("The user made a decision! {}", self.data.final_decision);
                        } else if self.finished {
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

        // Wait to record a point if the user doesn't move the dial for 500ms
        if let Some(timer) = self.timer {
            if timer.elapsed().as_millis() > 500 {
                let elapsed_time = SystemTime::now().duration_since(self.show_time).expect("Could not get timestamp for data point");
                self.data.data_points.push(DataPoint {
                    timestamp: elapsed_time.as_millis(),
                    value: self.value,
                });
                self.timer = None;

                //println!("Stored a point! {}", self.value);
            }
        }

        ScreenCommand::None
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40)
            .align_items(Alignment::Center)
            .push(Text::new("Was the person lying or telling the truth?").size(30))
            .push(self.arc_input.view())
            .push(Text::new(if self.finished { "Press down on the dial to continue" } else { "" }).size(25))
            .into()
    }

    fn show(&mut self) {
        self.show_time = SystemTime::now();
        self.value = 0;
        self.arc_input.set_value(0);
        self.data.data_points.clear();
        self.data.final_decision = 0;
        self.data.final_decision_timestamp = 0;
    }

    fn hide(&mut self) {
        
    }

    fn data(&self) -> Option<Box<&dyn super::ExperimentData>> {
        Some(Box::new(&self.data))
    }

    fn arc_settings(&self) -> Option<super::ArcSettings> {
        Some(super::ArcSettings {
            divisions: 10
        })
    }

    fn iced_input(&mut self, msg: Message) -> ScreenCommand {
        ScreenCommand::None
    }
}