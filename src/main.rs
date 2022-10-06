use std::fs;
use std::time::Duration;
use rand::{thread_rng, seq::IteratorRandom};

use surface_dial_rs::SurfaceDial;

extern crate iced;
extern crate surface_dial_rs;
extern crate yaml_rust;

use native_dialog::{MessageDialog, MessageType};

use iced::{executor, time, window, Application, Command, Element, Settings, Subscription};
use iced::keyboard::{self, KeyCode};
use views::participant_id_view::ParticipantIdView;
use yaml_rust::{YamlLoader, Yaml};

pub mod arc_input;
mod data;
mod views;

use crate::views::arc_dichotomous_view::ArcDichotomousView;
use crate::views::arc_input_video_view::ArcInputVideoView;
use crate::views::info_view::InfoView;
use crate::views::multichoice_view::MultiChoiceView;
use crate::views::textinput_view::{TextInputType, TextInputView};
use crate::views::ScreenCommand;

use crate::data::write_data_file;
use crate::data::partipant_data::ParticipantData;

const VIDEO_NAMES: [&'static str; 2] = [
    "alibi1_control.webm", // Lie
    "alibi2_control.webm"  // Truth
];

enum AppState {
    Participant,
    Instructions,
    Videos,
    Demographics,
    Final
}

struct DynBaseProgram<'a> {
    scaling_override: f64,
    config: Yaml,
    valid_ids: Vec<u32>,
    num_vids: usize,
    app_state: AppState,
    dial: SurfaceDial<'a>,
    current_screen: usize,
    participant_data: Option<ParticipantData>,
    participant_screen: Box<dyn views::DialView>,
    instruction_screen: Box<dyn views::DialView>,
    screens: Vec<Box<dyn views::DialView>>,
    demographics_screens: Vec<Box<dyn views::DialView>>,
    final_screen: Box<dyn views::DialView>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ProcessDialEvents,
    TextInputChanged(String),
    ButtonPressed,
    RadioSelected(u32),
    KeyPressed(KeyCode),
    KeyReleased(KeyCode)
}

impl DynBaseProgram<'_> {
    fn update_dial_settings(&mut self, settings: Option<views::ArcSettings>) {
        if let Some(actual_settings) = settings {
            if actual_settings.divisions > 0 {
                self.dial.set_subdivisions(actual_settings.divisions);
            } else {
                self.dial.disable_subdivisions();
            }
        }
    }
}

