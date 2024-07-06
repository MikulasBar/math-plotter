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

            let trans_coef = view.size.width / (2 * AMOUNT) as f32;
            let inv_zoom = 1.0 / view.zoom;
            
            // create vectors on the graph, its iterator,
            // collecting all items would be slow
            let mut points = (-AMOUNT..=AMOUNT)
                .map(|x| inv_zoom * (x as f32 * trans_coef - view.offset.x) )
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