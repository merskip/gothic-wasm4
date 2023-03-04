use alloc::string::String;
use wasm4::geometry::{Rect, Size};
use wasm4::{get_char_size};
use wasm4::framebuffer::Framebuffer;
use crate::renderable::{Renderable};

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

    pub fn content_size(&self) -> Size<u32> {
        let lines_widths = self.text.lines().map(|line| line.len());
        let max_width = lines_widths.clone().max().unwrap_or(0) as u32;
        let lines_count = lines_widths.count() as u32;
        let char_size = get_char_size();
        Size::new(
            max_width * char_size.width, // All system character are monospace
            lines_count * char_size.height,
        )
    }
}

impl Renderable for Text {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect) {
        match self.alignment {
            TextAlignment::Start => self.render_aligned_start(framebuffer, frame),
            TextAlignment::Center => unimplemented!(),
            TextAlignment::End => unimplemented!(),
        }
    }
}

impl Text {
    fn render_aligned_start(&self, framebuffer: &Framebuffer, frame: Rect) {
        framebuffer.text(&*self.text, frame.origin);
    }
}

