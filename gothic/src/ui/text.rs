use alloc::string::{String};
use wasm4::geometry::{Point, Rect, Size};
use wasm4::{get_char_size};
use wasm4::framebuffer::Framebuffer;
use crate::renderable::Renderable;
use crate::updatable::Updatable;

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

impl Updatable for Text {
    fn update(&mut self) {}
}

impl Renderable for Text {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect) {
        match self.alignment {
            TextAlignment::Start => self.render_aligned_start(framebuffer, frame),
            TextAlignment::Center => self.render_aligned_center(framebuffer, frame),
            TextAlignment::End => self.render_aligned_end(framebuffer, frame),
        }
    }
}

impl Text {
    fn render_aligned_start(&self, framebuffer: &Framebuffer, frame: Rect) {
        framebuffer.text(&*self.text, frame.origin);
    }

    fn render_aligned_center(&self, framebuffer: &Framebuffer, frame: Rect) {
        // TODO: Make works correctly with multilines
        let content_size = self.content_size();
        let x = (frame.size.width - content_size.width) / 2;
        framebuffer.text(&*self.text, Point::new(frame.origin.x + x as i32, frame.origin.y));
    }

    fn render_aligned_end(&self, framebuffer: &Framebuffer, frame: Rect) {
        // TODO: Make works correctly with multilines
        let content_size = self.content_size();
        let x = frame.size.width - content_size.width;
        framebuffer.text(&*self.text, Point::new(frame.origin.x + x as i32, frame.origin.y));
    }
}

