use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::renderable::{Canvas, Renderable, RenderContext, TextAlignment, TextWrapping};
use crate::ui::geometry::{Point, Size};
use crate::updatable::{Updatable, UpdateContext};

pub struct Text {
    pub text: String,
    pub alignment: TextAlignment,
    pub wrapping: TextWrapping,
}

impl Text {
    pub fn new(text: String) -> Self {
        Self {
            text,
            alignment: TextAlignment::Start,
            wrapping: TextWrapping::None,
        }
    }

    pub fn content_size(&self, canvas: &dyn Canvas, container_size: Size) -> Size {
        canvas.get_text_size(self.text.as_str(), container_size, self.wrapping)
    }
}

impl Updatable for Text {
    fn update(&mut self, _context: &mut UpdateContext) {}
}

impl Renderable for Text {
    fn render(&self, context: &mut RenderContext) {
        context.canvas.draw_text(
            self.text.as_str(),
            context.frame.origin,
            context.frame.size,
            self.wrapping,
            self.alignment,
        );
    }
}