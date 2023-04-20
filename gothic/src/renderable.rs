use crate::ui::geometry::{Point, Rect, Size};
use crate::ui::text::Text;
use crate::updatable::Updatable;

pub trait Renderable: Updatable {
    fn render(&self, context: &mut RenderContext);
}

pub struct RenderContext<'a> {
    pub canvas: &'a dyn Canvas,
    pub frame: Rect,
}

impl<'a> RenderContext<'a> {
    pub fn new(canvas: &'a dyn Canvas, frame: Rect) -> Self {
        Self { canvas, frame }
    }

    pub fn with_frame(&self, frame: Rect) -> Self {
        Self {
            canvas: self.canvas,
            frame,
        }
    }
}

pub trait Image {
    fn size(&self) -> Size;

    fn draw_in(&self, canvas: &dyn Canvas);
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Transparent,
    Background,
    Primary,
    Secondary,
    Tertiary,
}

pub struct TextMetrics {
    pub line_height: u32,
    pub average_character_width: u32,
    pub maximum_character_width: u32,
}

pub trait Canvas {
    fn get_size(&self) -> Size;

    fn get_text_metrics(&self) -> TextMetrics;

    fn get_text_size(&self, text: &str) -> Size;

    fn draw_line(&self, start: Point, end: Point);

    fn set_rectangle_color(&self, fill_color: Color, border: Color);

    fn draw_rectangle(&self, start: Point, size: Size);

    fn set_text_color(&self, foreground: Color, background: Color);

    fn draw_text(&self, text: &str, start: Point);

    fn set_image_colors(&self, colors: [Color; 4]);

    fn draw_image(&self, image: &dyn Image, start: Point);
}
