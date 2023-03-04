use alloc::string::{ToString};
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect};
use crate::renderable::{Renderable};
use crate::ui::text::Text;

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
        text
    }
}

impl Renderable for MainMenu {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect) {
        let mut y = 0;
        for item in &self.items {
            let item_size = item.content_size();
            let item_frame = Rect::new(
                Point::new(frame.origin.x, frame.origin.y + y),
                item_size,
            );
            item.render(framebuffer, item_frame);

            y += item_size.height as i32;
        }
    }
}
