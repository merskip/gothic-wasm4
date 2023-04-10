use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::Rect;

use crate::updatable::Updatable;

pub trait Renderable: Updatable {
    fn render(&self, context: &mut RenderContext);
}

pub struct RenderContext<'a> {
    pub framebuffer: &'a Framebuffer,
    pub frame: Rect
}

impl<'a> RenderContext<'a> {
    pub fn new(framebuffer: &'a Framebuffer, frame: Rect) -> Self {
        Self { framebuffer, frame }
    }

    pub fn with_frame(&self, frame: Rect) -> Self {
        Self {
            framebuffer: self.framebuffer,
            frame
        }
    }
}
