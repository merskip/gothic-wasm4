use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use core::cell::Cell;
use wasm4::framebuffer::Framebuffer;
use wasm4::gamepad::Gamepad;
use wasm4::gamepad::GamepadButton::{ButtonX, ButtonY, DPadDown, DPadUp};
use wasm4::geometry::{Point, Rect, Size};
use crate::dispatcher::Dispatcher;
use crate::renderable::Renderable;
use crate::updatable::Updatable;
use crate::ui::text::Text;
use crate::ui::text::TextAlignment::Center;

pub struct SimpleMenu {
    texts: Box<[Text]>,
    selected_index: usize,
    last_item_indicator_y: Cell<f32>,
    gamepad: &'static Gamepad,
    selection_handler: Rc<dyn Fn(usize)>,
    back_handler: Rc<dyn Fn()>,
}

impl SimpleMenu {
    pub fn new<T: ToString>(
        items: Box<[T]>,
        gamepad: &'static Gamepad,
        selection_handler: Rc<dyn Fn(usize)>,
        back_handler: Rc<dyn Fn()>,
    ) -> Self {
        Self {
            texts: items.iter()
                .map(|item| Self::make_menu_item(item.to_string()))
                .collect(),
            selected_index: 0,
            last_item_indicator_y: Cell::new(8.0),
            gamepad,
            selection_handler,
            back_handler,
        }
    }

    fn make_menu_item(title: String) -> Text {
        let mut text = Text::new(title);
        text.alignment = Center;
        text
    }
}

impl Updatable for SimpleMenu {
    fn update(&mut self, dispatcher: &mut Dispatcher) {
        let mut selected_index = self.selected_index as isize;
        if self.gamepad.is_pressed(DPadUp) {
            selected_index -= 1;
        } else if self.gamepad.is_pressed(DPadDown) {
            selected_index += 1;
        }
        selected_index = selected_index.clamp(0, self.texts.len() as isize - 1);
        self.selected_index = selected_index as usize;

        if self.gamepad.is_pressed(ButtonX) {
            let selected_index = self.selected_index;
            let selection_handler = self.selection_handler.clone();

            dispatcher.dispatch(Box::new(move || {
                (selection_handler)(selected_index);
            }));
        }

        if self.gamepad.is_pressed(ButtonY) {
            let back_handler = self.back_handler.clone();
            dispatcher.dispatch(Box::new(move || {
                back_handler();
            }));
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
                let line_y = self.animate_item_indicator_y(y);
                self.render_selected_item_indicator(framebuffer, frame, line_y - 2, item_size.height + 3);

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

impl SimpleMenu {
    fn animate_item_indicator_y(&self, item_y: i32) -> i32 {
        let line_y = Self::linear_interpolate(self.last_item_indicator_y.take(), item_y as f32, 0.3);
        self.last_item_indicator_y.replace(line_y);
        line_y as i32
    }

    fn linear_interpolate(a: f32, b: f32, t: f32) -> f32 {
        a * (1.0 - t) + b * t
    }
}
