#![no_std]
#![no_main]

extern crate alloc;

mod ui;
mod allocator;
pub mod renderable;
pub mod updatable;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::{RefCell};
#[cfg(not(test))]
use core::panic::PanicInfo;
use wasm4::application::Application;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect};
use wasm4::{main_application, trace};
use renderable::Renderable;
use updatable::Updatable;
use wasm4::inputs::Inputs;
use crate::ui::simple_menu::SimpleMenu;

struct GothicApplication {
    objects: Vec<Rc<RefCell<dyn Updatable>>>,
    root_renderable: Rc<RefCell<dyn Renderable>>
}

impl Application for GothicApplication {
    fn start(inputs: &'static Inputs) -> Self {
        let main_menu = Rc::new(RefCell::new(
            SimpleMenu::new(
                Box::new([
                    "New game",
                    "Settings",
                    "Authors"
                ]),
                &inputs.gamepad1,
                Box::new(|item| {
                    trace("Selected item");
                    trace(&*item.to_string())
                }),
            )
        ));
        Self {
            objects: vec![main_menu.clone()],
            root_renderable: main_menu.clone(),
        }
    }

    fn update(&mut self) {
        for object in &self.objects {
            object.borrow_mut().update();
        }
    }

    fn render(&self, framebuffer: &Framebuffer) {
        let frame = Rect::new(
            Point::new(0, 0),
            framebuffer.get_size(),
        );
        self.root_renderable.borrow().render(&framebuffer, frame);
    }
}

main_application! { GothicApplication }

#[panic_handler]
#[cfg(not(test))]
fn panic_handler(_panic_info: &PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable();
}
