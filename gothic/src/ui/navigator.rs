use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::RefCell;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::Rect;
use wasm4::inputs::Inputs;
use crate::dispatcher::Dispatcher;
use crate::renderable::Renderable;
use crate::updatable::Updatable;

pub struct Navigator {
    views: Vec<Box<RefCell<dyn Renderable>>>,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            views: Vec::new()
        }
    }

    pub fn push_view(&mut self, view: impl Renderable + 'static) {
        self.views.push(Box::new(RefCell::new(view)));
    }

    pub fn pop_top_view(&mut self) {
        if self.views.len() > 1 {
            self.views.pop();
        }
    }

    pub fn get_top_view(&self) -> Option<&Box<RefCell<dyn Renderable>>> {
        self.views.last()?.into()
    }
}

impl Updatable for Navigator {
    fn update(&mut self, inputs: &Inputs, dispatcher: &mut Dispatcher) {
        if let Some(view) = self.get_top_view() {
            view.borrow_mut().update(inputs, dispatcher);
        }
    }
}

impl Renderable for Navigator {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect) {
        if let Some(view) = self.get_top_view() {
            view.borrow().render(framebuffer, frame)
        }
    }
}