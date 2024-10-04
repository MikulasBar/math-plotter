use iced::{
    self,
    widget::container,
    Application, Command, Length,
};

#[rustfmt::skip]
use crate::{
    message::Message,
    plotter::Plotter,
};




pub fn run_default() -> iced::Result {
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
        let plotter = Plotter::default();

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
            Message::KeyPressed(key) => {
                // self.plotter.update_view(glam::Vec2 { x: 0.0, y: 1.0 });
                println!("Key pressed: {:?}", key);
            },

            Message::UpdateView(offset) => {
                self.plotter.update_view(offset);
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        container(
            self.plotter.get_widget()
                .height(500)
                .width(500)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into() 
    }
}