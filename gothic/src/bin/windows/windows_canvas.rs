use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, GetWindowRect};

use gothic::renderable::{Canvas, Color, Image, TextMetrics};
use gothic::sprites::RawImage;
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
        let mut rect = RECT::default();
        unsafe {
            GetClientRect(self.window, &mut rect)
        };
        Size::new(rect.right as u32, rect.bottom as u32)
    }

    // Line

    fn set_line_color(&self, color: Color) {
        // todo!()
    }

    fn draw_line(&self, start: Point, end: Point) {
        unsafe {
            SelectObject(self.paint.hdc, self.pen_solid);
            MoveToEx(self.paint.hdc, start.x, start.y, None);
            LineTo(self.paint.hdc, end.x, end.y);
        }
    }

    // Rectangle

    fn set_rectangle_color(&self, fill_color: Color, outline_color: Color) {
        // todo!()
    }

    fn draw_rectangle(&self, start: Point, size: Size) {
        // todo!()
    }

    // Text

    fn get_text_metrics(&self) -> TextMetrics {
        let mut text_metric = TEXTMETRICA::default();
        unsafe {
            GetTextMetricsA(self.paint.hdc, &mut text_metric);
        }
        TextMetrics {
            line_height: text_metric.tmHeight as u32,
            average_character_width: text_metric.tmAveCharWidth as u32,
            maximum_character_width: text_metric.tmMaxCharWidth as u32,
        }
    }

    fn get_text_size(&self, text: &str) -> Size {
        let mut size = SIZE::default();
        unsafe {
            GetTextExtentPointA(self.paint.hdc, text.as_ref(), &mut size);
        }
        Size::new(size.cx as u32, size.cy as u32)
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

    // Image

    fn set_image_colors(&self, colors: [Color; 4]) {
        // todo!()
    }

    fn draw_image(&self, image: &dyn Image, start: Point) {
        // todo!()
    }
}