use std::vec;
#[rustfmt::skip]
use iced::{
    mouse,
    widget::{canvas::{Cache, Frame, Geometry, Path}, Canvas, canvas},
    Color, Length, Point, Rectangle, Renderer, Theme
};

#[rustfmt::skip]
use crate::{
    utilities::{self, map_with_origin},
    graph::Graph2D
};

pub struct Plotter2D {
    graphs: Vec<Graph2D>,
    view: View,
    cache: Cache,

    width: Length,
    height: Length
}

// point factor = 350.0
impl Plotter2D {
    pub fn new(width: Length, height: Length) -> Self {
        let mut points: Vec<Point> = Vec::new();

        for _ in 0..3 {
            let point = utilities::rnd_point(100.0);
            points.push(point);
        }

        let color = utilities::rnd_color();
        let polygon = Graph2D::Polygon(points, color);
        
        Self {
            graphs: vec![polygon],
            width,
            height,
            ..Self::default()
        }
    }

    pub fn display<M>(&self) -> Canvas<&Self, M> {
        canvas(self)
            .width(self.width)
            .height(self.height)
    }

    // pub fn push(&mut self, graph: Graph2D) {
    //     self.graphs.push(graph);
    // }

    // pub fn pop(&mut self) -> Option<Graph2D> {
    //     self.graphs.pop()
    // }
}

impl Default for Plotter2D {
    fn default() -> Self {
        Self {
            graphs: vec![],
            view: View::default(),
            cache: Cache::default(),

            width: Length::Fixed(700.0),
            height: Length::Fixed(700.0)
        }
    }
}

impl<Message> canvas::Program<Message> for Plotter2D {
    type State = ();
    
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // fill background
            let bg_color = Color::from_rgb8(0x36, 0x39, 0x3F);
            utilities::background(frame, bg_color);
            
            // draw graphs
            let origin = Point::new(bounds.width / 2.0, bounds.height / 2.0);
            // self.graphs.iter().for_each(|graph| {
            //     graph.draw(frame, origin);
            // });

            let g = Graph2D::Point(Point{x: 0.0, y: 0.0}, Color::WHITE);
            let h = Graph2D::Point(Point{x: 0.0, y: 100.0}, Color::WHITE);
            let j = Graph2D::Point(Point{x: 100.0, y: 0.0}, Color::WHITE);
            g.draw(frame, origin);
            h.draw(frame, origin);
            j.draw(frame, origin);
        });
        vec![geometry]
    }
}


struct View {
    offset: (f32, f32),
    zoom: f32,
}

impl Default for View {
    fn default() -> Self {
        Self {
            offset: (0.0, 0.0),
            zoom: 1.0
        }
    }
}