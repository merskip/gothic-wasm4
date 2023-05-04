use alloc::string::ToString;

use crate::dialogue::{DialogueItem, Sentence};

use crate::renderable::{Renderable, RenderContext, TextWrapping};
use crate::renderable::Color::{Background, Primary, Secondary};
use crate::ui::dialogue::dialogue_overlay::DialogueItemView;
use crate::ui::geometry::Point;
use crate::ui::text::Text;
use crate::updatable::{Updatable, UpdateContext};

pub struct DialogueSentenceView {
    sentence: &'static Sentence,
    actor_text: Text,
    message_text: Text,
    finished: bool
}

impl DialogueSentenceView {
    pub fn new(sentence: &'static Sentence) -> Self {
        let actor_text = Text::new(sentence.actor.unwrap_or("").to_string());

        let mut message_text = Text::new(sentence.message.to_string());
        message_text.wrapping = TextWrapping::Words;
        Self { sentence, actor_text, message_text, finished: false }
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
        context.canvas.set_text_color(Secondary, Primary);
        self.actor_text.render(context);

        let actor_text_size = self.actor_text.size(context.frame.size, context.canvas);
        let mut message_frame = context.with_frame(context.frame + Point::new(0, actor_text_size.height as i32));
        context.canvas.set_text_color(Secondary, Background);
        self.message_text.render(&mut message_frame);
    }
}

