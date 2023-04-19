use crate::renderable::{Canvas, Image};
use crate::ui::geometry::Size;

pub enum Images {
    Player,
    KingRhobar2,
    Orc,
    Crossbones,
}

impl Image for Images {
    fn size(&self) -> Size {
        Size::default()
    }

    fn draw_in(&self, canvas: &dyn Canvas) {
        todo!()
    }
}