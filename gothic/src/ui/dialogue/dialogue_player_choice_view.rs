use alloc::string::ToString;
use alloc::vec::Vec;

use crate::dialogue::{DialogueItem, PlayerChoice};
use crate::renderable::{Renderable, RenderContext};
use crate::renderable::Color::{Primary, Secondary, Transparent};
use crate::ui::dialogue::dialogue_overlay::DialogueItemView;
use crate::ui::geometry::Point;
use crate::ui::text::{Text, TextWrapping};
use crate::updatable::{Updatable, UpdateContext};

pub struct DialoguePlayerChoiceView {
    choices: Vec<&'static PlayerChoice>,
    selected_index: usize,
    finished: bool,
}

impl DialoguePlayerChoiceView {
    pub fn new(choices: &'static [&'static PlayerChoice]) -> Self {
        Self {
            choices: choices.into_iter()
                .cloned()
                .filter(|choice| choice.enabled)
                .collect(),
            selected_index: 0,
            finished: false,
        }
    }
}

impl Updatable for DialoguePlayerChoiceView {
    fn update(&mut self, context: &mut UpdateContext) {
        let mut selected_index = self.selected_index as isize;
        if context.controls.arrow_top().is_just_released() {
            selected_index -= 1;
        }
        if context.controls.arrow_down().is_just_released() {
            selected_index += 1;
        }
        self.selected_index = selected_index.clamp(0, (self.choices.len() - 1) as isize) as usize;

        if context.controls.button_x().is_just_released() {
            self.finished = true;
        }
    }
}

impl DialogueItemView for DialoguePlayerChoiceView {
    fn finished(&self) -> bool {
        self.finished
    }

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
                context.canvas.set_text_color(Secondary, Transparent);
            } else {
                context.canvas.set_text_color(Primary, Transparent);
            }

            let mut text = Text::new(choice.choice.to_string());
            text.wrapping = TextWrapping::Words;

            text.render(&mut context.with_frame(context.frame + Point::new(0, y as i32)));
            y += text.content_size(context.frame.size, context.canvas).height;
        }
    }
}
