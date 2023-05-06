use gothic::renderable::ImageProvider;
use gothic::platform_context::PlatformContext;
use crate::wasm4_image_provider::Wasm4ImageProvider;

pub struct Wasm4System;

impl PlatformContext for Wasm4System {
    fn image_provider(&self) -> &'static dyn ImageProvider {
        &Wasm4ImageProvider
    }
}