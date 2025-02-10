use iced::{
    self, widget::{container, row, text_input}, Length, Task
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
    input: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            plotter: Plotter::new(),
            input: String::new(),
        }
    }
}

fn update(app: &mut App, message: Message) -> impl Into<Task<Message>> {
    match message {
        Message::UpdateView(offset, zoom) => {
            app.plotter.update_view(offset, zoom);
        },

        Message::UpdateInput(input) => {
            app.input = input;
        },

        Message::UpdateExpr => {
            app.plotter.update_expr(&app.input);
        },
    }
    
    Task::none()
}

fn view(app: &App) -> iced::Element<Message> {
    const WIDTH: Length = Length::Fill;
    const HEIGHT: Length = Length::Fill;

    row![
        container(
            text_input("Enter expression", &app.input)
                .on_input(|input| Message::UpdateInput(input))
                .on_submit(Message::UpdateExpr)
                .width(Length::Fill)
        )
        .padding(50.0)
        .center(Length::Fill),

        container(
            app.plotter.with_size(WIDTH, HEIGHT)
        )
        .align_right(Length::Fill)
    ]
    .into()
}
