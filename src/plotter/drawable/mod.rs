pub mod element;
pub mod bisector;
mod point;
mod graph;




mod imports {
    pub(super) use crate::plotter::{
        vector::Vec2,
        view::View,
    };

    pub(super) use super::{
        element::Element,
        graph::GraphElem,
        point::PointElem,
    };

    pub(super) use iced::{
        Color, Size,
        widget::canvas::{Frame, Path, Stroke, Style}
    };
}