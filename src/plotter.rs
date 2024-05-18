use std::vec;

use rand::random;

use iced::{
    widget::canvas::{self, Path, Cache, Geometry},
    Point, Rectangle, Renderer, Theme, Color, mouse
    
};

pub struct Plotter2D {
    graphs: Vec<Graph2D>,
    view: View,
    cache: Cache
}

impl Plotter2D {
    pub fn new() -> Self {
        let mut graphs = Vec::new();

        for _ in 0..20 {
            let x = random::<f32>() * 200.0;
            let y = random::<f32>() * 200.0;
            let point = Point::new(x, y);

            let r = random::<u8>();
            let g = random::<u8>();
            let b = random::<u8>();

            let color = Color::from_rgb8(r, g, b);
            graphs.push(Graph2D::Point(point, color));
        }
        
        Self {
            graphs,
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
        let cache = self.cache.draw(renderer, bounds.size(), |frame| {
            let origin = frame.center();
            
            for graph in &self.graphs {
                match graph {
                    Graph2D::Point(Point{x, y}, color)=> {
                        let point = Point::new(origin.x + x, origin.y + y);
                        let circle = Path::circle(point, 5.0);

                        frame.fill(&circle, *color)
                    },
                }
            }
        });
        vec![cache]
    }
}

pub enum Graph2D {
    Point(Point, Color),
    // Polygon(),
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