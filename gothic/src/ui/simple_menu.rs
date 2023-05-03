use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::Cell;

use crate::renderable::{Canvas, Color, Renderable, RenderContext};
use crate::renderable::TextAlignment::Center;
use crate::ui::geometry::{Point, Rect, Size};
use crate::ui::text::Text;
use crate::updatable::{Updatable, UpdateContext};

pub struct SimpleMenu<Item> where Item: ToString + Clone {
    items: Vec<Item>,
    texts: Vec<Text>,
    selected_index: usize,
    last_item_indicator_y: Cell<f32>,
    selection_handler: Rc<dyn Fn(Item, &mut UpdateContext)>,
}

impl<Item> SimpleMenu<Item> where Item: ToString + Clone {
    pub fn new<SelectionHandler>(
        items: &[Item],
        selection_handler: SelectionHandler,
    ) -> Self where SelectionHandler: Fn(Item, &mut UpdateContext) + 'static {
        Self {
            items: items.to_vec(),
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

impl<Item> Updatable for SimpleMenu<Item> where Item: ToString + Clone + 'static {
    fn update(&mut self, context: &mut UpdateContext) {
        let mut selected_index = self.selected_index as isize;
        if context.controls.arrow_top().is_just_released() {
            selected_index -= 1;
        } else if context.controls.arrow_down().is_just_released() {
            selected_index += 1;
        }
        selected_index = selected_index.clamp(0, self.texts.len() as isize - 1);
        self.selected_index = selected_index as usize;

        if context.controls.button_x().is_just_released() {
            let selection_handler = self.selection_handler.clone();
            let selected_item = self.items[selected_index as usize].clone();
            context.dispatcher.dispatch(move |context| {
                (selection_handler)(selected_item, context);
            });
        }

        if context.controls.button_y().is_just_released() {
            context.dispatcher.dispatch(move |context| {
                context.navigator.pop_top_view();
            });
        }
    }
}

impl<Item> Renderable for SimpleMenu<Item> where Item: ToString + Clone + 'static {
    fn render(&self, context: &mut RenderContext) {
        self.render_menu_items(context)
    }
}

impl<Item> SimpleMenu<Item> where Item: ToString + Clone {
    fn render_menu_items(&self, context: &mut RenderContext) {
        let mut y = 8;
        for (index, item) in self.texts.iter().enumerate() {
            let item_size = item.content_size(context.canvas, context.frame.size);
            let item_frame = Rect::new(
                Point::new(context.frame.origin.x, context.frame.origin.y + y),
                Size::new(context.frame.size.width, item_size.height),
            );
            let is_selected = index == self.selected_index;
            if is_selected {
                context.canvas.set_line_color(Color::Secondary);
                let line_y = self.animate_item_indicator_y(y);
                self.render_selected_item_indicator(context.canvas, context.frame, line_y, item_size.height + 4);
            }
            context.canvas.set_text_color(
                if is_selected { Color::Secondary } else { Color::Primary },
                Color::Transparent,
            );
            item.render(&mut context.with_frame(item_frame));

            y += item_size.height as i32 + 6;
        }
    }

    fn render_selected_item_indicator(&self, canvas: &dyn Canvas, frame: Rect, y: i32, item_height: u32) {
        canvas.draw_line(
            Point::new(frame.origin.x, frame.origin.y + y),
            Point::new(frame.origin.x + frame.size.width as i32, frame.origin.y + y),
        );
        canvas.draw_line(
            Point::new(frame.origin.x, frame.origin.y + y + item_height as i32),
            Point::new(frame.origin.x + frame.size.width as i32, frame.origin.y + y + item_height as i32),
        );
    }

    fn animate_item_indicator_y(&self, item_y: i32) -> i32 {
        let line_y = Self::linear_interpolate(self.last_item_indicator_y.take(), item_y as f32, 0.3);
        self.last_item_indicator_y.replace(line_y);
        line_y as i32
    }

    fn linear_interpolate(a: f32, b: f32, t: f32) -> f32 {
        a * (1.0 - t) + b * t
    }
}
