pub mod plotter;
mod primitive;
mod events;
mod scene;
mod element;
mod render_pipeline;

pub use plotter::Plotter;

mod imports {
    
    
    pub(super) use crate::{
        message::Message,
        event,
        mouse_event
    };

    pub(super) use iced::{
        Point, Rectangle, Renderer, Theme, Size, Color, Vector, Transformation,
        mouse::{
            self,
            Event as MouseEvent,
            Button as MouseButton
        },
    };
}