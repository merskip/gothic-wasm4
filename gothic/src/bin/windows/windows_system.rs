use gothic::renderable::ImageProvider;
use gothic::system::System;
use crate::windows_image_provider::WindowsImageProvider;

pub struct WindowsSystem;

impl System for WindowsSystem {
    fn image_provider(&self) -> &'static dyn ImageProvider {
        &WindowsImageProvider
    }
}

