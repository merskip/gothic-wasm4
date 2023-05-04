use crate::renderable::Canvas;
use crate::ui::geometry::Rect;

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
    pub(crate) draw_art: fn(&dyn Canvas, Rect),
}

impl CinematicScreen {
    pub const fn new(title: &'static str, draw: fn(&dyn Canvas, Rect)) -> Self {
        Self { text: title, draw_art: draw }
    }
}