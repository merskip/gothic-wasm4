#![no_std]
#![no_main]

extern crate alloc;

use core::any::Any;
use core::panic::PanicInfo;

use gothic::GothicApplication;
use gothic::image_asset::ImageAsset;
use gothic::renderable::{Image, ImageProvider};
use gothic::ui::geometry::Size;
use wasm4::application::Application;
use wasm4::framebuffer::Framebuffer;
use wasm4::inputs::Inputs;
use wasm4::main_application;
use wasm4::sprite::Sprite;

use crate::sprites::*;
use crate::wasm4_canvas::Wasm4Canvas;
use crate::wasm4_controls::Wasm4Controls;

mod wasm4_canvas;
mod wasm4_controls;
mod allocator;
mod sprites;

struct Wasm4ImageProvider;

impl ImageProvider for Wasm4ImageProvider {
    fn get_image(&self, asset: ImageAsset) -> &dyn Image {
        match asset {
            ImageAsset::Player => &SpriteImage(PLAYER_SPRITE),
            ImageAsset::KingRhobar2 => &SpriteImage(KING__RHOBAR_2_SPRITE),
            ImageAsset::Orc => &SpriteImage(ORC_SPRITE),
            ImageAsset::Crossbones => &SpriteImage(CROSSBONES_SPRITE),
        }
    }
}

struct SpriteImage(&'static Sprite);

impl Image for SpriteImage {
    fn size(&self) -> Size {
        Size::new(self.0.width, self.0.height)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct ApplicationWrapper {
    application: GothicApplication,
}

impl Application for ApplicationWrapper {
    fn start() -> Self {
        ApplicationWrapper { application: GothicApplication::start(&Wasm4ImageProvider) }
    }

    fn update(&mut self, inputs: &Inputs) {
        let controls = Wasm4Controls::new(inputs);
        self.application.update(&controls);
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
