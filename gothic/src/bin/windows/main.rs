use std::os::raw::c_void;
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

fn main() {
    let mut application = GothicApplication::start();

    let controls = WindowsControls::new();
    application.update(&controls);

    let mut canvas = WindowsCanvas;
    application.render(&mut canvas);

    println!("Hello world!");
}
