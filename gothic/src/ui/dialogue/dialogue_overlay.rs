use alloc::string::ToString;
use wasm4::geometry::Point;
use wasm4::get_char_size;

use crate::renderable::{Renderable, RenderContext};
use crate::ui::dialogue::dialogue::{Dialogue, Sentence};
use crate::ui::text::{Text, TextWrapping};
use crate::updatable::{Updatable, UpdateContext};

pub struct DialogueOverlay {
    dialogue: &'static Dialogue,
    sentence_view: DialogueSentenceView,
    current_sentence_index: usize,
}

impl DialogueOverlay {
    pub fn new(dialogue: &'static Dialogue) -> Self {
        Self {
            dialogue,
            sentence_view: DialogueSentenceView::new(&dialogue.sentences[0]),
            current_sentence_index: 0,
        }
    }

    fn current_sentence(&self) -> &'static Sentence {
        &self.dialogue.sentences[self.current_sentence_index]
    }
}

impl Updatable for DialogueOverlay {
    fn update(&mut self, context: &mut UpdateContext) {
        self.sentence_view.update(context);
    }
}

impl Renderable for DialogueOverlay {
    fn render(&self, context: &mut RenderContext) {
        self.sentence_view.render(context);
    }
}

struct DialogueSentenceView {
    sentence: &'static Sentence,
    message_text: Text,
}

impl DialogueSentenceView {
    pub fn new(sentence: &'static Sentence) -> Self {
        let mut message_text = Text::new(sentence.message.to_string());
        message_text.wrapping = TextWrapping::Character;
        Self { sentence, message_text }
    }
}

impl Updatable for DialogueSentenceView {
    fn update(&mut self, _context: &mut UpdateContext) {}
}

impl Renderable for DialogueSentenceView {
    fn render(&self, context: &mut RenderContext) {
        if let Some(actor) = self.sentence.actor {
            context.framebuffer.text(actor, context.frame.origin);
        }

        self.message_text.render(context);
    }
}