impl Application for DynBaseProgram<'_> {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let yaml_string = fs::read_to_string("config.yaml").expect("Could not load config file");
        let yaml_docs = YamlLoader::load_from_str(yaml_string.as_str()).expect("Invalid YAML in config.yaml");

        let yaml_config = &yaml_docs[0];

        let scaling_override = if !yaml_config["config"]["scaling"].is_badvalue() {
            yaml_config["config"]["scaling"].as_f64().unwrap()
        } else {
            0.0
        };

        let valid_ids: Vec<u32> = if !yaml_config["videos"]["ids"].is_badvalue() {
            let yaml_ids = yaml_config["videos"]["ids"].as_vec().expect("Could not read video ids from config");
            let mut ids: Vec<u32> = Vec::new();

            for id in yaml_ids.iter() {
                ids.push(id.as_i64().expect("Could not read video id") as u32);
            }

            ids
        } else {
            Vec::new()
        };

        let num_vids: usize = if !yaml_config["videos"]["num"].is_badvalue() {
            yaml_config["videos"]["num"].as_i64().expect("Could not read valid number of videos from config") as usize
        } else {
            5
        };

        let mut dial = SurfaceDial::new();

        dial.set_subdivisions(60);

        let participant_screen: Box<dyn views::DialView> = Box::new(ParticipantIdView::new());

        let instruction_screen: Box<dyn views::DialView> = Box::new(InfoView::new("Instructions".to_string(), 
        "In this study, you will watch video clips of different people providing an alibi, and answering
questions provided by an experimenter. Some people will be lying, whereas others will be telling
the truth. The clips will be randomly presented, so that each adult has a 50-50 likelihood of
telling the truth or lying. The segments are also independent. This means that if the person in
Clip 1 is telling the truth, there is still a 50-50 chance that the person in Clip 2 is telling the truth
or lying, and so on.\n\n
As you watch the video, please decide, as quickly and accurately as possible, whether the
person in the video was lying or telling the truth and use the dial to render your decision by
“locking in” your answer as demonstrated in the tutorial.".to_string()));

        let mut screens: Vec<Box<dyn views::DialView>> = vec![];

        let demographics_screens: Vec<Box<dyn views::DialView>> = vec![
            Box::new(TextInputView::new(
                TextInputType::Number, 
                "demographics_age".to_string(), 
                "Demographics".to_string(), 
                "What is your current age?".to_string(),
                "Age...".to_string())
            ),
            Box::new(MultiChoiceView::new(
                "demographics_gender".to_string(), 
                "Demographics".to_string(), 
                "With which gender do you most identify (select one)?".to_string(), 
                vec![
                    (0, "Male".to_string()), 
                    (1, "Female".to_string()), 
                    (2, "Other".to_string()),
                    (3, "Prefer not to disclose".to_string()),
                ]
            )),
            Box::new(MultiChoiceView::new(
                "demographics_race".to_string(), 
                "Demographics".to_string(), 
                "Which of the following races/ethnicities best describes you (select one)?".to_string(), 
                vec![
                    (0, "Aboriginal or indigenous (i.e., Alaskan native, American Indian, First Nations, Inuit, Metis)".to_string()), 
                    (1, "Arab or West Asian (e.g., Armenian, Egyptian, Iranian, Lebanese, Moroccan)".to_string()), 
                    (2, "Black (e.g., African, Haitian, Jamaican, Somali)".to_string()),
                    (3, "Chinese".to_string()),
                    (4, "Filipino".to_string()),
                    (5, "Japanese".to_string()),
                    (6, "Korean".to_string()),
                    (7, "Latino/Hispanic".to_string()),
                    (8, "Pacific Islander".to_string()),
                    (9, "South Asian".to_string()),
                    (10, "South East Asian".to_string()),
                    (11, "White, non-Hispanic (i.e., Caucasian)".to_string()),
                    (12, "Multi-ethnic".to_string()),
                    (13, "Other".to_string()),
                    (14, "Prefer not to disclose".to_string()),
                ]
            )),
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
        ];

        let final_screen: Box<dyn views::DialView> = Box::new(InfoView::new("Finished".to_string(), "Thank you for participating.".to_string()));

        for s in screens.iter_mut() {
            s.init();
        }

        (
            DynBaseProgram {
                scaling_override,
                config: yaml_config.clone(),
                valid_ids,
                num_vids,
                app_state: AppState::Participant,
                dial,
                current_screen: 0,
                participant_data: None,
                participant_screen,
                instruction_screen,
                screens,
                demographics_screens,
                final_screen,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Dynamic Base Decisions")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let mut command = ScreenCommand::None;
        let screen = match self.app_state {
            AppState::Participant => &mut self.participant_screen,
            AppState::Instructions => &mut self.instruction_screen,
            AppState::Videos => &mut self.screens[self.current_screen],
            AppState::Demographics => &mut self.demographics_screens[self.current_screen],
            AppState::Final => &mut self.final_screen,
        };
        let dial = &self.dial;

        match message {
            Message::ProcessDialEvents => {
                let result = dial.pop_event();
                command = screen.update(result);
            }
            Message::TextInputChanged(s) => {
                command =
                    screen.iced_input(Message::TextInputChanged(s));
            }
            Message::ButtonPressed => {
                command = screen.iced_input(Message::ButtonPressed);
            },
            Message::RadioSelected(c) => {
                command = screen.iced_input(Message::RadioSelected(c));
            }
            Message::KeyPressed(k) => {
                if k == KeyCode::Right {
                    command = ScreenCommand::NextScreen(None);
                }
            },
            Message::KeyReleased(_) => {},
        }

        match command {
            ScreenCommand::NextScreen(c) => {
                match self.app_state {
                    AppState::Participant => {
                        // Get participant from list and store it as participant data
                        if let Some(config) = c {
                            if config.contains_key("id") {
                                let id = config["id"].parse::<usize>().unwrap();
                                println!("Got the ID: {}!", id);

                                let info = self.config["participants"][id].clone();

                                if info.is_badvalue() {
                                    // Tell the user that they selected an incorrect participant
                                    MessageDialog::new()
                                        .set_type(MessageType::Error)
                                        .set_title("Invalid Participant")
                                        .set_text(format!("The participant ID {} does not have an entry. Please select a different participant ID.", id).as_str())
                                        .show_alert()
                                        .unwrap();
                                    
                                    self.app_state = AppState::Participant;

                                    self.participant_screen.init();
                                    self.participant_screen.show();
                                } else {
                                    let control = info["control"].as_bool().expect(format!("Participant {} is missing the control parameter", id).as_str());

                                    // Store the participant info and move on to instructions
                                    self.participant_data = Some(ParticipantData { 
                                        id, 
                                        data: info 
                                    });

                                    let mut video_set = Vec::from(self.valid_ids.clone());
                                    
                                    // Create a new set of video screens
                                    for i in 0..self.num_vids {
                                        // Select a random video path
                                        let index = (0..video_set.len()).choose(&mut thread_rng()).unwrap();

                                        // Remove the path from the set so it cannot be picked again
                                        let vid_index = video_set.swap_remove(index);

                                        let lie_truth_ind: usize = (rand::random::<u32>() % 2) as usize;
                                        let vid_name = VIDEO_NAMES[lie_truth_ind].clone();

                                        let vid_path = format!("videos/{}/{}", vid_index, vid_name);

                                        self.screens.push(Box::new(ArcInputVideoView::new(i, vid_path, control)));
                                        self.screens.push(Box::new(ArcDichotomousView::new(i, control)));
                                    }

                                    self.participant_screen.hide();

                                    // Switch to the instructions
                                    self.app_state = AppState::Instructions;

                                    self.instruction_screen.init();
                                    self.instruction_screen.show();

                                    self.update_dial_settings(self.instruction_screen.arc_settings());
                                }
                            }
                        }
                    },
                    AppState::Instructions => {
                        self.instruction_screen.hide();

                        self.app_state = AppState::Videos;

                        self.screens[0].init();
                        self.screens[0].show();

                        self.update_dial_settings(self.screens[0].arc_settings());
                    },
                    AppState::Videos => {
                        self.screens[self.current_screen].hide();

                        // If this screen has data to write, export it
                        if let Some(experiment_data) = self.screens[self.current_screen].data() {
                            write_data_file(self.participant_data.as_ref().expect("Missing participant information").id, experiment_data);
                        }

                        if self.current_screen + 1 < self.screens.len() {                
                            self.current_screen += 1;
        
                            self.screens[self.current_screen].init();
                            self.screens[self.current_screen].show();
        
                            self.update_dial_settings(self.screens[self.current_screen].arc_settings());
                        } else if self.current_screen + 1 >= self.screens.len() {
                            self.current_screen = 0;
                            self.app_state = AppState::Demographics;

                            self.demographics_screens[0].init();
                            self.demographics_screens[0].show();

                            self.update_dial_settings(self.demographics_screens[0].arc_settings());
                        }
                    },
                    AppState::Demographics => {
                        if self.current_screen + 1 < self.demographics_screens.len() {
                            self.demographics_screens[self.current_screen].hide();
        
                            // If this screen has data to write, export it
                            if let Some(experiment_data) = self.demographics_screens[self.current_screen].data() {
                                write_data_file(self.participant_data.as_ref().expect("Missing participant information").id, experiment_data);
                            }
        
                            self.current_screen += 1;
        
                            self.demographics_screens[self.current_screen].init();
                            self.demographics_screens[self.current_screen].show();
        
                            self.update_dial_settings(self.demographics_screens[self.current_screen].arc_settings());
                        } else if self.current_screen + 1 >= self.demographics_screens.len() {
                            self.current_screen = 0;
                            self.app_state = AppState::Final;

                            self.final_screen.init();
                            self.final_screen.show();

                            self.update_dial_settings(self.final_screen.arc_settings());
                        }                        
                    },
                    AppState::Final => {
                        self.current_screen = 0;
                        self.screens.clear();
                        self.app_state = AppState::Participant;

                        self.participant_screen.init();
                        self.participant_screen.show();

                        self.update_dial_settings(self.participant_screen.arc_settings());
                    }
                }
            }
            ScreenCommand::PreviousScreen => {}
            _ => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        use iced_native::event::Event;

        Subscription::batch(vec![
            time::every(Duration::from_millis(1000 / 60 as u64))
                .map(|_instant| Message::ProcessDialEvents),
            iced_native::subscription::events_with(|event, _status| {
                match event {
                Event::Keyboard(e) => {
                    match e {
                    keyboard::Event::KeyPressed{key_code, modifiers: _} => {
                        Some(Message::KeyPressed(key_code))
                    },
                    keyboard::Event::KeyReleased{key_code, modifiers: _} => {
                        Some(Message::KeyReleased(key_code))
                    }
                    _ => None
                    }
                },
                _ => None,
                }
            })
        ])
    }

    fn view(&mut self) -> Element<Message> {
        match self.app_state {
            AppState::Participant => return self.participant_screen.view(),
            AppState::Instructions => return self.instruction_screen.view(),
            AppState::Videos => return self.screens[self.current_screen].view(),
            AppState::Demographics => return self.demographics_screens[self.current_screen].view(),
            AppState::Final => return self.final_screen.view(),
        }
    }

    fn mode(&self) -> window::Mode {
        window::Mode::Fullscreen
    }

    fn scale_factor(&self) -> f64 {
        if self.scaling_override > 0.0 {
            self.scaling_override
        } else {
            1.5
        }
    }
}

pub fn main() -> iced::Result {
    DynBaseProgram::run(Settings::default())
}
