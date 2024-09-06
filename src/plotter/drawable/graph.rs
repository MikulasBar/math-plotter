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

use math_lib::{prelude::*, functions::advanced::log};
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
            // .add_funcs(hashmap! {
            //     "log" => (vec!["a"], log::new(10.0, "a"))
            // })
            .build();

        self.func.eval(&ctx)
    }
        
    pub fn draw(&self, origin: Vec2, view: View, frame: &mut Frame) {
        // println!("--------------- {} ---------------", self.func.to_string());

        const AMOUNT: u16 = 5000;
        
        let mut points = nums_in_view(AMOUNT, view)
            .map(|x| (x, self.eval(x)))
            .filter_map(|(x, res)| validate(x, res))
            .map(|v| v.prepare_for_drawing(origin, view));


        let path = Path::new(|builder| {
            // move to starting vector
            let Some(start) = points.next() else {println!("no point is valid"); return};
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

mod froms {
    use super::*;

    impl From<GraphElem> for Element {
        fn from(graph: GraphElem) -> Self {
            Element::Graph(graph)
        }
    }
}



fn nums_in_view(amount: u16, view: View) -> impl Iterator<Item = f32> {
    let start = - (2.0 * view.offset.x + view.size.width) / (2.0 * view.zoom);
    let gap = view.size.width / (view.zoom * amount as f32);

    (0..amount)
        .map(move |n| {
            start + gap * (n as f32)
        })
}

fn validate(x: f32, res: EvalResult) -> Option<Vec2> {
    res.ok().map(|y| Vec2::new(x, y))
}