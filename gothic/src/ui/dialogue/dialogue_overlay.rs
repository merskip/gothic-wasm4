use crate::renderable::{Renderable, RenderContext};
use crate::ui::dialogue::dialogue::Dialogue;
use crate::updatable::{Updatable, UpdateContext};

struct DialogueOverlay {
    dialogue: &'static Dialogue,
    current_sentence_index: usize,
}

impl DialogueOverlay {
    pub fn new(dialogue: &'static Dialogue) -> Self {
        Self {
            dialogue,
            current_sentence_index: 0,
        }
    }
}

impl Updatable for DialogueOverlay {
    fn update(&mut self, context: &mut UpdateContext) {
        todo!()
    }
}

impl Renderable for DialogueOverlay {
    fn render(&self, context: &mut RenderContext) {
        todo!()
    }
}
