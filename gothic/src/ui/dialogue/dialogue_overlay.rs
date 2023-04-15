use alloc::boxed::Box;

use wasm4::gamepad::GamepadButton::ButtonX;

use crate::dialogue::{Dialogue, DialogueItem};
use crate::renderable::{Renderable, RenderContext};
use crate::ui::dialogue::dialogue_player_choice_view::DialoguePlayerChoiceView;
use crate::ui::dialogue::dialogue_sentence_view::DialogueSentenceView;
use crate::updatable::{Updatable, UpdateContext};

pub struct DialogueOverlay {
    dialogue: &'static Dialogue,
    item: &'static DialogueItem,
    item_view: Box<dyn DialogueItemView>,
    finished: bool,
}

pub trait DialogueItemView: Renderable {
    fn next_item(&self) -> Option<&'static DialogueItem>;
}

impl DialogueOverlay {
    pub fn new(dialogue: &'static Dialogue) -> Self {
        let item = dialogue.start_item;
        Self {
            dialogue,
            item,
            item_view: Self::get_item_view(item),
            finished: false,
        }
    }

    fn next_dialogue_item(&mut self) {
        if let Some(next_item) = self.item_view.next_item() {
            self.item = next_item;
            self.item_view = Self::get_item_view(next_item);
        } else {
            self.finished = true;
        }
    }

    fn get_item_view(item: &'static DialogueItem) -> Box<dyn DialogueItemView> {
        match item {
            DialogueItem::Sentence(sentence) =>
                Box::new(DialogueSentenceView::new(sentence)),
            DialogueItem::PlayerChoice { choices } =>
                Box::new(DialoguePlayerChoiceView::new(choices)),
        }
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
            self.next_dialogue_item();
        }
    }
}

impl Renderable for DialogueOverlay {
    fn render(&self, context: &mut RenderContext) {
        self.item_view.render(context);
    }
}
