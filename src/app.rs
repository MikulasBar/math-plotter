use iced::{
    self, widget::{column, container, row, text_input, Column}, Length, Task
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

const AMOUNT: usize = 4;

struct App {
    plotter: Plotter,
    inputs: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            plotter: Plotter::new(),
            inputs: vec!["".to_string(); AMOUNT],
        }
    }
}

fn update(app: &mut App, message: Message) -> impl Into<Task<Message>> {
    match message {
        Message::UpdateView(offset, zoom) => {
            app.plotter.update_view(offset, zoom);
        },

        Message::UpdateInput(input, index) => {
            app.inputs[index] = input;
        },

        Message::UpdateExpr(index) => {
            // app.plotter.update_expr(&app.input);
        },
    }
    
    Task::none()
}

fn view(app: &App) -> iced::Element<Message> {
    const WIDTH: Length = Length::Fill;
    const HEIGHT: Length = Length::Fill;

    let input_column = app.inputs.iter()
        .enumerate()
        .fold(Column::new(), |column, (i, input)| {
            column.push(
                text_input(input, input)
                    .on_input(move |input| Message::UpdateInput(input, i))
                    .on_submit(Message::UpdateExpr(i))
                    .width(Length::Fill)
            )
        });

    row![
        input_column
            .spacing(10)
            .padding(50.0),

        container(
            app.plotter.with_size(WIDTH, HEIGHT)
        )
        .align_right(Length::Fill)
    ]
    .into()
}