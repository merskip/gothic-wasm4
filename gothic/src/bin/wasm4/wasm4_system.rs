use gothic::renderable::ImageProvider;
use gothic::system::System;
use crate::wasm4_image_provider::Wasm4ImageProvider;

pub struct Wasm4System;

impl System for Wasm4System {
    fn image_provider(&self) -> &'static dyn ImageProvider {
        &Wasm4ImageProvider
    }
}