use iced::widget::Space;
#[rustfmt::skip]
use iced::{
    self,
    widget::{container, text_input, text, column as iced_column, TextInput},
    Application, Command, Length, Color
};
use math_lib::prelude::parse;

#[rustfmt::skip]
use crate::{
    message::Message,
    plotter::{Plotter, drawable::element::Element as Element},

};




pub fn run_app() -> iced::Result {
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
        let plotter = Plotter::builder()
            .size(700.0, 700.0)
            // .add_sin_test()
            .build();

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
            Message::UpdateView(view) => {
                self.plotter.clear_cache();
                self.plotter.update_view(view);
            },
            Message::InputChanged(input) => {
                self.input = input;
            },
            Message::InputSubmitted => {
                let parse_result = parse(&self.input);
                
                match parse_result {
                    Ok(_) => self.status = "Parsed Successfully".to_string(),
                    Err(e) => {
                        self.status = format!("ERROR: {}", e);
                        return Command::none();
                    },
                };
                
                let function = parse_result.unwrap();

                // use math_lib::prelude::{Child, Function};

                // let function = Child::Fn(Box::new(Function::Sin(Child::Var("x".to_string()))));

                println!("Parsed function: {:?}", function);

                self.plotter.clear_elements();
                self.plotter.add_element(
                    Element::graph(function, Color::from_rgb8(255, 0, 0))
                );
                self.plotter.clear_cache();
            },
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let plotter = self.plotter.canvas();

        let content = iced_column!(
            plotter,

            text_input("Enter a function", &self.input)
                .on_input(|input| Message::InputChanged(input))
                .on_submit(Message::InputSubmitted)
                .size(20)
                .width(Length::Fixed(300.0)),

            text(&self.status)
                .size(20)
                .width(Length::Fixed(300.0)),
        )
        .align_items(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}