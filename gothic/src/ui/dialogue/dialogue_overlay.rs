use alloc::boxed::Box;

use wasm4::gamepad::GamepadButton::ButtonX;

use crate::renderable::{Renderable, RenderContext};
use crate::ui::dialogue::dialogue::{Dialogue, DialogueItem};
use crate::ui::dialogue::dialogue_sentence_view::DialogueSentenceView;
use crate::updatable::{Updatable, UpdateContext};

pub struct DialogueOverlay {
    dialogue: &'static Dialogue,
    item_view: Box<dyn Renderable>,
    current_item_index: usize,
    finished: bool,
}

impl DialogueOverlay {
    pub fn new(dialogue: &'static Dialogue) -> Self {
        Self {
            dialogue,
            item_view: Self::get_item_view(dialogue, 0),
            current_item_index: 0,
            finished: false,
        }
    }

    fn show_next_sentence(&mut self) {
        if self.current_item_index + 1 < self.dialogue.items.len() {
            self.current_item_index += 1;
            self.item_view = Self::get_item_view(self.dialogue, self.current_item_index);
        } else {
            self.finished = true;
        }
    }

    fn get_item_view(dialogue: &'static Dialogue, index: usize) -> Box<dyn Renderable> {
        let item = &dialogue.items[index];
        Box::new(match item {
            DialogueItem::Sentence(sentence) => DialogueSentenceView::new(sentence),
            DialogueItem::PlayerChoice { .. } => todo!("Not implemented yet"),
        })
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn dialogue(&self) -> &'static Dialogue {
        self.dialogue
    }
}

impl Updatable for DialogueOverlay {
    fn update(&mut self, context: &mut UpdateContext) {
        self.item_view.update(context);

        if context.inputs.gamepad1.is_released(ButtonX) {
            self.show_next_sentence();
        }
    }
}

impl Renderable for DialogueOverlay {
    fn render(&self, context: &mut RenderContext) {
        self.item_view.render(context);
    }
}
