#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
#[cfg(not(test))]
use core::panic::PanicInfo;

use wasm4::{main_application, println};
use wasm4::application::Application;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect};
use wasm4::inputs::Inputs;

use crate::audio::music::Music;
use crate::context::UpdateContext;
use crate::dispatcher::Dispatcher;
use crate::game::cinematic_intro::CINEMATIC_INTRO;
use crate::game::game_scene::GameScene;
use crate::game::game_world::GameWorld;
use crate::game::player::Player;
use crate::ui::cinematic::cinematic_player::CinematicPlayer;
use crate::ui::navigator::Navigator;
use crate::ui::simple_menu::SimpleMenu;
use crate::updatable::Updatable;

mod allocator;

pub mod ui;
pub mod game;
pub mod renderable;
pub mod updatable;
pub mod dispatcher;
pub mod sprites;
pub mod audio;
pub mod music_clips;
mod context;

struct GothicApplication {
    navigator: Navigator,
    dispatcher: Dispatcher,
}

impl Application for GothicApplication {
    fn start() -> Self {
        let mut navigator = Navigator::new();
        navigator.push_view(Self::make_main_menu());

        Music::shared().play_clip(&music_clips::MAIN_THEME);

        Self {
            navigator,
            dispatcher: Dispatcher::new(),
        }
    }

    fn update(&mut self, inputs: &Inputs) {
        let mut context = UpdateContext::new(&mut self.dispatcher, inputs, &mut self.navigator);

        if let Some(mut top_view) = context.navigator.top_view_mut() {
            top_view.update(&mut context);
            context.navigator.push_view_box(top_view);
        }
        Music::shared().update(&mut context);

        context.dispatcher.execute(&mut context);
    }

    fn render(&self, framebuffer: &Framebuffer) {
        let frame = Rect::new(
            Point::new(0, 0),
            framebuffer.get_size(),
        );
        if let Some(top_view) = self.navigator.top_view() {
            top_view.render(&framebuffer, frame);
        }
    }
}

impl GothicApplication {
    fn make_main_menu() -> SimpleMenu {
        SimpleMenu::new(
            Box::new([
                "New game",
                "Settings",
                "Authors"
            ]),
            Rc::new(move |item, context| {
                println!("[Main menu] Selected item index: {}", item);

                match item {
                    0 => {
                        Music::shared().stop();

                        context.navigator.push_view(Self::make_cinematic_intro());
                    }
                    _ => {}
                }
            }),
        )
    }

    fn make_cinematic_intro() -> CinematicPlayer {
        CinematicPlayer::new(&CINEMATIC_INTRO, Rc::new(move |context| {
            context.navigator.push_view(Self::make_game_scene());
        }))
    }

    fn make_game_scene() -> GameScene {
        let player = Player::new(Point::new(50.0, 50.0));
        let game_world = GameWorld::new(player);
        GameScene::new(game_world)
    }
}

main_application! { GothicApplication }

#[panic_handler]
#[cfg(not(test))]
fn panic_handler(panic_info: &PanicInfo<'_>) -> ! {
    println!();
    println!("[PANIC!]");
    println!("{}", panic_info);
    core::arch::wasm32::unreachable();
}
