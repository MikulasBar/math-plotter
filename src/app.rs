use iced::{
    self, widget::{column, container, text, text_input}, Application, Command, Element
};

pub fn run_app() -> iced::Result {
    App::run(iced::Settings::default())
}

struct App {
    input: String,
}

#[derive(Debug, Clone)]
enum Message {
    UpdateValue(String),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_: ()) -> (App, Command<Self::Message>) {
        let app = App {
            value: "".to_string(),
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
            Message::UpdateValue(input) => {
                self.input = input;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let value = calculate
        
        
        
        let content = column!(
            text_input("Placeholder", &self.value)
            .on_input(move |input| {Message::UpdateValue(input)})
            text(self.input.as_str()),
        );

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

use math_lib::{self, Parser, FnTree, Function};

pub fn calculate(input: &str) -> f64 {
    let parser = Parser::new();
    let tree = parser.parse(input).unwrap();

}