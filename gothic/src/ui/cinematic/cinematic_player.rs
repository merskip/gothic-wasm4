use alloc::format;
use alloc::rc::Rc;

use crate::renderable::{Renderable, RenderContext, TextAlignment, TextWrapping};
use crate::ui::cinematic::cinematic::Cinematic;
use crate::ui::cinematic::cinematic_screen::CinematicScreenView;
use crate::updatable::{Updatable, UpdateContext};

pub struct CinematicPlayer {
    cinematic: &'static Cinematic,
    current_screen_index: usize,
    current_screen: CinematicScreenView,
    finish_handler: Rc<dyn Fn(&mut UpdateContext)>,
}

impl CinematicPlayer {
    pub fn new<FinishHandler>(
        cinematic: &'static Cinematic,
        finish_handler: FinishHandler,
    ) -> Self where FinishHandler: Fn(&mut UpdateContext) + 'static {
        Self {
            cinematic,
            current_screen: CinematicScreenView::new(&cinematic.screens[0]),
            current_screen_index: 0,
            finish_handler: Rc::new(finish_handler),
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

    fn finish(&self, context: &mut UpdateContext) {
        let finish_handler = self.finish_handler.clone();
        context.dispatcher.dispatch(move |context|
            finish_handler(context)
        );
    }
}

impl Updatable for CinematicPlayer {
    fn update(&mut self, context: &mut UpdateContext) {
        self.current_screen.update(context);

        if context.controls.button_y().is_just_released() {
            self.finish(context);
        }
        else if context.controls.button_x().is_just_released() {
            if self.is_last_screen() {
                self.finish(context);
            } else {
                self.show_next_screen();
            }
        }
    }
}

impl Renderable for CinematicPlayer {
    fn render(&self, context: &mut RenderContext) {
        context.canvas.draw_text(format!("{} pomin  {} kontynuuj", "y", "x").as_str(), context.frame.origin, context.frame.size, TextWrapping::None, TextAlignment::Center);
        self.current_screen.render(context);
    }
}