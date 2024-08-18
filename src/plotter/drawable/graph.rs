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

use math_lib::prelude::*;
use maplit::hashmap;


pub type Func = Child;

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

    fn eval(&self, x: f32) -> EvalResult {
        let ctx = Context::builder()
            .add_elementary()
            .add_vars(hashmap!{"x" => x})
            .build();

        self.func.eval(&ctx)
    }
    
    pub fn draw(&self, origin: Vec2, view: &View, frame: &mut Frame) {
        // amount of points to draw
        const AMOUNT: i32 = 100;

        let trans_coef = view.size.width / (2 * AMOUNT) as f32;
        let inv_zoom = 1.0 / view.zoom;

        println!("--------------- {} ---------------", self.func.to_string());
        
        // create vectors on the graph, its iterator,
        // collecting all items would be slow
        let mut points = (-AMOUNT..=AMOUNT)
            // make x coordinates such that all points are evenly spaced, and all are visible
            .map(|x| inv_zoom * (x as f32 * trans_coef - view.offset.x) )
            // make x, y coordinates, filter out invalid values
            .filter_map(|x| {
                if let Ok(y) = self.eval(x) {
                    println!("x = {x}, y = {y}");
                    Some(Vec2::new(x, y))
                } else {
                    println!("invalid x = {x}");
                    None
                }
            })
            // prepare for drawing
            .map(|v| v.prepare_for_drawing(origin, view));

        let path = Path::new(|builder| {
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

        frame.stroke(&path, stroke);
    }
}


impl Into<Element> for GraphElem {
    fn into(self) -> Element {
        Element::Graph(self)
    }
}