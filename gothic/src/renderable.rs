use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::Rect;

use crate::updatable::Updatable;

pub trait Renderable: Updatable {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect);
}
