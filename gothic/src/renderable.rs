use core::any::Any;
use crate::image_asset::ImageAsset;
use crate::ui::geometry::{Point, Rect, Size};
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

    fn as_any(&self) -> &dyn Any;
}

pub trait ImageProvider {
    fn get_image(&self, asset: ImageAsset) -> &dyn Image;
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

    // Line

    fn set_line_color(&self, color: Color);

    fn draw_line(&self, start: Point, end: Point);

    // Rectangle

    fn set_rectangle_color(&self, fill_color: Color, outline_color: Color);

    fn draw_rectangle(&self, start: Point, size: Size);

    // Text

    fn get_text_metrics(&self) -> TextMetrics;

    fn get_text_size(&self, text: &str) -> Size;

    fn set_text_color(&self, foreground: Color, background: Color);

    fn draw_text(&self, text: &str, start: Point);

    // Image

    fn set_image_colors(&self, colors: [Color; 4]);

    fn draw_image(&self, image: &dyn Image, start: Point);
}
