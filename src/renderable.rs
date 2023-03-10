use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::Rect;

pub trait Renderable {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect);
}
