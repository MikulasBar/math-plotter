use super::imports::*;

use builder::Builder;

pub struct Plotter {
    elements: Vec<Element>,
    settings: Settings,
    cache: Cache,
    view: View,
}

impl Plotter {
    pub fn builder() -> Builder {
        Builder::default()
    }

    // pub fn canvas(&self, width: Length, height: Length) -> Canvas<&Self, Message> {
    //     canvas(self)
    //         .width(width)
    //         .height(height)
    // }

    pub fn update_view(&mut self, view: View) {
        self.view = view;
    }

    // pub fn add_elements(&mut self, elements: Vec<Element>) {
    //     self.elements.extend(elements);
    // }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    fn draw_elements(&self, frame: &mut Frame, origin: &Vec2) {
        self.elements.iter().for_each(|elem| {
            elem.draw(frame, origin, &self.view);
        });
    }

    fn draw_background(&self, frame: &mut Frame) {
        let color = self.settings.background;
        let path = Path::rectangle(Point::ORIGIN, frame.size());
        frame.fill(&path, color);
    }

    fn draw_axis(&self, frame: &mut Frame, origin: &Vec2) {
        let color = self.settings.axis;
        let Size { width, height } = frame.size();
        let stroke = Stroke {
            style: Style::from(color),
            ..Default::default()
        };

        let View {
            offset: Vec2{x: o_x, y: o_y},
            ..
        } = self.view;

        // Draw horizontal axis
        let start_point = Vec2::new(width - o_x, 0.0)
            .prepare_for_drawing(*origin, &self.view);

        let end_point = Vec2::new(-width - o_x, 0.0)
            .prepare_for_drawing(*origin, &self.view);

        let path = Path::line(start_point, end_point);

        frame.stroke(&path, stroke.clone());



        // Draw vertical axis
        let start_point = Vec2::new(0.0, height + o_y)
            .prepare_for_drawing(*origin, &self.view);

        let end_point = Vec2::new(0.0, -height + o_y)
            .prepare_for_drawing(*origin, &self.view);

        let path = Path::line(start_point, end_point);

        frame.stroke(&path, stroke);
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
            _ => (),
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
            // self.draw_axis(frame, &origin);
            self.draw_elements(frame, &origin);
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

    use super::Plotter;

    #[rustfmt::skip]
    use super::super::imports::{
        Settings,
        Element,
        Color
    };

    pub struct Builder {
        settings: Settings,
        elements: Vec<Element>,
    }

    impl Default for Builder {
        fn default() -> Self {
            Self {
                settings: Settings::default(),
                elements: Vec::new(),
            }
        }
    }

    impl Builder {
        // pub fn add_elements(mut self, elements: Vec<Element>) -> Self {
        //     self.elements.extend(elements);
        //     self
        // }

        pub fn add_sin_test(mut self) -> Self {
            let sin = |x: f32| x.sin();
            self.elements.push(Element::from( (sin as fn(f32) -> f32, Color::WHITE) ));
            self
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

        pub fn build(self) -> Plotter {
            Plotter {
                settings: self.settings,
                elements: self.elements,
                ..Plotter::default()
            }
        }
    }
}