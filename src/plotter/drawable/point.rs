#[rustfmt::skip]
use super::imports::{
    Color,
    Vec2,
    Element,
    Frame,
    Path,
    View,
};


#[derive(Debug, Clone)]
pub struct PointElem {
    pos: Vec2,
    color: Color,
}

impl PointElem {
    pub fn new(pos: Vec2, color: Color) -> Self {
        Self {
            pos,
            color
        }
    }

    pub fn draw(&self, origin: Vec2, view: &View, frame: &mut Frame) {
        let point = self.pos.prepare_for_drawing(origin, view);
        let circle = Path::circle(point, 5.0);

        frame.fill(&circle, self.color)
    }

    // pub fn random_points<C>(num: u32, factor: f32, color_fn: C)  -> Vec<Element>
    // where
    //     C: Fn() -> Color
    // {
    //     (0..num).into_iter()
    //         .map(move |_| {
    //             let vector = Vec2::random(factor);
    //             let color = color_fn();
    //             Self::new(vector, color).into()
    //         })
    //         .collect()
    // }
}

impl Into<Element> for PointElem {
    fn into(self) -> Element {
        Element::Point(self)
    }
}


//std::iter::Map<std::ops::Range<u32>, impl Fn(u32) -> Self>

