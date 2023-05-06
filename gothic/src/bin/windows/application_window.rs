use std::mem::MaybeUninit;

use windows::core::Result;
use windows::s;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

use gothic::GothicApplication;

use crate::direct2d_canvas::Direct2DCanvas;
use crate::windows_controls::WindowsControls;
use crate::windows_system::WindowsSystem;

pub struct ApplicationWindow {
    application: MaybeUninit<GothicApplication>,
    controls: WindowsControls,
    window_handle: HWND,
    canvas: Option<Direct2DCanvas>,
    visible: bool,
}

impl ApplicationWindow {
    const GAME_UPDATE_TIMER_ID: usize = 1;

    pub fn new() -> Result<Self> {
        Ok(Self {
            application: MaybeUninit::uninit(),
            controls: WindowsControls::new(),
            window_handle: HWND(0),
            canvas: None,
            visible: false,
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
                WM_CREATE => {
                    self.handle_create().unwrap();
                    LRESULT(0)
                }
                WM_ACTIVATE => {
                    self.visible = true;
                    LRESULT(0)
                }
                WM_TIMER => {
                    let timer_id = wparam.0;
                    self.handle_timer(timer_id).unwrap();
                    LRESULT(0)
                }
                WM_PAINT => {
                    self.handle_paint().unwrap();
                    LRESULT(0)
                }
                WM_DISPLAYCHANGE => {
                    self.render().unwrap();
                    LRESULT(0)
                }
                WM_SIZE => {
                    self.handle_size(
                        low_word(lparam),
                        high_word(lparam),
                    ).unwrap();
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

    fn handle_create(&mut self) -> Result<()> {
        let application = GothicApplication::start(&WindowsSystem);
        self.application.write(application);

        unsafe {
            SetTimer(self.window_handle, Self::GAME_UPDATE_TIMER_ID, 16, None);
        }

        Ok(())
    }

    fn handle_timer(&mut self, timer_id: usize) -> Result<()> {
        match timer_id {
            Self::GAME_UPDATE_TIMER_ID => self.game_update()?,
            _ => panic!("Unknown timer id: {}", timer_id)
        }
        Ok(())
    }

    fn handle_paint(&mut self) -> Result<()> {
        let mut paint = PAINTSTRUCT::default();
        unsafe {
            BeginPaint(self.window_handle, &mut paint);
            self.render()?;
            EndPaint(self.window_handle, &paint);
        }
        Ok(())
    }

    fn handle_size(&mut self, width: u16, height: u16) -> Result<()> {
        if let Some(canvas) = &self.canvas {
            canvas.resize(width as u32, height as u32)?;
        }
        Ok(())
    }

    fn game_update(&mut self) -> Result<()> {
        let application = unsafe {
            self.application.assume_init_mut()
        };
        application.update(&self.controls);
        self.controls.late_update();
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        if self.canvas.is_none() {
            self.canvas = Some(Direct2DCanvas::new(self.window_handle)?);
        }
        let application = unsafe { self.application.assume_init_ref() };
        let canvas = self.canvas.as_ref().unwrap();

        canvas.begin_draw()?;
        application.render(canvas);
        canvas.end_draw()?;

        Ok(())
    }
}

fn low_word(lparam: LPARAM) -> u16 {
    (lparam.0 & 0xffff) as u16
}

fn high_word(lparam: LPARAM) -> u16 {
    ((lparam.0 >> 16) & 0xffff) as u16
}
