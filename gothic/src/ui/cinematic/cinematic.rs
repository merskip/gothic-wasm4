use crate::ui::cinematic::cinematic_screen::CinematicScreen;

pub struct Cinematic {
    pub(crate) screens: &'static [CinematicScreen],
}

impl Cinematic {
    pub const fn new(screens: &'static [CinematicScreen]) -> Self {
        Self { screens }
    }
}