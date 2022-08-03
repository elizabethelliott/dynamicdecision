use std::borrow::Borrow;
use std::time::Duration;

use surface_dial_rs::SurfaceDial;

extern crate iced;
extern crate surface_dial_rs;

use iced::{Settings, Element, Application, executor, Command, window, Subscription, time};
use views::participant_id_view::ParticipantIdView;

mod views; 
mod data;
pub mod arc_input;

use crate::views::arc_dichotomous_view::ArcDichotomousView;
use crate::views::arc_input_video_view::ArcInputVideoView;
use crate::views::info_view::InfoView;
use crate::views::ScreenCommand;

use crate::data::write_data_file;

struct DynBaseProgram<'a> {
    dial: SurfaceDial<'a>,
    current_screen: usize,
    screens: Vec<Box<dyn views::DialView>>
}

#[derive(Debug, Clone)]
pub enum Message {
    ProcessDialEvents,
    TextInputChanged(String),
    ButtonPressed,
}

impl DynBaseProgram<'_> {
    fn update_dial_settings(&mut self, settings: views::ArcSettings) {
        if settings.divisions > 0 {
            self.dial.set_subdivisions(settings.divisions);
        } else {
            self.dial.disable_subdivisions();
        }
    }
}

impl Application for DynBaseProgram<'_> {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut dial = SurfaceDial::new();

        dial.set_subdivisions(60);

        let mut screens: Vec<Box<dyn views::DialView>> = vec![
            Box::new(ParticipantIdView::new()),
            Box::new(InfoView::new("Test".to_string(), "This is another test.".to_string())),
            Box::new(ArcInputVideoView::new(0)),
            Box::new(ArcDichotomousView::new(0)),
            Box::new(ArcInputVideoView::new(1)),
            Box::new(ArcDichotomousView::new(1)),
            Box::new(ArcInputVideoView::new(2)),
            Box::new(ArcDichotomousView::new(2)),
            Box::new(ArcInputVideoView::new(3)),
            Box::new(ArcDichotomousView::new(3)),
            Box::new(InfoView::new("Finished".to_string(), "Thank you for participating.".to_string())),
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
        let mut command = ScreenCommand::None;

        match message {
            Message::ProcessDialEvents => {
                let result = self.dial.pop_event();
                command = self.screens[self.current_screen].update(result);
            },
            Message::TextInputChanged(s) => {
                command = self.screens[self.current_screen].iced_input(Message::TextInputChanged(s));
            },
            Message::ButtonPressed => {
                command = self.screens[self.current_screen].iced_input(Message::ButtonPressed);
            }
        }

        match command {
            ScreenCommand::NextScreen(p) => {
                if self.current_screen + 1 < self.screens.len() {
                    self.screens[self.current_screen].hide();

                    // If this screen has data to write, export it
                    if let Some(experiment_data) = self.screens[self.current_screen].data() {
                        write_data_file(0, experiment_data);
                    }

                    self.current_screen += 1;

                    self.screens[self.current_screen].init();
                    self.screens[self.current_screen].show();

                    if let Some(dial_settings) = self.screens[self.current_screen].arc_settings() {
                        self.update_dial_settings(dial_settings);
                    }
                }
            },
            ScreenCommand::PreviousScreen => {
                if self.current_screen > 0 {
                    self.screens[self.current_screen].hide();
                    
                    self.current_screen -= 1;

                    self.screens[self.current_screen].init();
                    self.screens[self.current_screen].show();

                    if let Some(dial_settings) = self.screens[self.current_screen].arc_settings() {
                        self.update_dial_settings(dial_settings);
                    }
                }
            },
            _ => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(1000 / 60 as u64))
                .map(|_instant| { Message::ProcessDialEvents } )
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
