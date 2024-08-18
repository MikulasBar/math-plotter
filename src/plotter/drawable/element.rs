#[rustfmt::skip]
use super::imports::{
    View,
    Vec2,
    GraphElem,
    PointElem,
    Color,
    Frame
};


#[derive(Debug, Clone)]
pub enum Element {
    Point(PointElem),
    Graph(GraphElem),
}

impl Element {
    pub fn draw(&self, frame: &mut Frame, origin: Vec2, view: &View) {
        match self {
            Self::Point(point) => {
                point.draw(origin, view, frame);
            },
            Self::Graph(graph) => {
                graph.draw(origin, view, frame);
            }
        }
    }

    pub fn graph(func: super::graph::Func, color: Color) -> Self{
        GraphElem::new(func, color).into()
    }

    // pub fn random_points<C>(num: u32, factor: f32, color_fn: C) -> Vec<Self>
    // where 
    //     C: Fn() -> Color,
    // {
    //     PointElem::random_points(num, factor, color_fn)
    // }
}

impl Default for Element {
    fn default() -> Self {
        PointElem::new(Vec2::ZERO, Color::WHITE).into()
    }
}


impl From<(Vec2, Color)> for Element {
    fn from(value: (Vec2, Color)) -> Self {
        PointElem::new(value.0, value.1).into()
    }
}

// impl From<(fn(f32) -> f32, Color)> for Element {
//     fn from(value: (fn(f32) -> f32, Color)) -> Self {
//         GraphElem::new(value.0, value.1).into()
//     }
// }