#![no_std]
#![no_main]

extern crate alloc;

#[cfg(not(test))]
use core::panic::PanicInfo;
use gothic::audio::music::Music;
use gothic::dispatcher::Dispatcher;
use gothic::game::main_menu::make_main_menu;
use gothic::music_clips;
use gothic::renderable::RenderContext;
use gothic::ui::navigator::Navigator;
use gothic::updatable::UpdateContext;

use wasm4::application::Application;
use wasm4::audio::Audio;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect};
use wasm4::inputs::Inputs;
use wasm4::{main_application};

struct GothicApplication {
    dispatcher: Dispatcher,
    navigator: Navigator,
    music: Music,
}

impl Application for GothicApplication {
    fn start() -> Self {
        let mut navigator = Navigator::new();
        navigator.push_view(make_main_menu());

        let mut music = Music::new(Audio::shared());
        music.play_clip(&music_clips::MAIN_THEME);

        Self {
            dispatcher: Dispatcher::new(),
            navigator,
            music,
        }
    }

    fn update(&mut self, inputs: &Inputs) {
        let mut context = UpdateContext::new(
            &mut self.dispatcher,
            &mut self.navigator,
            inputs,
            &mut self.music,
        );

        if let Some(mut top_view) = context.navigator.top_view_mut() {
            top_view.update(&mut context);
            context.navigator.push_view_box(top_view);
        }
        context.music.update();
        context.dispatcher.execute(&mut context);
    }

    fn render(&self, framebuffer: &Framebuffer) {
        let frame = Rect::new(
            Point::new(0, 0),
            framebuffer.get_size(),
        );
        if let Some(top_view) = self.navigator.top_view() {
            let mut context = RenderContext::new(framebuffer, frame);
            top_view.render(&mut context);
        }
    }
}

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
