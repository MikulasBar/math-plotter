use std::vec;
use iced::{
    mouse, widget::canvas::{self, path::lyon_path::polygon, Cache, Frame, Geometry, Path}, Color, Point, Rectangle, Renderer, Theme
};

use crate::utilities;

pub struct Plotter2D {
    graphs: Vec<Graph2D>,
    view: View,
    cache: Cache
}

// point factor = 350.0
impl Plotter2D {
    pub fn new() -> Self {
        let mut points: Vec<Point> = Vec::new();

        for _ in 0..3 {
            let point = utilities::rnd_point(100.0);
            points.push(point);
        }

        let color = utilities::rnd_color();
        let polygon = Graph2D::Polygon(points, color);
        
        Self {
            graphs: vec![polygon],
            view: View::default(),
            cache: Cache::default()
        }
    }

    pub fn from_graphs(graphs: Vec<Graph2D>) -> Self {
        Self {
            graphs,
            view: View::default(),
            cache: Cache::default()
        }
    }

    // pub fn push(&mut self, graph: Graph2D) {
    //     self.graphs.push(graph);
    // }

    // pub fn pop(&mut self) -> Option<Graph2D> {
    //     self.graphs.pop()
    // }
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
            self.graphs.iter().for_each(|graph| {
                graph.draw(frame, origin);
            });
        });
        vec![geometry]
    }
}

pub enum Graph2D {
    Point(Point, Color),
    Polygon(Vec<Point>, Color),
}

impl Graph2D {
    pub fn draw(&self, frame: &mut Frame, origin: Point) {
        let Point{x: ox, y: oy} = origin;
        
        match self {
            Self::Point(Point{x, y}, color) => {
                let point = Point::new(*x + ox, *y + oy);
                let circle = Path::circle(point, 5.0);

                frame.fill(&circle, *color)
            },
            Self::Polygon(points, color) => {
                if points.len() < 2 {
                    return;
                }

                let path = Path::new(|builder| {
                    builder.move_to(points[0]);

                    for p in points.iter().skip(1) {
                        let point = Point::new(p.x + ox, p.y + oy);
                        builder.line_to(point);
                    }
                    builder.close();
                });
                
                frame.fill(&path, *color);
            }
        }
    }
} 

impl Default for Graph2D {
    fn default() -> Self {
        Self::Point(Point::default(), Color::WHITE)
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