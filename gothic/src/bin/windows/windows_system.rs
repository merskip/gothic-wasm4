use gothic::audio::audio_system::AudioSystem;
use gothic::platform_context::PlatformContext;
use gothic::renderable::ImageProvider;

use crate::windows_audio_system::WindowsAudioSystem;
use crate::windows_image_provider::WindowsImageProvider;

pub struct WindowsSystem;

impl PlatformContext for WindowsSystem {
    fn image_provider(&self) -> &'static dyn ImageProvider {
        &WindowsImageProvider
    }

    fn audio_system(&self) -> &'static dyn AudioSystem {
        &WindowsAudioSystem
    }
}

