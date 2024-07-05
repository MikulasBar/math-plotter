#[rustfmt::skip]
use iced::{
    self,
    widget::{canvas, container},
    Application, Command, Element, Length
};

#[rustfmt::skip]
use crate::{
    message::Message,
    plotter::Plotter,
};


pub fn run_app() -> iced::Result {
    App::run(iced::Settings::default())
}

struct App {
    plotter: Plotter,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let plotter = Plotter::builder()
            // .add_control_points()
            .size(700.0, 700.0)
            .add_sin_test()
            .build();

        let app = App {
            plotter,
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
            Message::UpdateView(view) => {
                self.plotter.clear_cache();
                self.plotter.update_view(view);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let content = self.plotter.canvas();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}