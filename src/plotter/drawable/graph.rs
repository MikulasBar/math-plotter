#[rustfmt::skip]
use super::imports::{
    Color,
    Frame,
    Path,
    Vec2,
    View,
    Stroke,
    Style,
    Element,
};



type Func = fn(f32) -> f32;

#[derive(Debug, Clone)]
pub struct GraphElem {
    func: Func,
    color: Color
}

impl GraphElem {
    pub fn new(func: Func, color: Color) -> Self {
        Self {
            func,
            color
        }
    }
    
    pub fn draw(&self, origin: &Vec2, view: &View, frame: &mut Frame) {
        let path = Path::new(|builder| {
            const MIN_RANGE     : i32 = 100;
            const RANGE_SCALE   : f32 = 0.1;

            let scaled_offset = view.offset.x / view.zoom;
            let scaled_range = (MIN_RANGE as f32 * view.zoom) as i32;
            let range = MIN_RANGE.max(scaled_range);
            
            // create vectors on the graph, its iterator,
            // collecting all items would be slow
            let mut points = (-range..=range)
                .map(|x| x as f32)
                .map(|x| x * RANGE_SCALE)
                .map(|x| x as f32 - scaled_offset)
                .map(|x| Vec2::new(x, (self.func)(x)))
                .map(|v| v.prepare_for_drawing(*origin, view));


            // move to starting vector
            let start = points.next()
                .unwrap();

            builder.move_to(start);


            for p in points {
                builder.line_to(p);
            }
            
        });

        let stroke = Stroke {
            style: Style::from(self.color),
            ..Default::default()
        };

        frame.stroke(&path, stroke);
    }
}


impl Into<Element> for GraphElem {
    fn into(self) -> Element {
        Element::Graph(self)
    }
}