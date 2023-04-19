#![no_std]
#![no_main]

extern crate alloc;

use core::any::Any;
use core::panic::PanicInfo;

use gothic::{GothicApplication, music_clips};
use gothic::audio::music::Music;
use gothic::dispatcher::Dispatcher;
use gothic::game::main_menu::make_main_menu;
use gothic::renderable::{Canvas, Color, Image, RenderContext};
use gothic::ui::geometry::{Point, Size};
use gothic::ui::navigator::Navigator;
use gothic::updatable::UpdateContext;
use wasm4::main_application;
use wasm4::application::Application;
use wasm4::audio::Audio;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect};
use wasm4::inputs::Inputs;
use wasm4::sprite::Sprite;

mod sprites;
mod allocator;

main_application! { GothicApplication }

struct Wasm4Canvas {
    framebuffer: &'static Framebuffer,
}

impl Canvas for Wasm4Canvas {

    fn get_size(&self) -> Size {
        Size::new(self.framebuffer.get_screen_width(),
                  self.framebuffer.get_screen_height())
    }

    fn get_char_size(&self) -> Size {
        Size::new(
            get_char_width(),
            get_char_height(),
        )
    }

    fn draw_line(&self, start: Point, end: Point) {
        self.framebuffer.line(start.x, start.y, end.x, end.y);
    }

    fn set_rectangle_color(&self, fill_color: Color, border: Color) {
        // TODO
    }

    fn draw_rectangle(&self, start: Point, size: Size) {
        self.framebuffer.rectangle(start.x, start.y, size.width, size.height);
    }

    fn set_text_color(&self, foreground: Color, background: Color) {
        // TODO
    }

    fn draw_text(&self, text: &str, start: Point) {
        self.framebuffer.text(text, start.x, start.y);
    }

    fn set_image_colors(&self, colors: [Color; 4]) {
        // TODO
    }

    fn draw_image(&self, image: &dyn Image, start: Point) {
        let sprite = (image as &dyn Any).downcast_ref::<Sprite>().unwrap();
        // TODO
    }
}

impl Image for Sprite {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

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
