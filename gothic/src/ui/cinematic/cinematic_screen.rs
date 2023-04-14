use alloc::format;

use wasm4::framebuffer::{DrawColorIndex, Framebuffer};
use wasm4::framebuffer::PaletteIndex::*;
use wasm4::gamepad::GamepadButton::ButtonX;
use wasm4::geometry::{Point, Rect, Size};
use wasm4::{char_x_button, get_char_size};

use crate::renderable::{Renderable, RenderContext};
use crate::updatable::{Updatable, UpdateContext};

pub struct CinematicScreen {
    text: &'static str,
    draw_art: fn(&Framebuffer, Rect),
}

impl CinematicScreen {
    pub const fn new(title: &'static str, draw: fn(&Framebuffer, Rect)) -> Self {
        Self { text: title, draw_art: draw }
    }
}

pub struct CinematicScreenView {
    screen: &'static CinematicScreen,
    current_text: &'static str,
    current_line_index: usize,

}

impl CinematicScreenView {
    const TEXT_LINES_COUNT: usize = 3;

    pub fn new(screen: &'static CinematicScreen) -> Self {
        let line_index = 0;
        Self {
            screen,
            current_text: screen.text.get_lines(line_index, Self::TEXT_LINES_COUNT).unwrap_or_default(),
            current_line_index: line_index,
        }
    }

    fn scroll_to_next_lines(&mut self) {
        self.current_line_index += Self::TEXT_LINES_COUNT;
        self.current_text = self.screen.text.get_lines(self.current_line_index, Self::TEXT_LINES_COUNT).unwrap_or_default();
    }

    pub fn is_finished(&self) -> bool {
        self.current_text.is_empty()
    }
}

impl Updatable for CinematicScreenView {
    fn update(&mut self, context: &mut UpdateContext) {
        if context.inputs.gamepad1.is_released(ButtonX) {
            self.scroll_to_next_lines();
        }
    }
}

impl Renderable for CinematicScreenView {
    fn render(&self, context: &mut RenderContext) {
        let panel_size = Size::new(
            context.frame.size.width,
            get_char_size().height * Self::TEXT_LINES_COUNT as u32 + 4,
        );
        let panel_frame = Rect::new(
            Point::new(context.frame.origin.x, context.frame.origin.y + context.frame.size.height as i32 - panel_size.height as i32),
            panel_size,
        );

        let art_frame = Rect::new(
            Point::new(context.frame.origin.x, context.frame.origin.y),
            Size::new(context.frame.size.width, context.frame.size.height - panel_size.height),
        );
        (self.screen.draw_art)(context.framebuffer, art_frame);

        context.framebuffer.set_draw_color(DrawColorIndex::Index1, Transparent);
        context.framebuffer.set_draw_color(DrawColorIndex::Index2, Palette4);
        context.framebuffer.rectangle(panel_frame.origin, panel_size);

        context.framebuffer.set_draw_color(DrawColorIndex::Index1, Palette3);
        context.framebuffer.set_draw_color(DrawColorIndex::Index2, Transparent);
        context.framebuffer.text(self.current_text, panel_frame.origin + Point::new(2, 2));

        let hint_text = format!("{} kontynuuj", char_x_button());
        let hint_size = Size::new(
            hint_text.len() as u32 * get_char_size().width,
            get_char_size().height,
        );
        let hint_origin = panel_frame.origin + Point::new((context.frame.size.width - hint_size.width) as i32, -(hint_size.height as i32));
        context.framebuffer.text(hint_text.as_str(), hint_origin);
    }
}

trait StringLinesUtilities<'a> {
    fn get_lines(&self, from_line: usize, count: usize) -> Option<&'a str>;
}

impl<'a> StringLinesUtilities<'a> for &'a str {
    fn get_lines(&self, from_line: usize, count: usize) -> Option<&'a str> {
        assert!(count > 0, "count_lines must be greater then 0");

        let mut lines = self.lines();
        let from = lines.nth(from_line)?;
        let from_index = unsafe {
            from.as_ptr().offset_from(self.as_ptr()) as usize
        };

        return if let Some(to) = lines.nth(count - 1) {
            let to_index = unsafe {
                to.as_ptr().offset_from(self.as_ptr()) as usize
            };
            Some(&self[from_index..to_index])
        } else {
            Some(&self[from_index..])
        };
    }
}
