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
            Box::new(InfoView::new("Instructions".to_string(), "In this study, you will watch video clips of different people providing an alibi, and answering
questions provided by an experimenter. Some people will be lying, whereas others will be telling
the truth. The clips will be randomly presented, so that each adult has a 50-50 likelihood of
telling the truth or lying. The segments are also independent. This means that if the person in
Clip 1 is telling the truth, there is still a 50-50 chance that the person in Clip 2 is telling the truth
or lying, and so on.\n\n
As you watch the video, please decide, as quickly and accurately as possible, whether the
person in the video was lying or telling the truth and use the dial to render your decision by
“locking in” your answer as demonstrated in the tutorial.".to_string())),
            Box::new(ArcInputVideoView::new(0)),
            Box::new(ArcDichotomousView::new(0)),
            Box::new(ArcInputVideoView::new(1)),
            Box::new(ArcDichotomousView::new(1)),
            Box::new(ArcInputVideoView::new(2)),
            Box::new(ArcDichotomousView::new(2)),
            Box::new(ArcInputVideoView::new(3)),
            Box::new(ArcDichotomousView::new(3)),
            Box::new(InfoView::new("Demographics".to_string(), "Demographics will go here".to_string())),
            Box::new(InfoView::new("Debriefing".to_string(), "As you read in the consent form, the goal of this study is to learn how people make decisions
about deception. We are trying to find out whether the types of decision-making tool (i.e., the
dial) that people use and the instructions that people receive when making lie detection decisions
might affect judgments of deception. For example, if someone uses continuous dial judgments to
make their decision, are they more likely to accurately judge if someone is a lie-teller than if they
made a single dial choice after they viewed an account? Currently, the impact of the dial as a
decision-making tool and instructions on lie detection are unknown.\n
In this study, you watched videos of people who were being interviewed and provided an alibi
for their whereabouts. We randomly chose which people would be encouraged to be honest, or
lie, to the interviewer. As a result, people’s actions were primarily due to our experimental
design and were not reflective of their natural behaviors or personal characteristics.\n
Thank you so much for taking part in our study! I want to let you know that it is very important
that you do not talk to anyone else about this study who hasn’t participated yet. If people know
what we’re studying before they arrive, they might change their behavior, and then we wouldn’t
be able to successfully run the study. So, it is very important that you do not discuss this study
with others.\n
If you wish to learn more about the study or the aggregate results, please feel free to contact the
Principal Investigator, Elizabeth Elliott, at elliotte@iastate.edu\n
Thank you again for participating!".to_string())),
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
                } else if self.current_screen + 1 >= self.screens.len() {
                    std::process::exit(0);
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
