mod primitive;
mod events;
mod scene;
mod render_state;


use crate::message::Message;
use scene::Scene;
use iced::{widget::{button, container, shader, stack, text}, Alignment, Color, Element, Length, Renderer, Theme};
use pemel::prelude::Expr;

pub struct Plotter {
    scene: Scene,
    status: Status,
}

impl Default for Plotter {
    fn default() -> Self {
        Plotter {
            scene: Scene::default(),
            status: Status::default(),
        }
    }
}

impl Plotter {
    pub fn new() -> Plotter {
        Plotter::default()
    }

    pub fn add_element(&mut self, input: &str) {
        // if the input is empty, we dont want to render anything
        if input.is_empty() {
            self.scene.elements.push(None);
            return;
        }

        match Expr::parse(input, true) {
            Ok(func) => {
                self.scene.elements.push(Some(func));
                self.status = Status::all_good();
            },

            Err(err) => {
                let msg = format!("{:?}", err);
                self.status = Status::bad(msg);
            },
        }
    }

    pub fn remove_element(&mut self, index: usize) {
        self.scene.elements.remove(index);
    }

    pub fn update_view(&mut self, offset: glam::Vec2, zoom: f32) {
        self.scene.offset = offset;
        self.scene.zoom = zoom;
    }

    pub fn update_expr(&mut self, input: &str, index: usize) {
        if input.is_empty() {
            self.scene.elements[index] = None;
            return;
        }

        match Expr::parse(input, true) {
            Ok(func) => {
                self.scene.elements[index] = Some(func);
                self.status = Status::all_good();
            },

            Err(err) => {
                let msg = format!("{:?}", err);
                self.status = Status::bad(msg);
            },
        }
    }

    pub fn with_size<T, U>(&self, width: T, height: U) -> Element<'_, Message, Theme, Renderer>
    where
        T: Into<iced::Length>,
        U: Into<iced::Length>
    {
        stack!(
            shader(&self.scene)
                .width(width)
                .height(height),

            home_button(),
            status_bar(&self.status),
        )
        .into()
    }
}



fn home_button<'a>() -> Element<'a, Message, Theme, Renderer> {
    container(
        button("Home")
            .on_press(Message::UpdateView(glam::Vec2::ZERO, 1.0))
    )
    .padding(10.0)
    .align_x(Alignment::End)
    .align_y(Alignment::Start)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn status_bar<'a>(status: &'a Status) -> Element<'a, Message, Theme, Renderer> {
    container(
        text(&status.msg)
            .color(status.color)
    )
    .padding(5.0)
    .align_x(Alignment::Start)
    .align_y(Alignment::Start)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}


struct Status {
    pub msg: String,
    pub color: Color,
}

impl Default for Status {
    fn default() -> Self {
        Status::all_good()
    }
}

impl Status {
    pub fn all_good() -> Self {
        Self {
            msg: "Everything's good!".to_string(),
            color: Color::from_rgb8(0, 255, 0)
        }
    }

    pub fn bad(msg: String) -> Self {
        Self {
            msg: msg,
            color: Color::from_rgb8(255, 0, 0),
        }
    }
}