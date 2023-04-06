use alloc::boxed::Box;
use alloc::rc::Rc;
use wasm4::framebuffer::{DrawColorIndex, Framebuffer};
use wasm4::framebuffer::PaletteIndex::Palette4;
use wasm4::gamepad::GamepadButton::ButtonX;
use wasm4::geometry::{Point, Rect};
use wasm4::inputs::Inputs;
use crate::dispatcher::Dispatcher;
use crate::renderable::Renderable;
use crate::ui::cinematic::cinematic::Cinematic;
use crate::ui::cinematic::cinematic_screen::CinematicScreen;
use crate::updatable::Updatable;

pub struct CinematicPlayer {
    cinematic: &'static Cinematic,
    current_screen_index: usize,
    finish_handler: Rc<dyn Fn()>,
}

impl CinematicPlayer {
    pub fn new(cinematic: &'static Cinematic, finish_handler: Rc<dyn Fn()>) -> Self {
        Self {
            cinematic,
            current_screen_index: 0,
            finish_handler
        }
    }

    fn get_current_screen(&self) -> &'static CinematicScreen {
        &self.cinematic.screens[self.current_screen_index]
    }

    fn show_next_screen(&mut self) {
        self.current_screen_index = (self.current_screen_index + 1)
            .clamp(0, self.cinematic.screens.len() - 1);
    }

    fn is_last_screen(&self) -> bool {
        self.current_screen_index == self.cinematic.screens.len() - 1
    }
}

impl Updatable for CinematicPlayer {
    fn update(&mut self, inputs: &Inputs, dispatcher: &mut Dispatcher) {
        if inputs.gamepad1.is_released(ButtonX) {
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
        self.get_current_screen().render(framebuffer, frame);

        framebuffer.set_draw_color(DrawColorIndex::Index1, Palette4);
        framebuffer.text("Press X to continue", Point::new(5, 150));
    }
}