use std::time::Duration;
use url::Url;

use flume::Receiver;
use surface_dial_rs::{SurfaceDial, events::TopLevelEvent, events::DialEvent, events::DialDirection};

extern crate iced;
extern crate surface_dial_rs;

use iced_video_player::{VideoPlayerMessage, VideoPlayer};
use iced::{Column, Text, Element, Settings, Application, executor, Command, window, Subscription, time, Length, Alignment, Row};

struct Counter<'a> {
    dial: SurfaceDial<'a>,
    value: i32,
    video: VideoPlayer,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ProcessDialEvents,
    VideoPlayerMessage(VideoPlayerMessage)
}

impl Application for Counter<'_> {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut dial = SurfaceDial::new();

        dial.set_subdivisions(30);

        (Counter {
            dial,
            value: 0,
            video: VideoPlayer::new(&Url::from_file_path(std::path::PathBuf::from(file!()).parent().unwrap().join("../videos/bad-guy.mp4").canonicalize().unwrap()).unwrap(), false).unwrap(),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Dynamic Base Decisions")
    }

    fn mode(&self) -> window::Mode {
        window::Mode::Fullscreen
    }

    fn scale_factor(&self) -> f64 {
        1.5
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(1000 / 60 as u64))
                .map(|_instant| { Message::ProcessDialEvents } )
        //self.video.subscription().map(Message::VideoPlayerMessage)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ProcessDialEvents => {
                let result = self.dial.pop_event();

                match result {
                    Some(e) => {
                        if let TopLevelEvent::DialEvent(DialEvent::Rotate { direction, velocity: _ }) = &e {
                            match direction {
                                DialDirection::Clockwise => {
                                    self.value += 1
                                },
                                DialDirection::Counterclockwise => {
                                    self.value -= 1
                                }
                            }
                        }

                        if let TopLevelEvent::DialEvent(DialEvent::Button { pressed }) = &e {
                            if *pressed {
                                self.value = 0;
                            }
                        }

                        if let TopLevelEvent::ConnectionEvent(c) = e {
                            println!("Connection event: {:?}", c);
                        }
                    },
                    _ => {}
                }
            },
            Message::VideoPlayerMessage(vmsg) => { return self.video.update(vmsg).map(Message::VideoPlayerMessage) },
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .align_items(Alignment::Center)
            .push(self.video.frame_view())
            .push(Text::new(self.value.to_string()).size(50))
            .into()
    }
}

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}
