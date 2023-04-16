#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use gothic::audio::music::Music;
use gothic::dispatcher::Dispatcher;
use gothic::game::main_menu::make_main_menu;
use gothic::{GothicApplication, music_clips};
use gothic::renderable::RenderContext;
use gothic::ui::navigator::Navigator;
use gothic::updatable::UpdateContext;

use wasm4::application::Application;
use wasm4::audio::Audio;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect};
use wasm4::inputs::Inputs;
use wasm4::{main_application};

main_application! { GothicApplication }

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
