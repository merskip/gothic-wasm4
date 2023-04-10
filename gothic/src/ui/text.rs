use alloc::string::String;

use wasm4::geometry::{Point, Size};
use wasm4::get_char_size;

use crate::renderable::{Renderable, RenderContext};
use crate::updatable::{Updatable, UpdateContext};

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
    fn update(&mut self, _context: &mut UpdateContext) {}
}

impl Renderable for Text {
    fn render(&self, context: &mut RenderContext) {
        match self.alignment {
            TextAlignment::Start => self.render_aligned_start(context),
            TextAlignment::Center => self.render_aligned_center(context),
            TextAlignment::End => self.render_aligned_end(context),
        }
    }
}

impl Text {
    fn render_aligned_start(&self, context: &mut RenderContext) {
        context.framebuffer.text(&*self.text, context.frame.origin);
    }

    fn render_aligned_center(&self, context: &mut RenderContext) {
        // TODO: Make works correctly with multilines
        let content_size = self.content_size();
        let x = (context.frame.size.width - content_size.width) / 2;
        context.framebuffer.text(&*self.text, Point::new(context.frame.origin.x + x as i32, context.frame.origin.y));
    }

    fn render_aligned_end(&self, context: &mut RenderContext) {
        // TODO: Make works correctly with multilines
        let content_size = self.content_size();
        let x = context.frame.size.width - content_size.width;
        context.framebuffer.text(&*self.text, Point::new(context.frame.origin.x + x as i32, context.frame.origin.y));
    }
}

