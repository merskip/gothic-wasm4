use wasm4::inputs::Inputs;

use crate::dispatcher::Dispatcher;
use crate::ui::navigator::Navigator;

pub struct UpdateContext<'a> {
    pub dispatcher: &'a Dispatcher,
    pub inputs: &'a Inputs,
    pub navigator: &'a mut Navigator,
}

impl<'a> UpdateContext<'a> {
    pub fn new(dispatcher: &'a Dispatcher, inputs: &'a Inputs, navigator: &'a mut Navigator) -> Self {
        Self { dispatcher, inputs, navigator }
    }
}
