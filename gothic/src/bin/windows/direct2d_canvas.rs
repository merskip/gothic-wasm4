use windows::core::{PWSTR, Result};
use windows::Foundation::Numerics::Matrix3x2;
use windows::w;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::Direct2D::Common::*;
use windows::Win32::Graphics::DirectWrite::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

use gothic::renderable::{Canvas, Color, Image, TextAlignment, TextWrapping};
use gothic::ui::geometry::{Point, Size};

pub struct Direct2DCanvas {
    window_handle: HWND,
    factory: ID2D1Factory1,
    write_factory: IDWriteFactory,
    target: ID2D1HwndRenderTarget,
    brush: ID2D1SolidColorBrush,
    stroke_style: ID2D1StrokeStyle,
    text_format: IDWriteTextFormat,
}

impl Direct2DCanvas {
    pub fn new(window_handle: HWND) -> Result<Self> {
        let factory = create_factory()?;
        let write_factory = create_write_factory()?;
        let target = create_render_target(&factory, window_handle)?;
        let brush = create_brush(&target)?;
        let stroke_style = create_stroke_style(&factory)?;
        let text_format = create_text_format(&write_factory)?;

        Ok(Self {
            window_handle,
            factory,
            write_factory,
            target,
            brush,
            stroke_style,
            text_format,
        })
    }

    pub fn resize(&self, width: u32, height: u32) -> Result<()> {
        unsafe {
            self.target.Resize(&D2D_SIZE_U {
                width,
                height,
            })
        }
    }

    pub fn begin_draw(&self) -> Result<()> {
        unsafe {
            self.target.BeginDraw();
            self.target.Clear(None);
        }
        Ok(())
    }

    pub fn end_draw(&self) -> Result<()> {
        unsafe {
            self.target.EndDraw(None, None)
        }
    }
}

impl Canvas for Direct2DCanvas {
    fn get_size(&self) -> Size {
        let size = unsafe { self.target.GetSize() };
        Size::new(size.width as u32, size.height as u32)
    }

    // Line

    fn set_line_color(&self, color: Color) {
        // todo!()
    }

    fn draw_line(&self, start: Point, end: Point) {
        unsafe {
            self.target.DrawLine(
                D2D_POINT_2F {
                    x: start.x as f32,
                    y: start.y as f32,
                },
                D2D_POINT_2F {
                    x: end.x as f32,
                    y: end.y as f32,
                },
                &self.brush,
                2.0,
                &self.stroke_style,
            );
        }
    }

    // Rectangle

    fn set_rectangle_color(&self, fill_color: Color, outline_color: Color) {
        // todo!()
    }

    fn draw_rectangle(&self, start: Point, size: Size) {
        // todo!()
    }

    fn get_text_size(&self, text: &str, container_size: Size, text_wrapping: TextWrapping) -> Size {
        let text_layout = create_text_layout(
            &self.write_factory,
            text,
            &self.text_format,
            container_size.width as f32,
            container_size.height as f32,
        ).unwrap();
        let mut metrics = DWRITE_TEXT_METRICS::default();
        unsafe { text_layout.GetMetrics(&mut metrics).unwrap() };
        Size::new(metrics.width as u32, metrics.height as u32)
    }

    fn set_text_color(&self, foreground: Color, background: Color) {
        // todo!()
    }

    fn draw_text(&self, text: &str, start: Point, size: Size, text_wrapping: TextWrapping, text_alignment: TextAlignment) {
        unsafe {
            self.target.DrawText(
                text.encode_utf16().collect::<Vec<u16>>().as_slice(),
                &self.text_format,
                &D2D_RECT_F {
                    left: start.x as f32,
                    top: start.y as f32,
                    right: start.x as f32 + size.width as f32,
                    bottom: start.y as f32 + size.height as f32,
                },
                &self.brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
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

fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, Some(&options)) }
}

fn create_write_factory() -> Result<IDWriteFactory> {
    unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED) }
}

fn create_render_target(
    factory: &ID2D1Factory1,
    window_handle: HWND,
) -> Result<ID2D1HwndRenderTarget> {
    unsafe {
        let mut rect = RECT::default();
        GetClientRect(window_handle, &mut rect);

        let mut dpi_x = 0.0;
        let mut dpi_y = 0.0;
        factory.GetDesktopDpi(&mut dpi_x, &mut dpi_y);

        factory.CreateHwndRenderTarget(
            &D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_UNKNOWN,
                    alphaMode: D2D1_ALPHA_MODE_UNKNOWN,
                },
                dpiX: dpi_x,
                dpiY: dpi_y,
                usage: D2D1_RENDER_TARGET_USAGE_NONE,
                minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
            },
            &D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd: window_handle,
                pixelSize: D2D_SIZE_U {
                    width: (rect.right - rect.left) as u32,
                    height: (rect.bottom - rect.top) as u32,
                },
                presentOptions: D2D1_PRESENT_OPTIONS_NONE,
            },
        )
    }
}

fn create_text_format(write_factory: &IDWriteFactory) -> Result<IDWriteTextFormat> {
    unsafe {
        let text_format = write_factory.CreateTextFormat(
            w!("Segoe UI"),
            None,
            DWRITE_FONT_WEIGHT_NORMAL,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            24.0,
            w!("en"),
        )?;
        text_format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER)?;
        text_format.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;
        Ok(text_format)
    }
}

fn create_text_layout(write_factory: &IDWriteFactory, text: &str, text_format: &IDWriteTextFormat, max_width: f32, max_height: f32) -> Result<IDWriteTextLayout> {
    unsafe {
        let chars = text.encode_utf16().collect::<Vec<u16>>();
        write_factory.CreateTextLayout(
            chars.as_slice(),
            text_format,
            max_width,
            max_height,
        )
    }
}

fn create_brush(target: &ID2D1HwndRenderTarget) -> Result<ID2D1SolidColorBrush> {
    let color = D2D1_COLOR_F {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    let properties = D2D1_BRUSH_PROPERTIES {
        opacity: 1.0,
        transform: Matrix3x2::identity(),
    };

    unsafe { target.CreateSolidColorBrush(&color, Some(&properties)) }
}

fn create_stroke_style(factory: &ID2D1Factory1) -> Result<ID2D1StrokeStyle> {
    let props = D2D1_STROKE_STYLE_PROPERTIES {
        startCap: D2D1_CAP_STYLE_ROUND,
        endCap: D2D1_CAP_STYLE_TRIANGLE,
        ..Default::default()
    };

    unsafe { factory.CreateStrokeStyle(&props, None) }
}
