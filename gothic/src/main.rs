#![no_std]
#![no_main]

extern crate alloc;

use core::fmt::{Display, Formatter};
#[cfg(not(test))]
use core::panic::PanicInfo;

use wasm4::{main_application, println};
use wasm4::application::Application;
use wasm4::audio::Audio;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect};
use wasm4::inputs::Inputs;

use crate::audio::music::Music;
use crate::dispatcher::Dispatcher;
use crate::game::cinematic_intro::CINEMATIC_INTRO;
use crate::game::game_scene::GameScene;
use crate::game::game_world::GameWorld;
use crate::game::player::Player;
use crate::renderable::RenderContext;
use crate::ui::cinematic::cinematic_player::CinematicPlayer;
use crate::ui::navigator::Navigator;
use crate::ui::simple_menu::SimpleMenu;
use crate::updatable::UpdateContext;

mod allocator;

pub mod ui;
pub mod game;
pub mod renderable;
pub mod updatable;
pub mod dispatcher;
pub mod sprites;
pub mod audio;
pub mod music_clips;

struct GothicApplication {
    dispatcher: Dispatcher,
    navigator: Navigator,
    music: Music,
}

impl Application for GothicApplication {
    fn start() -> Self {
        let mut navigator = Navigator::new();
        navigator.push_view(Self::make_main_menu());

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

impl GothicApplication {
    fn make_main_menu() -> SimpleMenu<MainMenuItem> {
        SimpleMenu::new(
            &[
                MainMenuItem::NewGame,
                MainMenuItem::Settings,
                MainMenuItem::Authors,
            ],
            |item, context| {
                println!("[Main menu] Selected item: {}", item);
                match item {
                    MainMenuItem::NewGame => {
                        context.music.stop();
                        context.navigator.push_view(Self::make_cinematic_intro());
                    }
                    MainMenuItem::Settings => {
                        println!("Settings not implemented yet");
                    }
                    MainMenuItem::Authors => {
                        println!("Authors not implemented yet");
                    }
                }
            },
        )
    }

    fn make_cinematic_intro() -> CinematicPlayer {
        CinematicPlayer::new(
            &CINEMATIC_INTRO,
            |context| {
                context.navigator.push_view(Self::make_game_scene());
            },
        )
    }

    fn make_game_scene() -> GameScene {
        let player = Player::new(Point::new(50.0, 50.0));
        let game_world = GameWorld::new(player);
        GameScene::new(game_world)
    }
}

#[derive(Clone, Eq, PartialEq)]
enum MainMenuItem {
    NewGame,
    Settings,
    Authors,
}

impl Display for MainMenuItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            MainMenuItem::NewGame => f.write_str("Nowa gra"),
            MainMenuItem::Settings => f.write_str("Ustawienia"),
            MainMenuItem::Authors => f.write_str("Autorzy"),
        }
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
