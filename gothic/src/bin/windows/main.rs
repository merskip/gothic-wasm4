use std::mem::MaybeUninit;
use std::os::raw::c_void;

use windows::core::{CanInto, PCSTR};
use windows::s;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

use gothic::dispatcher::Dispatcher;
use gothic::GothicApplication;
use gothic::image_asset::ImageAsset;
use gothic::renderable::{Canvas, Image};
use gothic::ui::geometry::Size;
use gothic::ui::navigator::Navigator;
use gothic::updatable::{Button, Controls, UpdateContext};
use crate::application_window::ApplicationWindow;

use crate::windows_canvas::WindowsCanvas;
use crate::windows_controls::{WindowsButton, WindowsControls};
use crate::windows_image_provider::WindowsImageProvider;

mod application_window;
mod windows_controls;
mod windows_canvas;
mod windows_image_provider;
mod windows_images;

static mut APPLICATION: MaybeUninit<GothicApplication> = MaybeUninit::<GothicApplication>::uninit();
static mut CONTROLS: MaybeUninit<WindowsControls> = MaybeUninit::<WindowsControls>::uninit();

fn main() -> windows::core::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;
    }
    let mut window = ApplicationWindow::new()?;
    window.run()
}