use core::any::Any;

use gothic::image_asset::ImageAsset;
use gothic::renderable::Image;
use gothic::ui::geometry::Size;
use wasm4::sprite::Sprite;

pub struct SpriteImage {
    pub image_asset: ImageAsset,
    pub sprite: &'static Sprite,
}

impl Image for SpriteImage {
    fn image_asset(&self) -> ImageAsset {
        self.image_asset
    }

    fn size(&self) -> Size {
        Size::new(self.sprite.width, self.sprite.height)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
