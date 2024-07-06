use super::imports::{
    Frame,
    Path,
    Vec2,
    View,
    Stroke,
    Size,
};

pub fn horizontal(frame: &mut Frame, view: &View, origin: Vec2, stroke: Stroke) {
    let View {
        size: Size {width, ..},
        offset: Vec2 {y: off_y, ..},
        ..
    } = view;


    let x = width / 2.0;
    let y = *off_y;
    
    let start = Vec2::new(x, y) + origin;
    let end = start.flip_x();

    let path = Path::line(start.into(), end.into());

    frame.stroke(&path, stroke);
}


pub fn vertical(frame: &mut Frame, view: &View, origin: Vec2, stroke: Stroke) {
    let View {
        size: Size {height, ..},
        offset: Vec2 {x: off_x, ..},
        ..
    } = view;

    let x = *off_x;
    let y = height / 2.0;

    let start = Vec2::new(x, y) + origin;
    let end = start.flip_y();

    let path = Path::line(start.into(), end.into());

    frame.stroke(&path, stroke);
}