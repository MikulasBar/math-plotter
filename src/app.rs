use iced::widget::{rule, Space};
#[rustfmt::skip]
use iced::{
    self,
    widget::{self, container, text_input, text},
    Application, Command, Length, Alignment, 
};

#[rustfmt::skip]
use crate::{
    message::Message,
    plotter::{Plotter},
    color,
};




pub fn run_default() -> iced::Result {
    App::run(iced::Settings::default())
}

struct App {
    plotter: Plotter,
    input: String,
    status: String,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let plotter = Plotter::default();

        let app = App {
            plotter,
            input: "".to_string(),
            status: "".to_string(),
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        "Plotter app".to_string()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Transform_View(matrix) => {
                // self.plotter.update_view(view);
                // self.plotter.clear_cache();
            },
            Message::InputChanged(input) => {
                self.input = input;
            },
            Message::InputSubmitted => {
                
            },
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let content = self.plotter.get_widget();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}