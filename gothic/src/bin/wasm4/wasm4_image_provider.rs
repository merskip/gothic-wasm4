use gothic::image_asset::ImageAsset;
use gothic::renderable::{Image, ImageProvider};

use crate::sprite_images::{CROSSBONES_IMAGE, KING_RHOBAR_2_IMAGE, ORC_IMAGE, PLAYER_IMAGE};

pub struct Wasm4ImageProvider;

impl ImageProvider for Wasm4ImageProvider {
    fn get_image(&self, asset: ImageAsset) -> &dyn Image {
        match asset {
            ImageAsset::Player => &PLAYER_IMAGE,
            ImageAsset::KingRhobar2 => &KING_RHOBAR_2_IMAGE,
            ImageAsset::Orc => &ORC_IMAGE,
            ImageAsset::Crossbones => &CROSSBONES_IMAGE,
        }
    }
}