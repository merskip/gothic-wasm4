use std::mem::MaybeUninit;
use std::os::raw::c_void;

use windows::core::{CanInto, PCSTR};
use windows::s;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

use gothic::dispatcher::Dispatcher;
use gothic::GothicApplication;
use gothic::images::Images;
use gothic::renderable::{Canvas, Image};
use gothic::ui::geometry::Size;
use gothic::ui::navigator::Navigator;
use gothic::updatable::{Button, Controls, UpdateContext};

use crate::windows_canvas::WindowsCanvas;
use crate::windows_controls::{WindowsButton, WindowsControls};

mod windows_controls;
mod windows_canvas;

static mut APPLICATION: MaybeUninit<GothicApplication> = MaybeUninit::<GothicApplication>::uninit();
static mut CONTROLS: MaybeUninit<WindowsControls> = MaybeUninit::<WindowsControls>::uninit();

fn main() {
    let instance = unsafe { GetModuleHandleA(PCSTR::null()).unwrap() };

    let window_class = s!("window");

    let wc = WNDCLASSA {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: instance,
        hIcon: Default::default(),
        hCursor: Default::default(),
        hbrBackground: HBRUSH(unsafe { GetStockObject(WHITE_BRUSH).0 }),
        lpszMenuName: PCSTR::null(),
        lpszClassName: window_class,
    };
    unsafe { RegisterClassA(&wc) };

    let window = unsafe {
        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("Gothic"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            640,
            480,
            HWND::default(),
            HMENU::default(),
            instance,
            None,
        )
    };

    unsafe {
        SetTimer(
            window,
            1,
            16,
            None,
        );
    }

    let mut message = MSG::default();
    unsafe {
        while GetMessageA(&mut message, HWND::default(), 0, 0).as_bool() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_CREATE => {
                let application = GothicApplication::start();
                APPLICATION.write(application);
                CONTROLS.write(WindowsControls::new());

                LRESULT(0)
            }
            WM_TIMER => {
                let application = APPLICATION.assume_init_mut();

                let controls = CONTROLS.assume_init_mut();
                application.update(controls);

                controls.late_update();

                InvalidateRect(window, None, true);
                LRESULT(0)
            }
            WM_PAINT => {
                let application = APPLICATION.assume_init_ref();

                let canvas = WindowsCanvas::new(window);
                application.render(&canvas);

                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
