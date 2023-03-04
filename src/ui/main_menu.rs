use alloc::string::{String, ToString};
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::Point;
use crate::renderable::Renderable;
use crate::ui::text::Text;

pub struct MainMenu {
    pub items: [Text; 3],
    pub selected_index: usize
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            items: [
                Text::new("New game".to_string()),
                Text::new("Settings".to_string()),
                Text::new("Credits".to_string()),
            ],
            selected_index: 0
        }
    }
}

impl Renderable for MainMenu {
    fn render(&self, framebuffer: &Framebuffer, origin: Point<i32>) {
        let mut y = 0;
        for mut item in &self.items {
            item.render(framebuffer, Point::new(origin.x, origin.y + y));
            let item_size = item.calculate_size();
            y += item_size.height as i32;
        }
    }
}
