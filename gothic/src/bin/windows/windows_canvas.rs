use std::char::decode_utf16;
use std::cmp::max;
use std::ffi::c_void;
use std::mem::{size_of, size_of_val};
use std::path::Path;

use windows::core::PCSTR;
use windows::s;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, GetWindowRect, IMAGE_BITMAP, LoadImageA, LR_LOADFROMFILE};

use gothic::renderable::{Canvas, Color, Image, TextMetrics};
use gothic::ui::geometry::{Point, Size};

use crate::windows_image_provider::WindowsImage;

pub struct WindowsCanvas {
    instance: HMODULE,
    window: HWND,
    paint: PAINTSTRUCT,
    pen_solid: HPEN,
}

impl WindowsCanvas {
    pub unsafe fn new(instance: HMODULE, window: HWND) -> Self {
        let mut paint = PAINTSTRUCT::default();
        BeginPaint(window, &mut paint);

        let pen_solid = CreatePen(PS_SOLID, 1, COLORREF(0xff0000));

        Self { instance, window, paint, pen_solid }
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
        let mut size = Size::new(0, 0);
        unsafe {
            for line in text.lines() {
                let mut line_size = SIZE::default();
                GetTextExtentPointA(self.paint.hdc, line.as_ref(), &mut line_size);
                size.width = max(size.width, line_size.cx as u32);
                size.height += line_size.cy as u32;
            }
        }
        size
    }

    fn set_text_color(&self, foreground: Color, background: Color) {
        // todo!()
    }

    fn draw_text(&self, text: &str, start: Point) {
        unsafe {
            SelectObject(self.paint.hdc, self.pen_solid);

            let mut y = start.y;
            for line in text.lines() {
                TextOutA(self.paint.hdc, start.x, y, line.as_ref());
                let line_size = self.get_text_size(line);
                y += line_size.height as i32;
            }
        }
    }

    // Image

    fn set_image_colors(&self, colors: [Color; 4]) {
        // todo!()
    }

    fn draw_image(&self, image: &dyn Image, start: Point) {
        let windows_image = image.as_any()
            .downcast_ref::<WindowsImage>()
            .unwrap();

        unsafe {
            let path = PCSTR::from_raw(r"King Rhobar 2.bmp".as_ptr());
            let bitmap_handler = LoadImageA(self.instance, path, IMAGE_BITMAP, 0, 0, LR_LOADFROMFILE).unwrap();

            let mut bitmap = BITMAP::default();
            GetObjectA(HBITMAP(bitmap_handler.0), size_of::<BITMAP>() as i32, Some(&mut bitmap as *mut _ as _));

            let hdc_mem = CreateCompatibleDC(GetDC(self.window));
            SelectObject(hdc_mem, HBITMAP(bitmap_handler.0));

            BitBlt(self.paint.hdc, start.x, start.y, bitmap.bmWidth, bitmap.bmHeight, hdc_mem, 0, 0, SRCCOPY);
        };
    }
}