use crate::dispatcher::Dispatcher;

pub trait Updatable {
    fn update(&mut self, dispatcher: &mut Dispatcher);
}