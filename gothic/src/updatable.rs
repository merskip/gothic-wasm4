use wasm4::inputs::Inputs;
use crate::dispatcher::Dispatcher;

pub trait Updatable {
    fn update(&mut self, inputs: &Inputs, dispatcher: &mut Dispatcher);
}