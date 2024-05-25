
use iced::widget::{canvas::Program, Canvas};
#[rustfmt::skip]
use iced::{
    self,
    widget::{canvas, column, container, text, text_input},
    Alignment, Application, Command, Element, Length, Color,
};

//use math_lib::{self, Parser, FnTree, Function};
use crate::{
    graph::Graph2D,
    vector::Vec2,
    plotter::Plotter2D,
    events::Message,
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
        let mut plotter = Plotter2D::default();

        let a = Graph2D::Point(Vec2::ZERO, Color::WHITE);
        let b = Graph2D::Point(Vec2::UNIT_X * 100.0, Color::WHITE);
        let c = Graph2D::Point(Vec2::UNIT_Y * 100.0, Color::WHITE);

        plotter.push(a);
        plotter.push(b);
        plotter.push(c);

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

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
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