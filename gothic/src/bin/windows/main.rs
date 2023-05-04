use std::mem::MaybeUninit;

use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx};

use gothic::GothicApplication;

use crate::application_window::ApplicationWindow;
use crate::windows_controls::WindowsControls;

mod application_window;
mod windows_controls;
mod windows_canvas;
mod windows_image_provider;
mod windows_images;
mod fps_counter;
mod direct2d_canvas;

static mut APPLICATION: MaybeUninit<GothicApplication> = MaybeUninit::<GothicApplication>::uninit();
static mut CONTROLS: MaybeUninit<WindowsControls> = MaybeUninit::<WindowsControls>::uninit();

fn main() -> windows::core::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;
    }
    let mut window = ApplicationWindow::new()?;
    window.run()
}