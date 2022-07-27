use iced::alignment::Horizontal;
use iced::canvas::path::arc::Elliptical;
use iced::canvas::{
    self, Cache, Canvas, Cursor, Geometry, LineCap, Stroke,
};
use iced::{Element, Vector, Column, Row, Text};
use iced::canvas::path::{Builder};
use iced_native::{Color, Length, Point, Rectangle};

use crate::Message;

pub struct ArcInput {
    value: i32,
    min_value: i32,
    max_value: i32,
    left_label: String,
    right_label: String,
    radius: f32,
    arc: Cache,
    disabled: bool,
}

impl ArcInput {
    pub fn new(min: i32, max: i32, initial: i32, radius: f32) -> ArcInput {
        ArcInput {
            value: initial,
            min_value: min,
            max_value: max,
            left_label: "".to_string(),
            right_label: "".to_string(),
            radius,
            arc: Cache::default(),
            disabled: false
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let rad = self.radius * 1.5;
        let left_label = self.left_label.clone();
        let right_label = self.right_label.clone();

        Column::new()
            .push(Canvas::new(self)
                .width(Length::Units(rad as u16))
                .height(Length::Units(rad as u16)))
            .push(Row::new()
                .width(Length::Units(rad as u16))
                .push(Text::new(left_label).size(10).width(Length::Fill))
                .push(Text::new(right_label).size(10).horizontal_alignment(Horizontal::Right).width(Length::Fill)))
            .into()
        
    }

    pub fn set_value(&mut self, new_value: i32) {
        self.value = new_value;
        self.request_redraw();
    }

    pub fn set_left_label(&mut self, new_label: String) {
        self.left_label = new_label;
    }

    pub fn set_right_label(&mut self, new_label: String) {
        self.right_label = new_label;
    }

    pub fn request_redraw(&mut self) {
        self.arc.clear();
    }

    pub fn set_disabled(&mut self, disable: bool) {
        self.disabled = disable;
        self.request_redraw();
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
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
            } else if self.value < self.min_value {
                self.min_value
            } else {
                self.value
            };

            let proportion = (((safe_value as f32 + self.max_value as f32) / (self.max_value - self.min_value) as f32) * 2.0) - 1.0;

            fill_build.ellipse(Elliptical {
                center: Point::new(bounds.width/2.0, self.radius as f32),
                radii: Vector::new(self.radius/2.0, self.radius/2.0),
                rotation: 1.57,
                start_angle: 3.141,
                end_angle: 3.141 + (2.356 * proportion),
            });

            let arc_path = arc_build.build();
            let fill_path = fill_build.build();

            let arc_stroke = Stroke {
                color: Color::from_rgb(0.8, 0.8, 0.8),
                width: 2.0,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            // let fill_color = if self.value == 0 {
            //     Color::from_rgb(0.0, 0.0, 0.8)
            // } else if self.value > 0 {
            //     Color::from_rgb(0.0, 0.8, 0.0)
            // } else {
            //     Color::from_rgb(0.8, 0.0, 0.0)
            // };

            let fill_color = if self.disabled {
                Color::from_rgb(0.5, 0.5, 0.8)
            } else {
                Color::from_rgb(0.0, 0.0, 0.8)
            };

            let fill_stroke = Stroke {
                color: fill_color,
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