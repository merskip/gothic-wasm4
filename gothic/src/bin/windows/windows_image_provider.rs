use std::any::Any;
use gothic::image_asset::ImageAsset;
use gothic::renderable::{Image, ImageProvider};
use gothic::ui::geometry::Size;

pub struct WindowsImage {
    pub bytes: &'static [u8],
    pub native_size: Size,
    pub size: Size,
}

pub struct WindowsImageProvider;

impl ImageProvider for WindowsImageProvider {
    fn get_image(&self, asset: ImageAsset) -> &dyn Image {
        match asset {
            ImageAsset::Player => &WindowsImage::PLAYER,
            ImageAsset::KingRhobar2 => &WindowsImage::KING_RHOBAR_2,
            ImageAsset::Orc => &WindowsImage::ORC,
            ImageAsset::Crossbones => &WindowsImage::CROSSBONES,
        }
    }
}
impl Image for WindowsImage {
    fn size(&self) -> Size {
        self.size
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}