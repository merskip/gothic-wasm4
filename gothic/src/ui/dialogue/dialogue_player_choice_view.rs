use alloc::string::ToString;
use wasm4::framebuffer::{DrawColorIndex, PaletteIndex};
use wasm4::gamepad::GamepadButton;
use wasm4::geometry::Point;
use crate::dialogue::{DialogueItem, PlayerChoice};
use crate::renderable::{Renderable, RenderContext};
use crate::ui::dialogue::dialogue_overlay::DialogueItemView;
use crate::ui::text::{Text, TextWrapping};
use crate::updatable::{Updatable, UpdateContext};

pub struct DialoguePlayerChoiceView {
    choices: &'static [PlayerChoice],
    selected_index: usize,
}

impl DialoguePlayerChoiceView {
    pub fn new(choices: &'static [PlayerChoice]) -> Self {
        Self { choices, selected_index: 0 }
    }
}

impl Updatable for DialoguePlayerChoiceView {
    fn update(&mut self, context: &mut UpdateContext) {
        let mut selected_index = self.selected_index as isize;
        if context.inputs.gamepad1.is_released(GamepadButton::DPadUp) {
            selected_index -= 1;
        }
        if context.inputs.gamepad1.is_released(GamepadButton::DPadDown) {
            selected_index += 1;
        }
        self.selected_index = selected_index.clamp(0, (self.choices.len() - 1) as isize) as usize;
    }
}

impl DialogueItemView for DialoguePlayerChoiceView {
    fn next_item(&self) -> Option<&'static DialogueItem> {
        let item = &self.choices[self.selected_index];
        item.next_item
    }
}

impl Renderable for DialoguePlayerChoiceView {
    fn render(&self, context: &mut RenderContext) {
        let mut y = 0;
        for (index, choice) in self.choices.iter().enumerate() {
            let is_selected = self.selected_index == index;
            if is_selected {
                context.framebuffer.set_draw_color(DrawColorIndex::Index1, PaletteIndex::Palette3);
                context.framebuffer.set_draw_color(DrawColorIndex::Index2, PaletteIndex::Transparent);
            } else {
                context.framebuffer.set_draw_color(DrawColorIndex::Index1, PaletteIndex::Palette2);
                context.framebuffer.set_draw_color(DrawColorIndex::Index2, PaletteIndex::Transparent);
            }

            let mut text = Text::new(choice.choice.to_string());
            text.wrapping = TextWrapping::Words;

            text.render(&mut context.with_frame(context.frame + Point::new(0, y as i32)));
            y += text.content_size(context.frame.size).height;
        }
    }
}
