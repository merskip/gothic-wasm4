use alloc::string::{String, ToString};
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::Point;
use crate::renderable::Renderable;

pub struct MainMenu {
    pub items: [MenuItem; 3],
    pub selected_index: usize
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            items: [
                MenuItem::new("New game".to_string()),
                MenuItem::new("Settings".to_string()),
                MenuItem::new("Credits".to_string()),
            ],
            selected_index: 0
        }
    }
}

impl Renderable for MainMenu {
    fn render(&self, framebuffer: &Framebuffer) {
        let mut y = 0;
        for item in &self.items {
            framebuffer.text(&*item.title, Point::new(0, y));
            y += 20;
        }
    }
}

pub struct MenuItem {
    pub title: String,
}

impl MenuItem {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

