use wasm4::inputs::Inputs;

use crate::audio::music::Music;
use crate::dispatcher::Dispatcher;
use crate::ui::navigator::Navigator;

pub struct UpdateContext<'a> {
    pub dispatcher: &'a Dispatcher,
    pub navigator: &'a mut Navigator,
    pub inputs: &'a Inputs,
    pub music: &'a mut Music,
}

impl<'a> UpdateContext<'a> {
    pub fn new(dispatcher: &'a Dispatcher,
               navigator: &'a mut Navigator,
               inputs: &'a Inputs,
               music: &'a mut Music) -> Self {
        Self { dispatcher, inputs, navigator, music }
    }
}
