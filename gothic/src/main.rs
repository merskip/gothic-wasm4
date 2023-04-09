#![no_std]
#![no_main]

extern crate alloc;

mod allocator;

pub mod ui;
pub mod game;
pub mod renderable;
pub mod updatable;
pub mod dispatcher;
pub mod sprites;

use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::{RefCell};
#[cfg(not(test))]
use core::panic::PanicInfo;
use wasm4::application::Application;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect};
use wasm4::{main_application, println};
use renderable::Renderable;
use wasm4::inputs::Inputs;
use crate::dispatcher::Dispatcher;
use crate::game::cinematic_intro::CINEMATIC_INTRO;
use crate::game::game_scene::GameScene;
use crate::game::game_world::GameWorld;
use crate::game::player::Player;
use crate::ui::cinematic::cinematic_player::CinematicPlayer;
use crate::ui::navigator::Navigator;
use crate::ui::simple_menu::SimpleMenu;

struct GothicApplication {
    dispatcher: Dispatcher,
    root_renderable: Rc<RefCell<dyn Renderable>>,
}

impl Application for GothicApplication {
    fn start() -> Self {
        let navigator = Rc::new(RefCell::new(
            Navigator::new()
        ));
        navigator.borrow_mut()
            .push_view(Rc::new(RefCell::new(
                Self::make_main_menu(navigator.clone())
            )));

        Self {
            dispatcher: Dispatcher::new(),
            root_renderable: navigator,
        }
    }

    fn update(&mut self, inputs: &Inputs) {
        self.root_renderable.borrow_mut()
            .update(inputs, &mut self.dispatcher);
        self.dispatcher.execute();
    }

    fn render(&self, framebuffer: &Framebuffer) {
        let frame = Rect::new(
            Point::new(0, 0),
            framebuffer.get_size(),
        );
        self.root_renderable.borrow()
            .render(&framebuffer, frame);
    }
}

impl GothicApplication {
    fn make_main_menu(navigator: Rc<RefCell<Navigator>>) -> SimpleMenu {
        let navigator_1 = navigator.clone();
        let navigator_2 = navigator.clone();
        SimpleMenu::new(
            Box::new([
                "New game",
                "Settings",
                "Authors"
            ]),
            Rc::new(move |item| {
                println!("[Main menu] Selected item index: {}", item);

                match item {
                    0 => {
                        let navigator = navigator_1.clone();
                        navigator.borrow_mut()
                            .push_view(Rc::new(RefCell::new(
                                Self::make_cinematic_intro(navigator.clone())
                            )));
                    }
                    _ => {}
                }
            }),
            Rc::new(move || {
                navigator_2.clone().borrow_mut()
                    .pop_top_view();
            }),
        )
    }

    fn make_cinematic_intro(navigator: Rc<RefCell<Navigator>>) -> CinematicPlayer {
        CinematicPlayer::new(&CINEMATIC_INTRO, Rc::new(move || {
            let navigator = navigator.clone();
            navigator.borrow_mut().push_view(Rc::new(RefCell::new(
                Self::make_game_scene(navigator.clone()))
            ));
        }))
    }

    fn make_game_scene(_navigator: Rc<RefCell<Navigator>>) -> GameScene {
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
