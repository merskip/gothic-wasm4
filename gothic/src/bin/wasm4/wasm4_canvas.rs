use core::any::Any;
use gothic::renderable::{Canvas, Color, Image};
use gothic::ui::geometry::{Point, Size};
use wasm4::framebuffer::Framebuffer;
use wasm4::sprite::Sprite;
use wasm4::{get_char_width, get_char_height};

pub struct Wasm4Canvas<'a> {
    framebuffer: &'a Framebuffer,
}

impl<'a> Wasm4Canvas<'a> {
    pub fn new(framebuffer: &'a Framebuffer) -> Self {
        Self { framebuffer }
    }
}

impl<'a> Canvas for Wasm4Canvas<'a> {
    fn get_size(&self) -> Size {
        Size::new(self.framebuffer.get_screen_width(),
                  self.framebuffer.get_screen_height())
    }

    fn get_char_size(&self) -> Size {
        Size::new(
            get_char_width(),
            get_char_height(),
        )
    }

    fn draw_line(&self, start: Point, end: Point) {
        self.framebuffer.line(start.x, start.y, end.x, end.y);
    }

    fn set_rectangle_color(&self, fill_color: Color, border: Color) {
        // TODO
    }

    fn draw_rectangle(&self, start: Point, size: Size) {
        self.framebuffer.rectangle(start.x, start.y, size.width, size.height);
    }

    fn set_text_color(&self, foreground: Color, background: Color) {
        // TODO
    }

    fn draw_text(&self, text: &str, start: Point) {
        self.framebuffer.text(text, start.x, start.y);
    }

    fn set_image_colors(&self, colors: [Color; 4]) {
        // TODO
    }

    fn draw_image(&self, image: &dyn Image, start: Point) {
        // let sprite = (image as &dyn Any).downcast_ref::<Sprite>().unwrap();
        // TODO
    }
}