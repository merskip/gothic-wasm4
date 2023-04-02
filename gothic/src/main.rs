#![no_std]
#![no_main]

extern crate alloc;

mod allocator;

pub mod ui;
pub mod game;
pub mod renderable;
pub mod updatable;
pub mod dispatcher;

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
                        navigator_1.clone().borrow_mut()
                            .push_view(Rc::new(RefCell::new(
                                Self::make_new_game_menu(navigator.clone())
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

    fn make_new_game_menu(navigator: Rc<RefCell<Navigator>>) -> SimpleMenu {
        SimpleMenu::new(
            Box::new([
                "Continue",
                "Start New Game",
                "Load Game"
            ]),
            Rc::new(move |item| {
                println!("[New Game menu] Selected item index: {}", item);
            }),
            Rc::new(move || {
                navigator.borrow_mut()
                    .pop_top_view();
            }),
        )
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
