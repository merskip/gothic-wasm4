use alloc::boxed::Box;
use alloc::string::{String, ToString};
use wasm4::framebuffer::Framebuffer;
use wasm4::gamepad::Gamepad;
use wasm4::gamepad::GamepadButton::{ButtonX, DPadDown, DPadUp};
use wasm4::geometry::{Point, Rect, Size};
use crate::renderable::Renderable;
use crate::updatable::Updatable;
use crate::ui::text::Text;
use crate::ui::text::TextAlignment::Center;

pub struct SimpleMenu {
    texts: Box<[Text]>,
    selected_index: usize,
    gamepad: &'static Gamepad,
    selection_handler: Box<dyn Fn(usize)>
}

impl SimpleMenu {
    pub fn new<T: ToString>(
        items: Box<[T]>,
        gamepad: &'static Gamepad,
        selection_handler: Box<dyn Fn(usize)>,
    ) -> Self {
        Self {
            texts: items.iter()
                .map(|item| Self::make_menu_item(item.to_string()))
                .collect(),
            selected_index: 0,
            gamepad,
            selection_handler,
        }
    }

    fn make_menu_item(title: String) -> Text {
        let mut text = Text::new(title);
        text.alignment = Center;
        text
    }
}

impl Updatable for SimpleMenu {
    fn update(&mut self) {
        let mut selected_index = self.selected_index as isize;
        if self.gamepad.is_pressed(DPadUp) {
            selected_index -= 1;
        } else if self.gamepad.is_pressed(DPadDown) {
            selected_index += 1;
        }
        selected_index = selected_index.clamp(0, self.texts.len() as isize - 1);
        self.selected_index = selected_index as usize;

        if self.gamepad.is_pressed(ButtonX) {
            (self.selection_handler)(self.selected_index);
        }
    }
}

impl Renderable for SimpleMenu {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect) {
        self.render_menu_items(framebuffer, frame)
    }
}

impl SimpleMenu {
    fn render_menu_items(&self, framebuffer: &Framebuffer, frame: Rect) {
        let mut y = 8;
        for (index, item) in self.texts.iter().enumerate() {
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