use std::time::Duration;
use url::Url;

use surface_dial_rs::{SurfaceDial, events::TopLevelEvent, events::DialEvent, events::DialDirection};

extern crate iced;
extern crate surface_dial_rs;

use iced_video_player::VideoPlayer;
use iced::{Settings, Column, Element, Text, Application, executor, Command, window, Subscription, time, Length, Alignment};

//mod arc_input;

struct Counter<'a> {
    dial: SurfaceDial<'a>,
    arc_input: arc_input::ArcInput,
    value: i32,
    video: VideoPlayer,
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

        let arc_input = arc_input::ArcInput::new(20, 0, 90.0);

        (Counter {
            dial,
            arc_input,
            value: 0,
            video: VideoPlayer::new(&Url::from_file_path(std::path::PathBuf::from(file!()).parent().unwrap().join("../videos/bad-guy.mp4").canonicalize().unwrap()).unwrap(), false).unwrap(),
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
                                    self.value += 1
                                },
                                DialDirection::Counterclockwise => {
                                    self.value -= 1
                                }
                            }
                            let safe_value: u32 = 
                                if self.value < 0 {
                                    0
                                } else {
                                    self.value as u32
                                };

                            self.arc_input.set_value(safe_value);
                        }

                        if let TopLevelEvent::DialEvent(DialEvent::Button { pressed }) = &e {
                            if *pressed {
                                self.value = 0;
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
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .align_items(Alignment::Center)
            .push(self.video.frame_view())
            .push(Text::new(self.value.to_string()).size(50))
            .push(self.arc_input.view())
            .into()
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

mod arc_input {
    use iced::canvas::path::arc::Elliptical;
    use iced::canvas::{
        self, Cache, Canvas, Cursor, Geometry, LineCap, Stroke,
    };
    use iced::{Element, Size, Vector};
    use iced::canvas::path::{Arc, Builder};
    use iced_native::{Color, Length, Point, Rectangle, Widget, renderer};

    use crate::Message;
    
    pub enum ArcMessage {
        UpdateValue(u32)
    }
    
    pub struct ArcInput {
        value: u32,
        max_value: u32,
        radius: f32,
        arc: Cache,
    }
    
    impl ArcInput {
        pub fn new(max: u32, initial: u32, radius: f32) -> ArcInput {
            ArcInput {
                value: initial,
                max_value: max,
                radius,
                arc: Cache::default()
            }
        }
    
        pub fn view(&mut self) -> Element<Message> {
            Canvas::new(self)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }

        pub fn set_value(&mut self, new_value: u32) {
            self.value = new_value;
            self.request_redraw();
        }
    
        pub fn request_redraw(&mut self) {
            self.arc.clear()
        }
    }
    
    impl canvas::Program<Message> for ArcInput {
        fn draw(
            &self,
            bounds: Rectangle,
            _cursor: Cursor,
        ) -> Vec<Geometry> {
            let arc = self.arc.draw(bounds.size(), |frame| {
                let mut arc_build = Builder::new();
                let mut fill_build = Builder::new();
    
                arc_build.ellipse(Elliptical {
                    center: Point::new(bounds.width/2.0, self.radius as f32),
                    radii: Vector::new(self.radius/2.0, self.radius/2.0),
                    rotation: 1.57,
                    start_angle: 0.785,
                    end_angle: 5.497,
                });

                let safe_value = if self.value > self.max_value {
                    self.max_value
                } else {
                    self.value
                };

                let proportion = safe_value as f32 / self.max_value as f32;

                fill_build.ellipse(Elliptical {
                    center: Point::new(bounds.width/2.0, self.radius as f32),
                    radii: Vector::new(self.radius/2.0, self.radius/2.0),
                    rotation: 1.57,
                    start_angle: 0.785,
                    end_angle: 0.785 + (4.712 * proportion),
                });
    
                let arc_path = arc_build.build();
                let fill_path = fill_build.build();
    
                let arc_stroke = Stroke {
                    color: Color::from_rgb(0.8, 0.8, 0.8),
                    width: 2.0,
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                };

                let fill_stroke = Stroke {
                    color: Color::from_rgb(0.0, 0.0, 0.8),
                    width: 2.0,
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                };
    
                frame.with_save(|frame| {
                    //frame.fill_rectangle(Point::new(0.0, 0.0), Size::new(frame.width(), frame.height()), Color::BLACK);
                    frame.stroke(&arc_path, arc_stroke);
                    frame.stroke(&fill_path, fill_stroke);
                });
            });
    
            vec![arc]
        }
    }
}
