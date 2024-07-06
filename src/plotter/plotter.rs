use super::imports::*;
use super::drawable::bisector;

use builder::{Builder, Unsized};

pub struct Plotter {
    elements: Vec<Element>,
    settings: Settings,
    cache: Cache,
    view: View,
}

impl Plotter {
    pub fn builder() -> Builder<Unsized> {
        Builder::<Unsized>::new()
    }

    pub fn canvas(&self) -> Canvas<&Self, Message> {
        let size = self.view.size;

        canvas(self)
            .width(size.width)
            .height(size.height)
    }

    pub fn update_view(&mut self, view: View) {
        self.view = view;
    }

    // pub fn add_elements(&mut self, elements: Vec<Element>) {
    //     self.elements.extend(elements);
    // }

    fn get_origin(&self) -> Vec2 {
        let size = self.view.size;
        Vec2::new(size.width, size.height) / 2.0
    }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    fn draw_elements(&self, frame: &mut Frame, origin: Vec2) {
        self.elements.iter().for_each(|elem| {
            elem.draw(frame, origin, &self.view);
        });
    }

    fn draw_background(&self, frame: &mut Frame) {
        let color = self.settings.background;
        let path = Path::rectangle(Point::ORIGIN, frame.size());

        frame.fill(&path, color);
    }

    fn draw_axis(&self, frame: &mut Frame, origin: Vec2) {
        let color = self.settings.axis;
        let stroke = Stroke::default()
            .with_color(color);

        bisector::horizontal(frame, &self.view, origin, stroke.clone());
        bisector::vertical(frame, &self.view, origin, stroke);
    }
}

impl Default for Plotter {
    fn default() -> Self {
        Self {
            elements: vec![],
            cache: Cache::default(),
            view: View::default(),
            settings: Settings::default(),
        }
    }
}

impl canvas::Program<Message> for Plotter {
    type State = State;

    fn update(
            &self,
            state: &mut Self::State,
            event: CanvasEvent,
            _bounds: Rectangle,
            cursor: mouse::Cursor,
    ) -> (CanvasStatus, Option<Message>) {
        // if !cursor.is_over(bounds) {
        //     return (CanvasStatus::Ignored, None);
        // }
        
        match event {
            event!(MOUSE_LEFT_DOWN) => {
                let pos: Vec2 = cursor.position().unwrap().into();
                *state = State::LeftButtonDown(pos);
            },
            event!(MOUSE_LEFT_UP) => {
                *state = State::Idle;
            },
            event!(MOUSE_MOVE: new_pos) => {
                if let State::LeftButtonDown(old_pos) = *state {
                    let old_offset = self.view.offset;
                    let offset: Vec2 = Vec2::from(new_pos) - old_pos;

                    let view = View {
                        offset: old_offset + offset,
                        ..self.view
                    };

                    // update cursor position
                    *state = State::LeftButtonDown(new_pos.into());

                    return (CanvasStatus::Captured, Some(Message::UpdateView(view)))
                }
            },
            event!(MOUSE_SCROLL: delta) => {
                let old_zoom = self.view.zoom;
                let coef = View::zoom_coef(delta);
                
                let view = View {
                    zoom: old_zoom * (1.0 + coef),
                    ..self.view
                };

                return (CanvasStatus::Captured, Some(Message::UpdateView(view)))
            },
            _ => (),
        }
        (CanvasStatus::Ignored, None)
    }


    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> { 
        
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let origin = Vec2::new(bounds.width, bounds.height) / 2.0;

            self.draw_background(frame);    
            self.draw_axis(frame, origin);
            self.draw_elements(frame, origin);

            // println!("{:?}", bounds);
        });

        vec![geometry]
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum State {
    #[default]
    Idle,
    LeftButtonDown(Vec2),
}


mod builder {

    use std::marker::PhantomData;

    use super::{Plotter, View};

    #[rustfmt::skip]
    use super::super::imports::{
        Settings,
        Element,
        Color,
        Size,
    };

    pub struct Unsized;
    pub struct Sized;

    pub struct Builder<T> {
        settings: Settings,
        elements: Vec<Element>,
        view: View,
        _marker: PhantomData<T>
    }

    impl<T> Builder<T> {
        // pub fn add_elements(mut self, elements: Vec<Element>) -> Self {
        //     self.elements.extend(elements);
        //     self
        // }

        pub fn new() -> Builder<Unsized> {
            Builder {
                settings: Settings::default(),
                elements: Vec::new(),
                view: View::default(),
                _marker: PhantomData
            }
        }

        pub fn add_sin_test(mut self) -> Self {
            let red_color = Color::from_rgb8(255, 0, 0);
            let sin = (|x: f32| x.sin()) as fn(f32) -> f32;

            self.elements.push((sin, red_color).into());
            self
        }

        pub fn size(self, width: f32, height: f32) -> Builder<Sized> {
            let Builder { mut view, ..} = self;
            view.size = Size::new(width, height);

            Builder {
                settings: self.settings,
                elements: self.elements,
                view,
                _marker: PhantomData,
            }
        }

        // pub fn add_control_points(mut self) -> Self {
        //     let center = Element::from( (Vec2::ZERO, Color::WHITE) );
        //     let right = Element::from( (Vec2::UNIT_X * 100.0, Color::WHITE) );
        //     let up = Element::from( (Vec2::UNIT_Y * 100.0, Color::WHITE) );
        
        //     self.elements.extend(vec![center, right, up]);

        //     self
        // }

        // pub fn background(mut self, color: Color) -> Self {
        //     self.settings.background = color;
        //     self
        // }
    }

    impl Builder<Sized> {
        pub fn build(self) -> Plotter {
            Plotter {
                settings: self.settings,
                elements: self.elements,
                view: self.view,
                ..Plotter::default()
            }
        }
    }
}