use alloc::vec::Vec;

use gothic::renderable::{Canvas, Color, Image, TextAlignment, TextWrapping};
use gothic::ui::geometry::{Point, Size};
use wasm4::{get_char_height, get_char_width};
use wasm4::framebuffer::{Framebuffer, PaletteIndex};

use crate::SpriteImage;

pub struct Wasm4Canvas<'a> {
    framebuffer: &'a Framebuffer,
}

impl<'a> Wasm4Canvas<'a> {
    pub fn new(framebuffer: &'a Framebuffer) -> Self {
        Self { framebuffer }
    }
}

impl<'a> Canvas for Wasm4Canvas<'a> {
    fn get_size(&self) -> Size {
        Size::new(self.framebuffer.get_screen_width(),
                  self.framebuffer.get_screen_height())
    }

    // Line

    fn set_line_color(&self, color: Color) {
        self.framebuffer.set_draw_colors([
            Some(color_to_palette_index(color)),
            None,
            None,
            None,
        ])
    }

    fn draw_line(&self, start: Point, end: Point) {
        self.framebuffer.line(start.x, start.y, end.x, end.y);
    }

    // Rectangle

    fn set_rectangle_color(&self, fill_color: Color, outline_color: Color) {
        self.framebuffer.set_draw_colors([
            Some(color_to_palette_index(fill_color)),
            Some(color_to_palette_index(outline_color)),
            None,
            None,
        ]);
    }

    fn draw_rectangle(&self, start: Point, size: Size) {
        self.framebuffer.rectangle(start.x, start.y, size.width, size.height);
    }

    fn get_text_size(&self, text: &str, container_size: Size, text_wrapping: TextWrapping) -> Size {
        let lines = Self::get_text_with_wrapping(text, container_size.width, text_wrapping);
        let lines_widths = lines.iter().map(|line| line.len());
        let max_line_chars = lines_widths.clone().max().unwrap_or(0) as u32;
        let lines_count = lines_widths.count() as u32;

        Size::new(
            max_line_chars * get_char_width(),  // All system character are monospace
            lines_count * get_char_height(),
        )
    }

    fn set_text_color(&self, foreground: Color, background: Color) {
        self.framebuffer.set_draw_colors([
            Some(color_to_palette_index(foreground)),
            Some(color_to_palette_index(background)),
            None,
            None,
        ]);
    }

    fn draw_text(&self, text: &str, start: Point, size: Size, text_wrapping: TextWrapping, text_alignment: TextAlignment) {
        let lines = Self::get_text_with_wrapping(text, size.width, text_wrapping);
        match text_alignment {
            TextAlignment::Start => {
                let mut y = start.y;
                for line in lines {
                    self.framebuffer.text(line, start.x, y);
                    y += get_char_height() as i32;
                }
            }
            TextAlignment::Center => {
                let text_size = self.get_text_size(text, size, text_wrapping);
                let x = (size.width - text_size.width) / 2;
                self.framebuffer.text(text, start.x + x as i32, start.y);
            }
            TextAlignment::End => {
                let text_size = self.get_text_size(text, size, text_wrapping);
                let x = size.width - text_size.width;
                self.framebuffer.text(text, start.x + x as i32, start.y);
            }
        }
    }

    // Image

    fn set_image_colors(&self, colors: [Color; 4]) {
        self.framebuffer.set_draw_colors([
            Some(color_to_palette_index(colors[0])),
            Some(color_to_palette_index(colors[1])),
            Some(color_to_palette_index(colors[2])),
            Some(color_to_palette_index(colors[3])),
        ]);
    }

    fn draw_image(&self, image: &dyn Image, start: Point) {
        let sprite_image = image.as_any()
            .downcast_ref::<SpriteImage>()
            .unwrap();
        self.framebuffer.sprite(sprite_image.0, start.x, start.y);
    }
}

impl Wasm4Canvas<'_> {
    fn get_text_with_wrapping(text: &str, max_width: u32, text_wrapping: TextWrapping) -> Vec<&str> {
        match text_wrapping {
            TextWrapping::None => text.lines().collect(),
            TextWrapping::Words => Self::wrap_text_by_words(text, max_width),
        }
    }

    fn wrap_text_by_words(text: &str, max_width: u32) -> Vec<&str> {
        let max_chars_per_line = max_width / get_char_width();

        let mut lines = Vec::new();
        let mut current_line = 0..0;

        for word in text.split_whitespace() {
            let word_start = unsafe {
                word.as_ptr().offset_from(text.as_ptr()) as usize
            };
            let word_end = unsafe {
                word.as_ptr().add(word.len()).offset_from(text.as_ptr()) as usize
            };
            let whitespace = unsafe {
                text.get_unchecked(current_line.end..word_start)
            };

            if whitespace.contains('\n') || current_line.len() + whitespace.len() + word.len() > max_chars_per_line as usize {
                lines.push(current_line);
                current_line = word_start..word_end;
            }
            current_line.end = word_end;
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        unsafe {
            lines.iter().map(|line_range| text.get_unchecked(line_range.clone())).collect()
        }
    }
}


fn color_to_palette_index(color: Color) -> PaletteIndex {
    match color {
        Color::Transparent => PaletteIndex::Transparent,
        Color::Background => PaletteIndex::Palette1,
        Color::Primary => PaletteIndex::Palette2,
        Color::Secondary => PaletteIndex::Palette3,
        Color::Tertiary => PaletteIndex::Palette4,
    }
}