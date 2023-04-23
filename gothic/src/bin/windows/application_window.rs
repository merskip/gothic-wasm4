use windows::{s, w};
use windows::core::{ComInterface, Interface, PCSTR, PWSTR, Result};
use windows::Foundation::Numerics::Matrix3x2;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_ALPHA_MODE_IGNORE, D2D1_ALPHA_MODE_UNKNOWN, D2D1_COLOR_F, D2D1_PIXEL_FORMAT, D2D_RECT_F, D2D_SIZE_U};
use windows::Win32::Graphics::DirectWrite::{DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_NORMAL, DWRITE_MEASURING_MODE_NATURAL, DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_CENTER, DWriteCreateFactory, IDWriteFactory2, IDWriteTextFormat};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_UNKNOWN;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::fps_counter::FPSCounter;

pub struct ApplicationWindow {
    window_handle: HWND,
    factory: ID2D1Factory1,
    visible: bool,
    target: Option<ID2D1HwndRenderTarget>,
    brush: Option<ID2D1SolidColorBrush>,
    text_format: Option<IDWriteTextFormat>,
    fps_counter: FPSCounter,
}

impl ApplicationWindow {
    pub fn new() -> Result<Self> {
        let factory = create_factory()?;

        Ok(Self {
            window_handle: HWND(0),
            factory,
            visible: false,
            target: None,
            brush: None,
            text_format: None,
            fps_counter: FPSCounter::new(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        unsafe {
            let instance = GetModuleHandleA(None)?;
            debug_assert!(!instance.is_invalid());

            let window_class = s!("window");

            let wc = WNDCLASSA {
                hInstance: instance,
                hCursor: LoadCursorW(None, IDI_APPLICATION)?,
                lpszClassName: window_class,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let window_handle = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                window_class,
                s!("Gothic"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                640,
                480,
                None,
                None,
                instance,
                Some(self as *mut _ as _),
            );
            debug_assert!(window_handle.0 != 0);
            debug_assert!(self.window_handle == window_handle);

            self.run_message_loop()
        }
    }

    fn run_message_loop(&mut self) -> Result<()> {
        let mut message = MSG::default();
        loop {
            if self.visible {
                self.render()?;

                unsafe {
                    while PeekMessageA(&mut message, None, 0, 0, PM_REMOVE).into() {
                        if message.message == WM_QUIT {
                            return Ok(());
                        }
                        DispatchMessageA(&message);
                    }
                }
            } else {
                unsafe {
                    GetMessageA(&mut message, None, 0, 0);
                    if message.message == WM_QUIT {
                        return Ok(());
                    }
                    DispatchMessageA(&message);
                }
            }
        }
    }

    extern "system" fn wndproc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match message {
                WM_NCCREATE => {
                    let create = lparam.0 as *const CREATESTRUCTA;
                    let this = (*create).lpCreateParams as *mut Self;
                    (*this).window_handle = window;
                    SetWindowLongPtrA(window, GWLP_USERDATA, this as _);
                }
                _ => {
                    let this = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut Self;
                    if !this.is_null() {
                        return (*this).handle_message(message, wparam, lparam);
                    }
                }
            }
            return DefWindowProcA(window, message, wparam, lparam);
        }
    }

    fn handle_message(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match message {
                WM_ACTIVATE => {
                    self.visible = true;
                    LRESULT(0)
                }
                WM_PAINT => {
                    let mut paint = PAINTSTRUCT::default();
                    BeginPaint(self.window_handle, &mut paint);
                    self.render().unwrap();
                    EndPaint(self.window_handle, &paint);
                    LRESULT(0)
                }
                WM_DISPLAYCHANGE => {
                    self.render().unwrap();
                    LRESULT(0)
                }
                WM_SIZE => {
                    if let Some(target) = &self.target {
                        let width = low_word(lparam);
                        let height = high_word(lparam);
                        target.Resize(&D2D_SIZE_U {
                            width: width as u32,
                            height: height as u32,
                        }).unwrap();
                    }
                    LRESULT(0)
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(self.window_handle, message, wparam, lparam),
            }
        }
    }

    fn render(&mut self) -> Result<()> {
        if self.target.is_none() {
            let target = create_render_target(&self.factory, self.window_handle)?;
            self.brush = create_brush(&target).ok();
            self.target = Some(target);
            self.text_format = Some(create_text_format()?);
        }

        let target = self.target.as_ref().unwrap();
        unsafe {
            target.BeginDraw();
        };

        let fps = self.fps_counter.tick();
        self.draw(&target, fps)?;

        unsafe {
            target.EndDraw(None, None)?;
        }

        Ok(())
    }

    fn draw(&self, target: &ID2D1HwndRenderTarget, fps: u32) -> Result<()> {
        unsafe {
            target.SetTransform(&Matrix3x2::identity());
            target.Clear(Some(&D2D1_COLOR_F {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }));

            let brush = self.brush.as_ref().unwrap();

            target.DrawRectangle(
                &D2D_RECT_F {
                    left: 100.0,
                    top: 100.0,
                    right: 200.0,
                    bottom: 200.0,
                },
                brush,
                2.0,
                None,
            );

            let text = format!("fps: {}\0", fps);

            target.DrawText(
                 PWSTR(text.encode_utf16().collect::<Vec<u16>>().as_mut_ptr()).as_wide(),
                self.text_format.as_ref().unwrap(),
                &D2D_RECT_F {
                    left: 0.0,
                    top: 0.0,
                    right: 100.0,
                    bottom: 0.0,
                },
                brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
        Ok(())
    }
}

fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, Some(&options)) }
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

fn create_text_format() -> Result<IDWriteTextFormat> {
    unsafe {
        let factory: IDWriteFactory2 = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;

        let format = factory.CreateTextFormat(
            w!("Segoe UI"),
            None,
            DWRITE_FONT_WEIGHT_NORMAL,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            16.0,
            w!("en"),
        )?;
        Ok(format)
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

fn create_style(factory: &ID2D1Factory1) -> Result<ID2D1StrokeStyle> {
    let props = D2D1_STROKE_STYLE_PROPERTIES {
        startCap: D2D1_CAP_STYLE_ROUND,
        endCap: D2D1_CAP_STYLE_TRIANGLE,
        ..Default::default()
    };

    unsafe { factory.CreateStrokeStyle(&props, None) }
}

fn low_word(lparam: LPARAM) -> u16 {
    (lparam.0 & 0xffff) as u16
}

fn high_word(lparam: LPARAM) -> u16 {
    ((lparam.0 >> 16) & 0xffff) as u16
}
