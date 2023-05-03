use alloc::format;
use alloc::string::ToString;

use crate::dialogue::{DialogueItem, Sentence};

use crate::renderable::{Renderable, RenderContext};
use crate::renderable::Color::{Background, Primary, Secondary};
use crate::ui::dialogue::dialogue_overlay::DialogueItemView;
use crate::ui::geometry::Point;
use crate::ui::text::{Text, TextWrapping};
use crate::updatable::{Updatable, UpdateContext};

pub struct DialogueSentenceView {
    sentence: &'static Sentence,
    message_text: Text,
    finished: bool
}

impl DialogueSentenceView {
    pub fn new(sentence: &'static Sentence) -> Self {
        let mut message_text = Text::new(sentence.message.to_string());
        message_text.wrapping = TextWrapping::Words;
        Self { sentence, message_text, finished: false }
    }
}

impl DialogueItemView for DialogueSentenceView {
    fn finished(&self) -> bool {
        self.finished
    }

    fn next_item(&self) -> Option<&'static DialogueItem> {
        self.sentence.next_item
    }
}

impl Updatable for DialogueSentenceView {
    fn update(&mut self, context: &mut UpdateContext) {
        if context.controls.button_x().is_just_released() {
            self.finished = true;
        }
    }
}

impl Renderable for DialogueSentenceView {
    fn render(&self, context: &mut RenderContext) {
        if let Some(actor) = self.sentence.actor {
            context.canvas.set_text_color(Secondary, Primary);
            context.canvas.draw_text(format!("{}:", actor).as_str(), context.frame.origin);
        }

        context.canvas.set_text_color(Secondary, Background);
        let mut message_frame = context.with_frame(context.frame + Point::new(0, context.canvas.get_text_metrics(&self.message_text.text).line_height as i32));
        self.message_text.render(&mut message_frame);
    }
}

