use gothic::audio::audio_system::AudioSystem;
use gothic::renderable::ImageProvider;
use gothic::platform_context::PlatformContext;
use crate::wasm4_audio_system::Wasm4AudioSystem;
use crate::wasm4_image_provider::Wasm4ImageProvider;

pub struct Wasm4System;

impl PlatformContext for Wasm4System {
    fn image_provider(&self) -> &'static dyn ImageProvider {
        &Wasm4ImageProvider
    }

    fn audio_system(&self) -> &'static dyn AudioSystem {
        &Wasm4AudioSystem
    }
}