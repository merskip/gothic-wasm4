use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::context::UpdateContext;

pub struct Dispatcher {
    executes: RefCell<Vec<Box<dyn FnOnce(&mut UpdateContext)>>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            executes: RefCell::new(Vec::new()),
        }
    }

    pub fn dispatch<T: FnOnce(&mut UpdateContext) + 'static>(&self, execute: T) {
        self.executes.borrow_mut().push(Box::new(execute));
    }

    pub fn execute(&self, context: &mut UpdateContext) {
        let executes = self.executes.take();
        for execute in executes {
            execute(context);
        }
    }
}
