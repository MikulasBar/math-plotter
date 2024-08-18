pub mod drawable;
pub mod plotter;
pub mod view;
pub mod settings;
mod events;
mod vector;

pub use plotter::Plotter;

mod imports {
    pub(super) use super::{
        drawable::element::Element,
        view::View,
        settings::Settings,
        vector::Vec2,
    };
    
    pub(super) use crate::{
        message::Message,
        event,
        mouse_event
    };

    pub(super) use iced::{
        Point, Rectangle, Renderer, Theme, Size, Color,
        widget::canvas,
        widget::canvas::{
            Cache, Frame,Geometry, Path, Stroke, Style, Canvas,
            Event as CanvasEvent,
            event::Status as CanvasStatus,
        },
        mouse::{
            self,
            Event as MouseEvent,
            Button as MouseButton
        },
    };
}