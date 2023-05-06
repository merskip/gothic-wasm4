use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx};

use crate::application_window::ApplicationWindow;

mod application_window;
mod windows_controls;
mod windows_image_provider;
mod windows_images;
mod direct2d_canvas;
mod windows_image;
mod windows_platform;
mod windows_audio_system;
mod square_wave;
mod noise_wave;

fn main() -> windows::core::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;
    }
    let mut window = ApplicationWindow::new()?;
    window.run()
}