use gothic::image_asset::ImageAsset;
use gothic::renderable::{Image, ImageProvider};

use crate::windows_image::WindowsImage;
use crate::windows_images::*;

pub struct WindowsImageProvider;

impl ImageProvider for WindowsImageProvider {
    fn get_image(&self, asset: ImageAsset) -> &dyn Image {
        unsafe { self.get_mut_image(asset) }
    }
}

impl WindowsImageProvider {
    pub unsafe fn get_mut_image(&self, asset: ImageAsset) -> &mut WindowsImage {
        match asset {
            ImageAsset::Player => &mut PLAYER_IMAGE,
            ImageAsset::KingRhobar2 => &mut KING_RHOBAR_2_IMAGE,
            ImageAsset::Orc => &mut ORC_IMAGE,
            ImageAsset::Crossbones => &mut CROSSBONES_IMAGE,
        }
    }
}