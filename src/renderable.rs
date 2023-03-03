use wasm4::framebuffer::Framebuffer;

pub trait Renderable {
    fn render(&self, framebuffer: &Framebuffer);
}