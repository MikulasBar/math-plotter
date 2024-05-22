
#[rustfmt::skip]
use iced::{
    self,
    widget::{canvas, column, container, text, text_input},
    Alignment, Application, Command, Element, Length
};

use math_lib::{self, Parser, FnTree, Function};
use crate::plotter::*;

pub fn run_app() -> iced::Result {
    App::run(iced::Settings::default())
}

struct App {
    plotter: Plotter2D,
}

#[derive(Debug, Clone)]
enum Message {

}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let app = App {
            plotter: Plotter2D::new(
                Length::Fixed(700.0),
                Length::Fixed(700.0)
            ),
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
        let content = iced::widget::column!(
            self.plotter.display(),
        );

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into()
    }
}

use maplit::hashmap;

pub fn calculate(input: &str) -> f64 {
    let mut parser = Parser::new();
    let wrapped_func = parser.parse(input);

    if let Ok(func) = wrapped_func {
        let func = FnTree::new(func);

        let args = hashmap!{
            "x" => 0.0,
        };

        let result = func.evaluate(&args);

        if let Ok(value) = result {
            return value
        }
    }
    f64::NAN
}