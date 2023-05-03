use alloc::string::String;
use core::cell::Cell;

use crate::renderable::{Canvas, Renderable, RenderContext, TextAlignment, TextWrapping};
use crate::ui::geometry::Size;
use crate::updatable::{Updatable, UpdateContext};

pub struct Text {
    pub text: String,
    pub alignment: TextAlignment,
    pub wrapping: TextWrapping,
    cached_size: Cell<Option<CachedSize>>,
}

#[derive(Copy, Clone)]
struct CachedSize {
    container_size: Size,
    size: Size,
}

impl Text {
    pub fn new(text: String) -> Self {
        Self {
            text,
            alignment: TextAlignment::Start,
            wrapping: TextWrapping::None,
            cached_size: Cell::new(None),
        }
    }

    pub fn size(&self, container_size: Size, canvas: &dyn Canvas) -> Size {
        if let Some(cached_size) = self.cached_size.get() {
            if cached_size.container_size == container_size {
                return cached_size.size
            }
        }
        let size = canvas.get_text_size(self.text.as_str(), container_size, self.wrapping);
        self.cached_size.set(Some(CachedSize {
            container_size,
            size,
        }));
        return size;
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