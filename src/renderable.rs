use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::Point;

pub trait Renderable {
    fn render(&self, framebuffer: &Framebuffer, origin: Point<i32>);
}