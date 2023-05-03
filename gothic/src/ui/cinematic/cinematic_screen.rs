use alloc::string::ToString;

use crate::renderable::{Renderable, RenderContext};
use crate::renderable::Color::{Secondary, Transparent};
use crate::ui::cinematic::cinematic::CinematicScreen;
use crate::ui::geometry::{Point, Rect, Size};
use crate::ui::text::Text;
use crate::updatable::{Updatable, UpdateContext};

pub struct CinematicScreenView {
    screen: &'static CinematicScreen,
    screen_text: Text,
}

impl CinematicScreenView {
    const TEXT_PADDING: i32 = 4;

    pub fn new(screen: &'static CinematicScreen) -> Self {
        Self {
            screen,
            screen_text: Text::word_wrapping(screen.text.to_string()),
        }
    }
}

impl Updatable for CinematicScreenView {
    fn update(&mut self, _context: &mut UpdateContext) {}
}

impl Renderable for CinematicScreenView {
    fn render(&self, context: &mut RenderContext) {
        let text_container_size = Size::new(
            context.frame.size.width - 2 * Self::TEXT_PADDING as u32,
            context.frame.size.height - 2 * Self::TEXT_PADDING as u32,
        );
        let text_size = self.screen_text.size(text_container_size, context.canvas);
        let text_origin = Point::new(
            context.frame.origin.x + Self::TEXT_PADDING as i32,
            context.frame.origin.y + context.frame.size.height as i32 - text_size.height as i32 - Self::TEXT_PADDING,
        );

        let separator_y = text_origin.y - Self::TEXT_PADDING;

        let art_frame = Rect::new(
            context.frame.origin,
            Size::new(context.frame.size.width, context.frame.size.height - separator_y as u32),
        );
        (self.screen.draw_art)(context.canvas, art_frame);

        context.canvas.set_line_color(Secondary);
        context.canvas.draw_line(Point::new(context.frame.min_x(), separator_y),
                                 Point::new(context.frame.max_x(), separator_y));

        context.canvas.set_text_color(Secondary, Transparent);
        self.screen_text.render(&mut context.with(text_origin, text_size));
    }
}
