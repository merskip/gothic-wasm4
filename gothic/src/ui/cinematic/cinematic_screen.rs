use wasm4::framebuffer::{DrawColorIndex, Framebuffer};
use wasm4::framebuffer::PaletteIndex::*;
use wasm4::geometry::{Point, Rect};
use wasm4::inputs::Inputs;
use wasm4::sprite::Sprite;
use crate::dispatcher::Dispatcher;
use crate::renderable::Renderable;
use crate::updatable::Updatable;

pub struct CinematicScreen {
    picture: Sprite,
    title: &'static str
}

impl CinematicScreen {
    pub const fn new(picture: Sprite, title: &'static str) -> Self {
        Self { picture, title }
    }
}

impl Updatable for CinematicScreen {
    fn update(&mut self, _inputs: &Inputs, _dispatcher: &mut Dispatcher) {}
}

impl Renderable for CinematicScreen {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect) {
        framebuffer.sprite(&self.picture, frame.origin);
        framebuffer.set_draw_color(DrawColorIndex::Index1, Palette3);
        framebuffer.set_draw_color(DrawColorIndex::Index2, Palette1);
        framebuffer.text(&*self.title, frame.origin + Point::new(2, 2));
    }
}