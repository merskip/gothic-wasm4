use alloc::string::{ToString};
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect, Size};
use crate::renderable::{Renderable};
use crate::ui::text::Text;
use crate::ui::text::TextAlignment::Center;

pub struct MainMenu {
    pub items: [Text; 3],
    pub selected_index: usize
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            items: [
                Self::make_menu_item("New game"),
                Self::make_menu_item("Settings"),
                Self::make_menu_item("Credits"),
            ],
            selected_index: 0
        }
    }

    fn make_menu_item(title: &str) -> Text {
        let mut text = Text::new(title.to_string());
        text.alignment = Center;
        text
    }
}

impl Renderable for MainMenu {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect) {
        self.render_menu_items(framebuffer, frame)
    }
}

impl MainMenu {
    fn render_menu_items(&self, framebuffer: &Framebuffer, frame: Rect) {
        let mut y = 8;
        for (index, item) in self.items.iter().enumerate() {
            let item_size = item.content_size();
            let item_frame = Rect::new(
                Point::new(frame.origin.x, frame.origin.y + y),
                Size::new(frame.size.width, item_size.height),
            );
            let is_selected = index == self.selected_index;
            if is_selected {
                framebuffer.set_color(3);
                self.render_selected_item_indicator(framebuffer, frame, y - 2, item_size.height + 3);
            } else {
                framebuffer.set_color(2);
            }
            item.render(framebuffer, item_frame);

            y += item_size.height as i32 + 6;
        }
    }

    fn render_selected_item_indicator(&self, framebuffer: &Framebuffer, frame: Rect, y: i32, item_height: u32) {
        framebuffer.line_horizontal(Point::new(frame.origin.x, frame.origin.y + y), frame.size.width);
        framebuffer.line_horizontal(Point::new(frame.origin.x, frame.origin.y + y + item_height as i32), frame.size.width);
    }
}