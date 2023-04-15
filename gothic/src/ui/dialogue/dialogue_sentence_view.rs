use alloc::format;
use alloc::string::ToString;

use wasm4::framebuffer::{DrawColorIndex, PaletteIndex};
use wasm4::geometry::Point;
use wasm4::get_char_size;
use crate::dialogue::{DialogueItem, Sentence};

use crate::renderable::{Renderable, RenderContext};
use crate::ui::dialogue::dialogue_overlay::DialogueItemView;
use crate::ui::text::{Text, TextWrapping};
use crate::updatable::{Updatable, UpdateContext};

pub struct DialogueSentenceView {
    sentence: &'static Sentence,
    message_text: Text,
}

impl DialogueSentenceView {
    pub fn new(sentence: &'static Sentence) -> Self {
        let mut message_text = Text::new(sentence.message.to_string());
        message_text.wrapping = TextWrapping::Words;
        Self { sentence, message_text }
    }
}

impl DialogueItemView for DialogueSentenceView {
    fn next_item(&self) -> Option<&'static DialogueItem> {
        self.sentence.next_item
    }
}

impl Updatable for DialogueSentenceView {
    fn update(&mut self, _context: &mut UpdateContext) {}
}

impl Renderable for DialogueSentenceView {
    fn render(&self, context: &mut RenderContext) {
        if let Some(actor) = self.sentence.actor {
            context.framebuffer.set_draw_color(DrawColorIndex::Index1, PaletteIndex::Palette3);
            context.framebuffer.set_draw_color(DrawColorIndex::Index2, PaletteIndex::Palette2);
            context.framebuffer.text(format!("{}:", actor).as_str(), context.frame.origin);
        }

        context.framebuffer.set_draw_color(DrawColorIndex::Index1, PaletteIndex::Palette3);
        context.framebuffer.set_draw_color(DrawColorIndex::Index2, PaletteIndex::Palette1);
        let mut message_frame = context.with_frame(context.frame + Point::new(0, get_char_size().height as i32));
        self.message_text.render(&mut message_frame);
    }
}

