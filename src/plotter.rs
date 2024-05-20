use std::vec;

use rand::random;

use iced::{
    widget::canvas::{self, Path, Cache, Geometry, Frame},
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

        for _ in 0..40 {
            let x = random_signed() * 350.0;
            let y = random_signed() * 350.0;
            let point = Point{x, y};

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
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // fill background
            let bg_color = Color::from_rgb8(0x36, 0x39, 0x3F);
            background(frame, bg_color);
            
            // draw graphs
            let origin = Point::new(bounds.width / 2.0, bounds.height / 2.0);
            self.graphs.iter().for_each(|graph| {
                graph.draw(frame, origin);
            });
        });
        vec![geometry]
    }
}

fn background(frame: &mut Frame, color: Color) {
    let canvas_path = Path::rectangle(Point::ORIGIN, frame.size());
    frame.fill(&canvas_path, color);
}

fn random_signed() -> f32 {
    let sign = if random::<bool>() { 1.0 } else { -1.0 };
    sign * random::<f32>()
}

pub enum Graph2D {
    Point(Point, Color),
}

impl Graph2D {
    pub fn draw(&self, frame: &mut Frame, origin: Point) {
        let Point{x: ox, y: oy} = origin;

        match self {
            Graph2D::Point(Point{x, y}, color) => {
                let point = Point::new(*x + ox, *y + oy);
                let circle = Path::circle(point, 5.0);

                frame.fill(&circle, *color)
            },
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