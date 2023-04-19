use std::mem::MaybeUninit;
use std::os::raw::c_void;
use windows::core::PCSTR;
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
        hbrBackground: Default::default(),
        lpszMenuName: PCSTR::null(),
        lpszClassName: window_class,
    };
    unsafe { RegisterClassA(&wc) };

    unsafe {
        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("My Window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            640,
            480,
            HWND::default(),
            HMENU::default(),
            instance,
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

    // let controls = WindowsControls::new();
    // application.update(&controls);
    //
    // let mut canvas = WindowsCanvas;
    // application.render(&mut canvas);
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    println!("wndproc: message={:?}, wparam={:?}, lparam={:?}", message, wparam, lparam);
    unsafe {
        match message {
            WM_CREATE => {
                let application = GothicApplication::start();
                APPLICATION.write(application);
                LRESULT(0)
            }
            WM_PAINT => {
                let canvas = WindowsCanvas::new(window);
                let application = APPLICATION.assume_init_ref();
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
