use math_lib::{self, Parser, FnTree, Function};
use iced::{
    self,
    widget::{column, container, text, text_input, button},
    Application, Command, Element, Alignment, Length
};

pub fn run_app() -> iced::Result {
    App::run(iced::Settings::default())
}

struct App {
    input: String,
    value: f64,
}

#[derive(Debug, Clone)]
enum Message {
    SaveInput(String),
    Calculate,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_: ()) -> (App, Command<Self::Message>) {
        let app = App {
            input: "x".to_string(),
            value: 0.0,
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
            Message::SaveInput(input) => {
                self.input = input;
            },
            Message::Calculate => {
                self.value = calculate(&self.input);
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let content = column!(
            text_input("Placeholder", &self.input)
                .on_input(move |input| Message::SaveInput(input)),

            button("Calculate")
                .on_press(Message::Calculate),

            text(&self.value),
        )
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fixed(400.0))
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