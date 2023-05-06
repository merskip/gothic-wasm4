use gothic::renderable::ImageProvider;
use gothic::platform_context::PlatformContext;
use crate::windows_image_provider::WindowsImageProvider;

pub struct WindowsSystem;

impl PlatformContext for WindowsSystem {
    fn image_provider(&self) -> &'static dyn ImageProvider {
        &WindowsImageProvider
    }
}

