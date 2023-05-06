use crate::dispatcher::Dispatcher;
use crate::platform_context::PlatformContext;
use crate::ui::navigator::Navigator;

pub trait Updatable {
    fn update(&mut self, context: &mut UpdateContext);
}

pub struct UpdateContext<'a> {
    pub dispatcher: &'a Dispatcher,
    pub navigator: &'a mut Navigator,
    pub controls: &'a dyn Controls,
    pub platform: &'a dyn PlatformContext,
}

impl<'a> UpdateContext<'a> {
    pub fn new(
        dispatcher: &'a Dispatcher,
        navigator: &'a mut Navigator,
        controls: &'a dyn Controls,
        platform: &'a dyn PlatformContext,
    ) -> Self {
        Self { dispatcher, controls, navigator, platform }
    }
}

pub trait Controls {
    fn button_x(&self) -> &dyn Button;
    fn button_y(&self) -> &dyn Button;
    fn arrow_left(&self) -> &dyn Button;
    fn arrow_top(&self) -> &dyn Button;
    fn arrow_right(&self) -> &dyn Button;
    fn arrow_down(&self) -> &dyn Button;
}

pub trait Button {
    fn is_pressed(&self) -> bool;
    fn is_just_pressed(&self) -> bool;
    fn is_just_released(&self) -> bool;
}