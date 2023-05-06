use crate::audio::audio_system::AudioSystem;
use crate::renderable::ImageProvider;

pub trait PlatformContext {
    fn image_provider(&self) -> &'static dyn ImageProvider;

    fn audio_system(&self) -> &'static dyn AudioSystem;
}
