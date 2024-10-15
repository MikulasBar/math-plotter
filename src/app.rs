use iced::{
    self,
    widget::container,
    Length, Task
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
            println!("Key pressed: {key:?}");
        },

        Message::UpdateView(offset, zoom) => {
            println!("Offset: {offset}, Zoom: {zoom}");
            app.plotter.update_view(offset, zoom);
        }
    }
    
    Task::none()
}

fn view(app: &App) -> iced::Element<Message> {
    let size = 800.0;
    
    container(
        app.plotter.get_widget()
            .height(size)
            .width(size)
    )
    .center(Length::Fill)
    .into()
}