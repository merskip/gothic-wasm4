use crate::context::UpdateContext;

pub trait Updatable {
    fn update(&mut self, context: &mut UpdateContext);
}