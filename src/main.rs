use std::{time::Duration};
use url::Url;

use surface_dial_rs::{SurfaceDial, events::TopLevelEvent, events::DialEvent, events::DialDirection};

extern crate iced;
extern crate surface_dial_rs;

use iced::{Settings, Column, Element, Text, Application, executor, Command, window, Subscription, time, Length, Alignment};

mod dial_views;
pub mod arc_input;

struct Counter<'a> {
    dial: SurfaceDial<'a>,
    current_window: Box<dyn dial_views::DialView>
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ProcessDialEvents,
}

impl Application for Counter<'_> {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut dial = SurfaceDial::new();

        dial.set_subdivisions(60);

        let initial_window = Box::new(dial_views::ArcInputVideoView::new());

        (Counter {
            dial,
            current_window: initial_window
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Dynamic Base Decisions")
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
                                    if self.value + 1 <= self.max_value { self.value += 1 }
                                },
                                DialDirection::Counterclockwise => {
                                    if self.value > self.min_value { self.value -= 1 }
                                }
                            }

                            self.arc_input.set_value(self.value);
                        }

                        if let TopLevelEvent::DialEvent(DialEvent::Button { pressed }) = &e {
                            if *pressed {
                                self.value = 0;
                                self.arc_input.set_value(self.value);

                                if self.video.paused() {
                                    self.video.set_paused(false);
                                } else {
                                    self.video.set_paused(true);
                                }
                            }
                        }

                        if let TopLevelEvent::ConnectionEvent(c) = e {
                            println!("Connection event: {:?}", c);
                        }
                    },
                    _ => {}
                }
            },
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(1000 / 60 as u64))
                .map(|_instant| { Message::ProcessDialEvents } )
        //self.video.subscription().map(Message::VideoPlayerMessage)
    }

    fn view(&mut self) -> Element<Message> {
        self.current_window.view()
    }

    fn mode(&self) -> window::Mode {
        window::Mode::Fullscreen
    }

    fn scale_factor(&self) -> f64 {
        1.5
    }
}

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}
