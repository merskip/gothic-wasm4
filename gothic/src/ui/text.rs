use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::max;

use wasm4::geometry::{Point, Size};
use wasm4::get_char_size;

use crate::renderable::{Renderable, RenderContext};
use crate::updatable::{Updatable, UpdateContext};

pub struct Text {
    pub text: String,
    pub alignment: TextAlignment,
    pub wrapping: TextWrapping,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TextAlignment {
    Start,
    Center,
    End,
}

pub enum TextWrapping {
    None,
    Character,
    Words,
}

impl Text {
    pub fn new(text: String) -> Self {
        Self {
            text,
            alignment: TextAlignment::Start,
            wrapping: TextWrapping::None,
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
        let text = self.get_text_with_wrapping(context.frame.size.width);
        context.framebuffer.text(&*text, context.frame.origin);
    }

    fn render_aligned_center(&self, context: &mut RenderContext) {
        // TODO: Make works correctly with multilines
        let content_size = self.content_size();
        let x = (context.frame.size.width - content_size.width) / 2;
        let text = self.get_text_with_wrapping(context.frame.size.width);
        context.framebuffer.text(&*text, Point::new(context.frame.origin.x + x as i32, context.frame.origin.y));
    }

    fn render_aligned_end(&self, context: &mut RenderContext) {
        // TODO: Make works correctly with multilines
        let content_size = self.content_size();
        let x = context.frame.size.width - content_size.width;
        let text = self.get_text_with_wrapping(context.frame.size.width);
        context.framebuffer.text(&*text, Point::new(context.frame.origin.x + x as i32, context.frame.origin.y));
    }
}

impl Text {
    fn get_text_with_wrapping(&self, max_width: u32) -> String {
        match self.wrapping {
            TextWrapping::None => self.text.to_string(),
            TextWrapping::Character => Self::wrapping_text_by_character(&self.text, max_width),
            TextWrapping::Words => Self::wrapping_text_by_words(&self.text, max_width),
        }
    }

    fn wrapping_text_by_character(text: &str, max_width: u32) -> String {
        let max_chars_per_line = max_width / get_char_size().width;
        let lines = text.chars()
            .collect::<Vec<char>>()
            .chunks(max_chars_per_line as usize)
            .map(|chunk| chunk.iter().collect())
            .collect::<Vec<String>>();

        return lines.join("\n");
    }

    fn wrapping_text_by_words(text: &str, max_width: u32) -> String {
        todo!("Not implemented yet")
    }
}

