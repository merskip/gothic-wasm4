use crate::renderable::ImageProvider;

pub trait PlatformContext {
    fn image_provider(&self) -> &'static dyn ImageProvider;
}
