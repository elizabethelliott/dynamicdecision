use std::{time::Duration};
use url::Url;

use surface_dial_rs::{SurfaceDial, events::TopLevelEvent, events::DialEvent, events::DialDirection};

extern crate iced;
extern crate surface_dial_rs;

use iced::{Settings, Column, Element, Text, Application, executor, Command, window, Subscription, time, Length, Alignment};

mod views; 
pub mod arc_input;

use crate::views::arc_input_video_view::ArcInputVideoView;
use crate::views::info_view::InfoView;
use crate::views::ScreenCommand;

struct DynBaseProgram<'a> {
    dial: SurfaceDial<'a>,
    current_screen: usize,
    screens: Vec<Box<dyn views::DialView>>
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ProcessDialEvents,
}

impl Application for DynBaseProgram<'_> {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut dial = SurfaceDial::new();

        dial.set_subdivisions(60);

        let mut screens: Vec<Box<dyn views::DialView>> = vec![
            Box::new(InfoView::new("Test".to_string(), "This is another test.".to_string())),
            Box::new(ArcInputVideoView::new()),
            Box::new(InfoView::new("All done".to_string(), "Thank you for watching a video. You win.\n\nHooray!".to_string())),
        ];

        for s in screens.iter_mut() {
            s.init();
        }

        (DynBaseProgram {
            dial,
            current_screen: 0,
            screens
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Dynamic Base Decisions")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ProcessDialEvents => {
                let result = self.dial.pop_event();
                let command = self.screens[self.current_screen].update(result);

                match command {
                    ScreenCommand::NextScreen => {
                        if self.current_screen + 1 < self.screens.len() {
                            self.screens[self.current_screen].hide();

                            self.current_screen += 1;

                            self.screens[self.current_screen].init();
                            self.screens[self.current_screen].show();
                        }
                    },
                    ScreenCommand::PreviousScreen => {
                        if self.current_screen > 0 {
                            self.screens[self.current_screen].hide();
                            
                            self.current_screen -= 1;

                            self.screens[self.current_screen].init();
                            self.screens[self.current_screen].show();
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
        self.screens[self.current_screen].view()
    }

    fn mode(&self) -> window::Mode {
        window::Mode::Fullscreen
    }

    fn scale_factor(&self) -> f64 {
        1.5
    }
}

pub fn main() -> iced::Result {
    DynBaseProgram::run(Settings::default())
}
