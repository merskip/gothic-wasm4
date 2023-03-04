#![no_std]
#![no_main]

extern crate alloc;

mod ui;
mod renderable;
mod allocator;
mod updatable;

use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
#[cfg(not(test))]
use core::panic::PanicInfo;
use wasm4::application::Application;
use wasm4::framebuffer::Framebuffer;
use wasm4::gamepad::Gamepad;
use wasm4::geometry::{Point, Rect};
use wasm4::main_application;
use crate::renderable::Renderable;
use crate::ui::main_menu::MainMenu;
use crate::updatable::Updatable;

struct GothicApplication {
    framebuffer: Framebuffer,
    gamepad: Rc<RefCell<Gamepad>>,
    objects: Vec<Rc<RefCell<dyn Updatable>>>,
    root_renderable: Rc<RefCell<dyn Renderable>>
}

impl Application for GothicApplication {
    fn start(framebuffer: Framebuffer) -> Self {
        let gamepad = Rc::new(RefCell::new(Gamepad::gamepad1()));
        let main_menu = Rc::new(RefCell::new(
            MainMenu::new(gamepad.clone())
        ));
        Self {
            framebuffer,
            gamepad: gamepad.clone(),
            objects: vec![main_menu.clone()],
            root_renderable: main_menu.clone(),
        }
    }

    fn update(&mut self) {
        self.update_phase();
        self.render_phase();
        self.late_update_phase();
    }
}

impl GothicApplication {
    fn update_phase(&mut self) {
        for object in &self.objects {
            object.borrow_mut().update();
        }
    }

    fn render_phase(&self) {
        let frame = Rect::new(
            Point::new(0, 0),
            self.framebuffer.get_size(),
        );
        self.root_renderable.borrow().render(&self.framebuffer, frame);
    }

    fn late_update_phase(&self) {
        self.gamepad.borrow_mut().late_update();
    }
}

main_application! { GothicApplication }

#[panic_handler]
#[cfg(not(test))]
fn panic_handler(_panic_info: &PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable();
}
