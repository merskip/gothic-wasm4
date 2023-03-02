#![no_std]
#![no_main]

#[cfg(not(test))]
use core::panic::PanicInfo;
use wasm4::application::Application;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::Point;
use wasm4::main_application;

struct GothicApplication {
    framebuffer: Framebuffer
}

impl Application for GothicApplication {
    fn start(framebuffer: Framebuffer) -> Self {
        Self {
            framebuffer
        }
    }

    fn update(&mut self) {
        self.framebuffer.text("Gothic", Point::new(0, 0));
    }
}

main_application! { GothicApplication }

#[panic_handler]
#[cfg(not(test))]
fn panic_handler(_panic_info: &PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable();
}
