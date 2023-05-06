use std::cell::RefCell;

use windows::core::{CanInto, Result};
use windows::Foundation::Numerics::Matrix3x2;
use windows::w;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::Direct2D::Common::*;
use windows::Win32::Graphics::DirectWrite::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Imaging::*;
use windows::Win32::System::Com::*;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

use gothic::renderable::{Canvas, Color, Image, TextAlignment, TextWrapping};
use gothic::ui::geometry::{Point, Size};

use crate::windows_image_provider::WindowsImageProvider;

pub struct Direct2DCanvas {
    write_factory: IDWriteFactory,
    imaging_factory: IWICImagingFactory,
    target: ID2D1HwndRenderTarget,
    stroke_style: ID2D1StrokeStyle,
    text_format: IDWriteTextFormat,
    palette: Palette,
    draw_indexes: RefCell<DrawIndexes>,
}

type Palette = [D2D1_COLOR_F; 4];
type DrawIndexes = [Color; 4];

const TRANSPARENT_COLOR: D2D1_COLOR_F = D2D1_COLOR_F {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

impl Direct2DCanvas {
    pub fn new(window_handle: HWND) -> Result<Self> {
        let factory = create_factory()?;
        let write_factory = create_write_factory()?;
        let imaging_factory = create_imaging_factory()?;
        let target = create_render_target(&factory, window_handle)?;

        let stroke_style = create_stroke_style(&factory)?;
        let text_format = create_text_format(&write_factory)?;

        Ok(Self {
            write_factory,
            imaging_factory,
            target,
            palette: [
                create_d2d1_color(0xe0f8cf_ff),
                create_d2d1_color(0x86c06c_ff),
                create_d2d1_color(0x306850_ff),
                create_d2d1_color(0x071821_ff),
            ],
            draw_indexes: RefCell::new([
                Color::Background,
                Color::Primary,
                Color::Secondary,
                Color::Tertiary,
            ]),
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
            self.target.Clear(Some(&self.get_color_by_index(0)));
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
        let mut draw_indexes = self.draw_indexes.borrow_mut();
        draw_indexes[1] = color;
    }

    fn draw_line(&self, start: Point, end: Point) {
        unsafe {
            let brush = create_brush(&self.target, self.get_color_by_index(1)).unwrap();

            self.target.DrawLine(
                D2D_POINT_2F {
                    x: start.x as f32,
                    y: start.y as f32,
                },
                D2D_POINT_2F {
                    x: end.x as f32,
                    y: end.y as f32,
                },
                &brush,
                2.0,
                &self.stroke_style,
            );
        }
    }

    // Rectangle

    fn set_rectangle_color(&self, _fill_color: Color, _outline_color: Color) {
        // todo!()
    }

    fn draw_rectangle(&self, _start: Point, _size: Size) {
        // todo!()
    }

    fn get_text_size(&self, text: &str, container_size: Size, _text_wrapping: TextWrapping) -> Size {
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
        let mut draw_indexes = self.draw_indexes.borrow_mut();
        draw_indexes[1] = foreground;
        draw_indexes[2] = background;
    }

    fn draw_text(&self, text: &str, start: Point, size: Size, text_wrapping: TextWrapping, text_alignment: TextAlignment) {
        unsafe {
            let foreground_brush = create_brush(&self.target, self.get_color_by_index(1)).unwrap();

            let text_layout = create_text_layout(
                &self.write_factory,
                text,
                &self.text_format,
                size.width as f32,
                size.height as f32,
            ).unwrap();

            match text_wrapping {
                TextWrapping::None => text_layout.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP).unwrap(),
                TextWrapping::Words => text_layout.SetWordWrapping(DWRITE_WORD_WRAPPING_WRAP).unwrap(),
            }
            match text_alignment {
                TextAlignment::Start => text_layout.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_LEADING).unwrap(),
                TextAlignment::Center => text_layout.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER).unwrap(),
                TextAlignment::End => text_layout.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_TRAILING).unwrap(),
            }
            text_layout.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_NEAR).unwrap();

            if self.draw_indexes.borrow()[1] == Color::Tertiary {
                let text_range = DWRITE_TEXT_RANGE {
                    startPosition: 0,
                    length: text.len() as u32,
                };
                text_layout.SetFontWeight(DWRITE_FONT_WEIGHT_BOLD, text_range).unwrap();
            }

            self.target.DrawTextLayout(
                D2D_POINT_2F {
                    x: start.x as f32,
                    y: start.y as f32,
                },
                &text_layout,
                &foreground_brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
            );
        }
    }

    // Image

    fn set_image_colors(&self, _colors: [Color; 4]) {
        // todo!()
    }

    fn draw_image(&self, image: &dyn Image, start: Point) {
        unsafe {
            let windows_image = WindowsImageProvider.get_mut_image(image.image_asset());
            let bitmap = windows_image.bitmap(&self.imaging_factory, self.target.can_into());

            self.target.DrawBitmap(
                &bitmap,
                Some(&D2D_RECT_F {
                    left: start.x as f32,
                    top: start.y as f32,
                    right: start.x as f32 + windows_image.size.width as f32,
                    bottom: start.y as f32 + windows_image.size.height as f32,
                }),
                1.0,
                D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
                Some(&D2D_RECT_F {
                    left: 0.0,
                    top: 0.0,
                    right: windows_image.native_size.width as f32,
                    bottom: windows_image.native_size.height as f32,
                }),
            );
        }
    }
}

impl Direct2DCanvas {
    fn get_color_by_index(&self, index: usize) -> D2D1_COLOR_F {
        assert!(index <= 4);
        let draw_index = self.draw_indexes.borrow()[index];
        match draw_index {
            Color::Transparent => TRANSPARENT_COLOR,
            Color::Background => self.palette[0],
            Color::Primary => self.palette[1],
            Color::Secondary => self.palette[2],
            Color::Tertiary => self.palette[3],
        }
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

fn create_imaging_factory() -> Result<IWICImagingFactory> {
    unsafe { CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER) }
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

fn create_brush(target: &ID2D1HwndRenderTarget, color: D2D1_COLOR_F) -> Result<ID2D1SolidColorBrush> {
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

fn create_d2d1_color(rgba: u32) -> D2D1_COLOR_F {
    let red = ((rgba >> 24) & 0xFF) as f32 / 255.0;
    let green = ((rgba >> 16) & 0xFF) as f32 / 255.0;
    let blue = ((rgba >> 8) & 0xFF) as f32 / 255.0;
    let alpha = (rgba & 0xFF) as f32 / 255.0;
    D2D1_COLOR_F { r: red, g: green, b: blue, a: alpha }
}