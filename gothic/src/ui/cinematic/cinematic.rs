use crate::renderable::RenderContext;

pub struct Cinematic {
    pub(crate) screens: &'static [CinematicScreen],
}

impl Cinematic {
    pub const fn new(screens: &'static [CinematicScreen]) -> Self {
        Self { screens }
    }
}

pub struct CinematicScreen {
    pub(crate) text: &'static str,
    pub(crate) draw_art: fn(context: &RenderContext),
}

impl CinematicScreen {
    pub const fn new(title: &'static str, draw: fn(context: &RenderContext)) -> Self {
        Self { text: title, draw_art: draw }
    }
}