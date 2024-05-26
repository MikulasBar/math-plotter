
use iced::widget::{canvas::Program, Canvas};
#[rustfmt::skip]
use iced::{
    self,
    widget::{canvas, column, container, text, text_input},
    Alignment, Application, Command, Element, Length, Color,
};

//use math_lib::{self, Parser, FnTree, Function};
use crate::{
    events::Message, graph::Graph2D, plotter::Plotter2D, utilities::{add_control_points, rnd_color, rnd_vector}, vector::Vec2
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
        add_control_points(&mut plotter);

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
            Message::Redraw => {
                self.plotter.clear_graphs();
                self.plotter.clear_cache();

                let graphs = (0..10).into_iter()
                    .map(|_| {
                        let vec = rnd_vector(100.0);
                        let color = rnd_color();
                        Graph2D::Point(vec, color)
                    })
                    .collect::<Vec<_>>();

                self.plotter.add_graphs(graphs);
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