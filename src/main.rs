#![no_std]
#![no_main]

extern crate alloc;

mod ui;
mod renderable;
mod allocator;

use alloc::boxed::Box;
#[cfg(not(test))]
use core::panic::PanicInfo;
use wasm4::application::Application;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect};
use wasm4::main_application;
use crate::renderable::Renderable;
use crate::ui::main_menu::MainMenu;

struct GothicApplication {
    framebuffer: Framebuffer,
    root_renderable: Box<dyn Renderable>
}

impl Application for GothicApplication {
    fn start(framebuffer: Framebuffer) -> Self {
        Self {
            framebuffer,
            root_renderable: Box::new(MainMenu::new()),
        }
    }

    fn update(&mut self) {
        let frame = Rect::new(
            Point::new(0, 0),
            self.framebuffer.get_size(),
        );
        self.root_renderable.render(&self.framebuffer, frame);
    }
}

main_application! { GothicApplication }

#[panic_handler]
#[cfg(not(test))]
fn panic_handler(_panic_info: &PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable();
}
