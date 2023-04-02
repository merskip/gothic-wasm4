use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct Dispatcher {
    executes: Vec<Box<dyn Fn()>>
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            executes: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, execute: Box<dyn Fn()>) {
        self.executes.push(execute);
    }

    pub fn execute(&mut self) {
        for execute in &self.executes {
            execute();
        }
        self.executes.clear();
    }
}
