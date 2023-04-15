use crate::dialogue::{DialogueItem, Script};
use crate::renderable::{Renderable, RenderContext};
use crate::ui::dialogue::dialogue_overlay::DialogueItemView;
use crate::updatable::{Updatable, UpdateContext};

pub struct DialogueScriptView {
    script: &'static Script,
    finished: bool,
}

impl DialogueScriptView {
    pub fn new(script: &'static Script) -> Self {
        Self { script, finished: false }
    }
}

impl DialogueItemView for DialogueScriptView {
    fn finished(&self) -> bool {
        self.finished
    }

    fn next_item(&self) -> Option<&'static DialogueItem> {
        self.script.next_item
    }
}

impl Updatable for DialogueScriptView {
    fn update(&mut self, context: &mut UpdateContext) {
        self.finished = (self.script.update)(context);
    }
}


impl Renderable for DialogueScriptView {
    fn render(&self, context: &mut RenderContext) {
        (self.script.render)(context);
    }
}

