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
    
    pub fn draw(&self, origin: Vec2, view: &View, frame: &mut Frame) {
        let path = Path::new(|builder| {
            const AMOUNT: i32 = 100;

            let scaled_offset = view.offset.x / view.zoom;
            let half_x_size = view.size.width / 2.0;
            let trans_coef = half_x_size / view.zoom / AMOUNT as f32;
            
            // create vectors on the graph, its iterator,
            // collecting all items would be slow
            let mut points = (-AMOUNT..=AMOUNT)
                .map(|x| x as f32 * trans_coef - scaled_offset)
                .map(|x| Vec2::new(x, (self.func)(x)))
                .map(|v| v.prepare_for_drawing(origin, view));


            // move to starting vector
            let start = points.next().unwrap();
            builder.move_to(start);


            for p in points {
                builder.line_to(p);
            }
            
        });

        let stroke = Stroke {
            style: Style::from(self.color),
            width: 3.0,
            ..Default::default()
        };
        // println!("{:?}", stroke);

        frame.stroke(&path, stroke);
    }
}


impl Into<Element> for GraphElem {
    fn into(self) -> Element {
        Element::Graph(self)
    }
}