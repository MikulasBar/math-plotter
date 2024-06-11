use iced::widget::{Canvas};

#[rustfmt::skip]
use iced::{
    self,
    widget::{canvas, column, container, text, text_input},
    Alignment, Application, Command, Element, Length, Color
};

#[rustfmt::skip]
use crate::{
    events::Message,
    plotter::{
        graph::Graph2D,
        plotter::*,
    },
    utilities::{rnd_color},
    vector::Vec2
};

pub fn run_app() -> iced::Result {
    App::run(iced::Settings::default())
}

struct App {
    plotter: Plotter2D,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let plotter = Plotter2D::builder()
            .add_control_points()
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
        let content = canvas(&self.plotter)
            .width(Length::Fixed(700.0))
            .height(Length::Fixed(700.0));

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}