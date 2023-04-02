#![no_std]
#![no_main]

extern crate alloc;

mod ui;
mod allocator;
pub mod renderable;
pub mod updatable;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::ToString;
use core::cell::{RefCell};
#[cfg(not(test))]
use core::panic::PanicInfo;
use wasm4::application::Application;
use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect};
use wasm4::{main_application, println, trace};
use renderable::Renderable;
use wasm4::inputs::Inputs;
use crate::ui::navigator::Navigator;
use crate::ui::simple_menu::SimpleMenu;

struct GothicApplication {
    root_renderable: Rc<RefCell<dyn Renderable>>,
}

impl GothicApplication {
    fn make_main_menu(inputs: &'static Inputs, navigator: Rc<RefCell<Navigator>>) -> SimpleMenu {
        SimpleMenu::new(
            Box::new([
                "New game",
                "Settings",
                "Authors"
            ]),
            &inputs.gamepad1,
            Box::new(move |item| {
                trace("[Main menu] Selected item");
                trace(&*item.to_string());

                navigator.borrow_mut()
                    .push_view(Rc::new(RefCell::new(
                        Self::make_new_game_menu(inputs, navigator.clone())
                    )));
            }),
        )
    }

    fn make_new_game_menu(inputs: &'static Inputs, navigator: Rc<RefCell<Navigator>>) -> SimpleMenu {
        SimpleMenu::new(
            Box::new([
                "Continue",
                "Start New Game",
                "Load Game"
            ]),
            &inputs.gamepad1,
            Box::new(move |item| {
                trace("[New Game menu] Selected item");
                trace(&*item.to_string());

                navigator.borrow_mut()
                    .pop_top_view();
            }),
        )
    }
}

impl Application for GothicApplication {
    fn start(inputs: &'static Inputs) -> Self {
        let navigator = Rc::new(RefCell::new(
            Navigator::new()
        ));
        navigator.borrow_mut()
            .push_view(Rc::new(RefCell::new(
                Self::make_main_menu(inputs, navigator.clone())
            )));

        Self {
            root_renderable: navigator,
        }
    }

    fn update(&mut self) {
        let mut root_renderable = self.root_renderable.borrow_mut();
        root_renderable.update();
    }

    fn render(&self, framebuffer: &Framebuffer) {
        let frame = Rect::new(
            Point::new(0, 0),
            framebuffer.get_size(),
        );
        let root_renderable = self.root_renderable.borrow();
        root_renderable.render(&framebuffer, frame);
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
