use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::renderable::{Canvas, Renderable, RenderContext};
use crate::ui::geometry::{Point, Size};
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

    pub fn content_size(&self, size: Size, canvas: &dyn Canvas) -> Size {
        let text = self.get_text_with_wrapping(size.width, canvas);
        canvas.get_text_size(text.as_str())
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
        let text = self.get_text_with_wrapping(context.frame.size.width, context.canvas);
        context.canvas.draw_text(&*text, context.frame.origin);
    }

    fn render_aligned_center(&self, context: &mut RenderContext) {
        // TODO: Make works correctly with multilines
        let content_size = self.content_size(context.frame.size, context.canvas);
        let x = (context.frame.size.width - content_size.width) / 2;
        let text = self.get_text_with_wrapping(context.frame.size.width, context.canvas);
        context.canvas.draw_text(&*text, Point::new(context.frame.origin.x + x as i32, context.frame.origin.y));
    }

    fn render_aligned_end(&self, context: &mut RenderContext) {
        // TODO: Make works correctly with multilines
        let content_size = self.content_size(context.frame.size, context.canvas);
        let x = context.frame.size.width - content_size.width;
        let text = self.get_text_with_wrapping(context.frame.size.width, context.canvas);
        context.canvas.draw_text(&*text, Point::new(context.frame.origin.x + x as i32, context.frame.origin.y));
    }
}

impl Text {
    fn get_text_with_wrapping(&self, max_width: u32, canvas: &dyn Canvas) -> String {
        let max_chars_per_line = max_width / canvas.get_text_metrics(&self.text).maximum_character_width;
        match self.wrapping {
            TextWrapping::None => return self.text.to_string(),
            TextWrapping::Character => Self::wrapping_text_by_character(&self.text, max_chars_per_line),
            TextWrapping::Words => Self::wrapping_text_by_words(&self.text, max_chars_per_line),
        }.join("\n")
    }

    fn wrapping_text_by_character(text: &str, max_length: u32) -> Vec<String> {
        text.chars()
            .collect::<Vec<char>>()
            .chunks(max_length as usize)
            .map(|chunk| chunk.iter().collect())
            .collect::<Vec<String>>()
    }

    fn wrapping_text_by_words(text: &str, max_length: u32) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 > max_length as usize {
                lines.push(current_line.trim().to_string());
                current_line = String::new();
            }
            current_line.push_str(word);
            current_line.push(' ');
        }

        if !current_line.is_empty() {
            lines.push(current_line.trim().to_string());
        }

        lines
    }
}

