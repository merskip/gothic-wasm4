use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use core::cell::Cell;

use wasm4::framebuffer::{DrawColorIndex, Framebuffer, PaletteIndex};
use wasm4::gamepad::GamepadButton::{ButtonX, ButtonY, DPadDown, DPadUp};
use wasm4::geometry::{Point, Rect, Size};

use crate::renderable::{Renderable, RenderContext};
use crate::ui::text::Text;
use crate::ui::text::TextAlignment::Center;
use crate::updatable::{Updatable, UpdateContext};

pub struct SimpleMenu {
    texts: Box<[Text]>,
    selected_index: usize,
    last_item_indicator_y: Cell<f32>,
    selection_handler: Rc<dyn Fn(usize, &mut UpdateContext)>,
}

impl SimpleMenu {
    pub fn new<Item, SelectionHandler>(
        items: &[Item],
        selection_handler: SelectionHandler,
    ) -> Self where Item: ToString, SelectionHandler: Fn(usize, &mut UpdateContext) + 'static {
        Self {
            texts: items.iter()
                .map(|item| Self::make_menu_item(item.to_string()))
                .collect(),
            selected_index: 0,
            last_item_indicator_y: Cell::new(8.0),
            selection_handler: Rc::new(selection_handler),
        }
    }

    fn make_menu_item(title: String) -> Text {
        let mut text = Text::new(title);
        text.alignment = Center;
        text
    }
}

impl Updatable for SimpleMenu {
    fn update(&mut self, context: &mut UpdateContext) {
        let mut selected_index = self.selected_index as isize;
        if context.inputs.gamepad1.is_released(DPadUp) {
            selected_index -= 1;
        } else if context.inputs.gamepad1.is_released(DPadDown) {
            selected_index += 1;
        }
        selected_index = selected_index.clamp(0, self.texts.len() as isize - 1);
        self.selected_index = selected_index as usize;

        if context.inputs.gamepad1.is_released(ButtonX) {
            let selection_handler = self.selection_handler.clone();
            context.dispatcher.dispatch(move |context| {
                (selection_handler)(selected_index as usize, context);
            });
        }

        if context.inputs.gamepad1.is_released(ButtonY) {
            context.dispatcher.dispatch(move |context| {
                context.navigator.pop_top_view();
            });
        }
    }
}

impl Renderable for SimpleMenu {
    fn render(&self, context: &mut RenderContext) {
        self.render_menu_items(context)
    }
}

impl SimpleMenu {
    fn render_menu_items(&self, context: &mut RenderContext) {
        let mut y = 8;
        for (index, item) in self.texts.iter().enumerate() {
            let item_size = item.content_size();
            let item_frame = Rect::new(
                Point::new(context.frame.origin.x, context.frame.origin.y + y),
                Size::new(context.frame.size.width, item_size.height),
            );
            let is_selected = index == self.selected_index;
            if is_selected {
                context.framebuffer.set_draw_color(DrawColorIndex::Index1, PaletteIndex::Palette3);
                let line_y = self.animate_item_indicator_y(y);
                self.render_selected_item_indicator(context.framebuffer, context.frame, line_y - 2, item_size.height + 3);
            } else {
                context.framebuffer.set_draw_color(DrawColorIndex::Index1, PaletteIndex::Palette2);
            }
            item.render(&mut context.with_frame(item_frame));

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
