use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;

use gothic::renderable::{Canvas, Color, Image};
use gothic::ui::geometry::{Point, Size};

pub struct WindowsCanvas {
    window: HWND,
    paint: PAINTSTRUCT,
    pen_solid: HPEN,
}

impl WindowsCanvas {
    pub unsafe fn new(window: HWND) -> Self {
        let mut paint = PAINTSTRUCT::default();
        BeginPaint(window, &mut paint);

        let pen_solid = CreatePen(PS_SOLID, 1, COLORREF(0xff0000));

        Self { window, paint, pen_solid }
    }
}

impl Drop for WindowsCanvas {
    fn drop(&mut self) {
        unsafe {
            EndPaint(self.window, &mut self.paint);
        }
    }
}

impl Canvas for WindowsCanvas {
    fn get_size(&self) -> Size {
        Size::new(160, 160)
    }

    fn get_char_size(&self) -> Size {
        Size::new(8, 8)
    }

    fn draw_line(&self, start: Point, end: Point) {
        unsafe {
            SelectObject(self.paint.hdc, self.pen_solid);
            MoveToEx(self.paint.hdc, start.x, start.y, None);
            LineTo(self.paint.hdc, end.x, end.y);
        }
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
        unsafe {
            SelectObject(self.paint.hdc, self.pen_solid);
            TextOutA(self.paint.hdc, start.x, start.y, text.as_ref());
        }
    }

    fn set_image_colors(&self, colors: [Color; 4]) {
        // todo!()
    }

    fn draw_image(&self, image: &dyn Image, start: Point) {
        // todo!()
    }
}