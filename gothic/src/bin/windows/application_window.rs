use std::iter::Peekable;

use windows::s;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::Dxgi::CreateDXGIFactory1;
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, PAINTSTRUCT};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

pub struct ApplicationWindow {
    window_handle: HWND,
    // factory: ID2D1Factory1,
    // dx_factory: ID2D1Factory2,
    visible: bool,
}

impl ApplicationWindow {
    pub fn new() -> windows::core::Result<Self> {
        // let factory = unsafe {
        //     let mut options = D2D1_FACTORY_OPTIONS::default();
        //     if cfg!(debug_assertions) {
        //         options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
        //     }
        //     D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, Some(&options))?
        // };
        //
        // let dx_factory = unsafe {
        //     CreateDXGIFactory1()?
        // };

        Ok(Self {
            window_handle: HWND(0),
            // factory,
            // dx_factory,
            visible: false,
        })
    }

    pub fn run(&mut self) -> windows::core::Result<()> {
        unsafe {
            let instance = GetModuleHandleA(None)?;
            debug_assert!(!instance.is_invalid());

            let window_class = s!("window");

            let wc = WNDCLASSA {
                hInstance: instance,
                hCursor: LoadCursorW(None, IDC_ARROW)?,
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

            let mut message = MSG::default();
            loop {
                if self.visible {
                    self.render()?;

                    while PeekMessageA(&mut message, None, 0, 0, PM_REMOVE).into() {
                        if message.message == WM_QUIT {
                            return Ok(());
                        }
                        DispatchMessageA(&message);
                    }
                } else {
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
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(self.window_handle, message, wparam, lparam),
            }
        }
    }

    fn render(&mut self) -> windows::core::Result<()> {
        Ok(())
    }
}