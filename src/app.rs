use iced::{
    self,
    widget::container,
    Application, Length, application::{Update, View}, Task
};

use crate::{
    message::Message,
    plotter::Plotter,
};


pub fn run_default() -> iced::Result {
    iced::application("Plotter app", update, view)
        .resizable(true)
        .run()
}

struct App {
    plotter: Plotter,
}

impl Default for App {
    fn default() -> Self {
        App {
            plotter: Plotter::new()
        }
    }
}

fn update(app: &mut App, message: Message) -> impl Into<Task<Message>> {
    match message {
        Message::KeyPressed(key) => {
            // self.plotter.update_view(glam::Vec2 { x: 0.0, y: 1.0 });
            println!("Key pressed: {:?}", key);
        },

        Message::UpdateView(offset) => {
            app.plotter.update_view(offset);
        }
    }
    ()
}

fn view(app: &App) -> iced::Element<Message> {
    container(
        app.plotter.get_widget()
            .height(500)
            .width(500)
    )
    .center(Length::Fill)
    .into()
}