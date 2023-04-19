use gothic::renderable::{Canvas, Color, Image};
use gothic::ui::geometry::{Point, Size};

pub struct WindowsCanvas;

impl Canvas for WindowsCanvas {
    fn get_size(&self) -> Size {
        Size::new(160, 160)
    }

    fn get_char_size(&self) -> Size {
        Size::new(8, 8)
    }

    fn draw_line(&self, start: Point, end: Point) {
        // todo!()
    }

    fn set_rectangle_color(&self, fill_color: Color, border: Color) {
        // todo!()
    }

    fn draw_rectangle(&self, start: Point, size: Size) {
        // todo!()
    }

    fn set_text_color(&self, foreground: Color, background: Color) {
        // todo!()
    }

    fn draw_text(&self, text: &str, start: Point) {
        // todo!()
    }

    fn set_image_colors(&self, colors: [Color; 4]) {
        // todo!()
    }

    fn draw_image(&self, image: &dyn Image, start: Point) {
        // todo!()
    }
}