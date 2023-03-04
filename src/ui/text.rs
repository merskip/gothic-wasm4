use alloc::string::String;
use wasm4::geometry::{Point, Size};
use wasm4::{SYSTEM_CHAR_HEIGHT, SYSTEM_CHAR_WIDTH};
use wasm4::framebuffer::Framebuffer;
use crate::renderable::Renderable;

pub struct Text {
    pub text: String,
    pub alignment: TextAlignment,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TextAlignment {
    Start,
    Center,
    End,
}

impl Text {
    pub fn new(text: String) -> Self {
        Self {
            text,
            alignment: TextAlignment::Start,
        }
    }

    pub fn calculate_size(&self) -> Size<usize> {
        let lines_widths = self.text.lines().map(|line| line.len());
        let max_width = lines_widths.clone().max().unwrap_or(0);
        let lines_count = lines_widths.count();
        Size::new(
            max_width * SYSTEM_CHAR_WIDTH, // All system character are monospace
            lines_count * SYSTEM_CHAR_HEIGHT,
        )
    }
}

impl Renderable for Text {
    fn render(&self, framebuffer: &Framebuffer, origin: Point<i32>) {
        match self.alignment {
            TextAlignment::Start => self.render_aligned_start(framebuffer, origin),
            TextAlignment::Center => unimplemented!(),
            TextAlignment::End => unimplemented!(),
        }
    }
}

impl Text {
    fn render_aligned_start(&self, framebuffer: &Framebuffer, origin: Point<i32>) {
        framebuffer.text(&*self.text, origin);
    }\
}

