use iced::{
    self, widget::{self, button, scrollable, container, row, stack, text_input, Column}, Alignment, Length, Padding, Task
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
    inputs: Vec<String>,
}


impl Default for App {
    fn default() -> Self {
        let plotter = Plotter::new();

        App {
            plotter,
            inputs: vec![],
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
            app.plotter.update_expr(&app.inputs[index], index);
        },

        Message::AddInput => {
            app.plotter.add_element("");
            app.inputs.push("".to_string());
        },

        Message::RemoveInput(index) => {
            app.plotter.remove_element(index);
            app.inputs.remove(index);
        }
    }
    
    Task::none()
}

fn view(app: &App) -> iced::Element<Message> {
    row![
        input_frame(app),
        plot_frame(app)
    ]
    .into()
}



fn plot_frame<'a>(app: &'a App) -> iced::Element<'a, Message> {
    const WIDTH: Length = Length::FillPortion(1);
    const HEIGHT: Length = Length::Fill;

    app.plotter.with_size(WIDTH, HEIGHT)
        .into()
}

fn input_frame<'a>(app: &App) -> iced::Element<'a, Message> {
    let input_column = app.inputs.iter()
        .enumerate()
        .fold(Column::new(), |column, (i, input)| {
            column.push(
                row![
                    text_input(input, input)
                        .on_input(move |input| Message::UpdateInput(input, i))
                        .on_submit(Message::UpdateExpr(i))
                        .width(Length::Fill),

                    container(
                        button("X")
                            .on_press(Message::RemoveInput(i))
                    )
                    .padding(Padding::new(0.0).left(10.0))
                ]
            )
        })
        .width(Length::FillPortion(1))
        .spacing(10)
        .padding(50.0);

    let scrollable = widget::scrollable(input_column)
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::new()
        ));

    stack!(
        scrollable,
        add_button()
    )
    .into()
}

fn add_button<'a>() -> iced::Element<'a, Message> {
    container(
        button("Add")
            .on_press(Message::AddInput)
    )
    .padding(10.0)
    .align_x(Alignment::Start)
    .align_y(Alignment::Start)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

