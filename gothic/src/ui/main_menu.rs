use alloc::rc::Rc;
use alloc::string::{ToString};
use core::cell::{Cell, RefCell};
use wasm4::framebuffer::Framebuffer;
use wasm4::gamepad::Gamepad;
use wasm4::gamepad::GamepadButton::{DPadDown, DPadLeft, DPadUp};
use wasm4::geometry::{Point, Rect, Size};
use crate::renderable::Renderable;
use crate::updatable::Updatable;
use crate::ui::text::Text;
use crate::ui::text::TextAlignment::Center;

pub struct MainMenu {
    pub items: [Text; 3],
    pub selected_index: usize,
    gamepad: &'static Gamepad,
}

impl MainMenu {
    pub fn new(gamepad: &'static Gamepad) -> Self {
        Self {
            items: [
                Self::make_menu_item("New game"),
                Self::make_menu_item("Settings"),
                Self::make_menu_item("Credits"),
            ],
            selected_index: 0,
            gamepad,
        }
    }

    fn make_menu_item(title: &str) -> Text {
        let mut text = Text::new(title.to_string());
        text.alignment = Center;
        text
    }
}

impl Updatable for MainMenu {
    fn update(&mut self) {
        let mut selected_index = self.selected_index as isize;
        if self.gamepad.is_pressed(DPadUp) {
            selected_index -= 1;
        }
        else if self.gamepad.is_pressed(DPadDown) {
            selected_index += 1;
        }
        selected_index = selected_index.clamp(0, self.items.len() as isize - 1);
        self.selected_index = selected_index as usize;
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