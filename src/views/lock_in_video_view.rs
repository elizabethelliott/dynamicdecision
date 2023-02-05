use std::borrow::{Borrow, BorrowMut};
use std::env::current_exe;
use std::time::Instant;

use iced::Alignment;
use iced::widget::{Column, Text};
use iced::Element;

use iced::Length;
use iced_video_player::VideoPlayer;
use url::Url;

use surface_dial_rs::events::{DialEvent, TopLevelEvent};

use crate::Message;
use crate::arc_input::ArcInput;

use crate::views::{ExperimentData, Printable, ScreenCommand};
use crate::views::DialView;

struct DataStructure {
    id: usize,
    path: String,
    final_decision_timestamp: u128,
}

impl DataStructure {
    pub fn new(id: usize, path: String) -> DataStructure {
        DataStructure {
            id,
            path,
            final_decision_timestamp: 0,
        }
    }
}

impl ExperimentData for DataStructure {
    fn name(&self) -> String {
        format!("lie_truth_lock_in_{}", self.id).to_string()
    }

    fn headers(&self) -> String {
        "type,timestamp,value,velocity".to_string()
    }

    fn data(&self) -> Box<&dyn Printable> {
        Box::new(self)
    }
}

impl Printable for DataStructure {
    fn to_csv(&self) -> String {
        let mut final_string: String = "".to_string();

        final_string.push_str(format!("path,0,{},0.0\n", self.path).as_str());
        final_string.push_str(format!("final,{},0,0.0\n", self.final_decision_timestamp).as_str());

        final_string
    }
}

pub struct LockInVideoView {
    path: String,
    video: Option<VideoPlayer>,
    data: DataStructure,
    finished: bool,
}

impl LockInVideoView {
    pub fn new(id: usize, path: String) -> LockInVideoView {
        LockInVideoView {
            path: path.clone(),
            data: DataStructure::new(id, path),
            finished: false,
            video: None }
    }
}

impl DialView for LockInVideoView {
    fn init(&mut self) {

    }

    fn update(&mut self, msg: Option<TopLevelEvent>) -> ScreenCommand {
        match msg {
            Some(e) => {

                if let TopLevelEvent::DialEvent(DialEvent::Button { pressed }) = &e {
                    if *pressed {
                        if !self.finished {
                            self.data.final_decision_timestamp = self.video.as_mut().expect("No video is playing").position().as_millis();
                            self.video.as_mut().expect("No video is playing").set_paused(true);

                            self.finished = true;
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
            self.finished = true;
        }

        ScreenCommand::None
    }

    fn view(&self) -> Element<Message> {
        let mut column = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40)
            .align_items(Alignment::Center)
            .push(Text::new("Please watch this video").size(30));

        if let Some(v) = self.video.borrow() {
            column = column.push(v.frame_view().width(Length::Units(640)).height(Length::Units(360)));
        } else {
            column = column.push(Text::new("Video is loading"));
        }

        column = column.push(Text::new(if self.finished { "Now, press down on the dial to continue" } else { "Press down on the dial when you have made your decision" }).size(18));

        column.into()
    }

    fn show(&mut self) {
        let path = std::path::PathBuf::from(current_exe().unwrap());
        let root_path = path.parent().unwrap().parent().unwrap().parent().unwrap();
        let uri = Url::from_file_path(root_path.join(self.path.clone()).canonicalize().unwrap()).unwrap();

        self.video = Some(VideoPlayer::new(&uri, false).unwrap(),);
        self.video.as_mut().expect("No video is loaded").set_paused(false);
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
        None
    }

    fn iced_input(&mut self, msg: Message) -> ScreenCommand {
        ScreenCommand::None
    }
}