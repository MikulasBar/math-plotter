use iced::{
    self,
    widget::container,
    Length, Task
};

use crate::{
    message::Message,
    plotter::Plotter,
};


pub fn run() -> iced::Result {
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
            println!("Key pressed: {key:?}");
        },

        Message::UpdateView(offset, zoom) => {
            app.plotter.update_view(offset, zoom);
        }
    }
    
    Task::none()
}

fn view(app: &App) -> iced::Element<Message> {
    const WIDTH: Length = Length::Fixed(300.0);
    const HEIGHT: Length = Length::Fill;
    
    container(
        app.plotter.with_size(WIDTH, HEIGHT)
    )
    .align_right(Length::Fill)
    .into()
}