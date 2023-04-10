use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::renderable::Renderable;

pub struct Navigator {
    views: Vec<Box<dyn Renderable>>,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            views: Vec::new(),
        }
    }

    pub fn push_view<T>(&mut self, view: T)
        where T: Renderable + 'static {
        self.views.push(Box::new(view));
    }

    pub fn push_view_box(&mut self, view: Box<dyn Renderable>) {
        self.views.push(view);
    }

    pub fn pop_top_view(&mut self) {
        if self.views.len() > 1 {
            self.views.pop();
        }
    }

    pub fn top_view(&self) -> Option<&Box<dyn Renderable>> {
        self.views.last()
    }

    pub fn top_view_mut(&mut self) -> Option<Box<dyn Renderable>> {
        self.views.pop()
    }
}
