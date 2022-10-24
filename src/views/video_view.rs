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

use surface_dial_rs::events::{DialEvent, TopLevelEvent};

use crate::Message;
use crate::arc_input::ArcInput;

use crate::views::ScreenCommand;
use crate::views::DialView;

pub struct VideoView {
    path: String,
    video: Option<VideoPlayer>,
    finished: bool,
}

impl VideoView {
    pub fn new(path: String) -> VideoView {
        VideoView {
            path: path.clone(),
            finished: false,
            video: None }
    }
}

impl DialView for VideoView {
    fn init(&mut self) {

    }

    fn update(&mut self, msg: Option<TopLevelEvent>) -> ScreenCommand {
        match msg {
            Some(e) => {

                if let TopLevelEvent::DialEvent(DialEvent::Button { pressed }) = &e {
                    if *pressed {
                        if self.finished {
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

    fn view(&mut self) -> Element<Message> {
        let mut column = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40)
            .align_items(Alignment::Center)
            .push(Text::new("Please watch this video").size(30));

            if let Some(v) = self.video.borrow_mut() { 
                column = column.push(v.frame_view().width(Length::Units(640)).height(Length::Units(360)));
            } else { 
                column = column.push(Text::new("Video is loading"));
            }

            column = column.push(Text::new(if self.finished { "Press down on the dial to continue" } else { "" }).size(18));
            
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
        None
    }

    fn arc_settings(&self) -> Option<super::ArcSettings> {
        None
    }

    fn iced_input(&mut self, msg: Message) -> ScreenCommand {
        ScreenCommand::None
    }
}