use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::Rect;
use crate::dispatcher::Dispatcher;
use crate::renderable::Renderable;
use crate::updatable::Updatable;

pub struct Navigator {
    views: Vec<Rc<RefCell<dyn Renderable>>>,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            views: Vec::new()
        }
    }

    pub fn push_view(&mut self, view: Rc<RefCell<dyn Renderable>>) {
        self.views.push(view.clone());
    }

    pub fn pop_top_view(&mut self) {
        if self.views.len() > 1 {
            self.views.pop();
        }
    }

    pub fn get_top_view(&self) -> Option<Rc<RefCell<dyn Renderable>>> {
        self.views.last()
            .map(|top_view| top_view.clone())
    }
}

impl Updatable for Navigator {
    fn update(&mut self, dispatcher: &mut Dispatcher) {
        if let Some(view) = self.get_top_view() {
            view.borrow_mut().update(dispatcher);
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