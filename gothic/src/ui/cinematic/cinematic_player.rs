use alloc::boxed::Box;
use alloc::rc::Rc;
use wasm4::framebuffer::{Framebuffer};
use wasm4::gamepad::GamepadButton::ButtonX;
use wasm4::geometry::{Rect};
use wasm4::inputs::Inputs;
use crate::dispatcher::Dispatcher;
use crate::renderable::Renderable;
use crate::ui::cinematic::cinematic::Cinematic;
use crate::ui::cinematic::cinematic_screen::{CinematicScreenView};
use crate::updatable::Updatable;

pub struct CinematicPlayer {
    cinematic: &'static Cinematic,
    current_screen_index: usize,
    current_screen: CinematicScreenView,
    finish_handler: Rc<dyn Fn()>,
}

impl CinematicPlayer {
    pub fn new(cinematic: &'static Cinematic, finish_handler: Rc<dyn Fn()>) -> Self {
        Self {
            cinematic,
            current_screen: CinematicScreenView::new(&cinematic.screens[0]),
            current_screen_index: 0,
            finish_handler,
        }
    }

    fn show_next_screen(&mut self) {
        self.current_screen_index = (self.current_screen_index + 1)
            .clamp(0, self.cinematic.screens.len() - 1);
        self.current_screen = CinematicScreenView::new(&self.cinematic.screens[self.current_screen_index]);
    }

    fn is_last_screen(&self) -> bool {
        self.current_screen_index == self.cinematic.screens.len() - 1
    }
}

impl Updatable for CinematicPlayer {
    fn update(&mut self, inputs: &Inputs, dispatcher: &mut Dispatcher) {
        self.current_screen.update(inputs, dispatcher);

        if self.current_screen.is_finished()
            && inputs.gamepad1.is_released(ButtonX) {
            if self.is_last_screen() {
                let finish_handler = self.finish_handler.clone();
                dispatcher.dispatch(Box::new(move ||
                    finish_handler()
                ));
            } else {
                self.show_next_screen();
            }
        }
    }
}

impl Renderable for CinematicPlayer {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect) {
        self.current_screen.render(framebuffer, frame);
    }
}