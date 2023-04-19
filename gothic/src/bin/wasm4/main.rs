#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use core::str::from_utf8_unchecked;

use gothic::GothicApplication;
use wasm4::application::Application;
use wasm4::framebuffer::Framebuffer;
use wasm4::inputs::Inputs;
use wasm4::main_application;
use crate::wasm4_canvas::Wasm4Canvas;

mod wasm4_canvas;
mod allocator;
mod sprites;


struct ApplicationWrapper {
    application: GothicApplication,
}

impl Application for ApplicationWrapper {
    fn start() -> Self {
        ApplicationWrapper { application: GothicApplication::start() }
    }

    fn update(&mut self, _: &Inputs) {

        // TODO::
    }
    fn render(&self, framebuffer: &Framebuffer) {
        let canvas = Wasm4Canvas::new(framebuffer);
        self.application.render(&canvas);
    }
}

main_application! { ApplicationWrapper }

#[panic_handler]
#[cfg(not(test))]
#[allow(unused_variables)]
fn panic_handler(panic_info: &PanicInfo<'_>) -> ! {
    if cfg!(debug_assertions) {
        use wasm4::println;
        println!("[FATAL ERROR]");
        println!("{}", panic_info);
    }
    core::arch::wasm32::unreachable()
}
