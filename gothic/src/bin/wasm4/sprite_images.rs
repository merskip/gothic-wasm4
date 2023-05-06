use gothic::image_asset::ImageAsset::*;

use crate::sprite_image::SpriteImage;
use crate::sprites::*;

pub const PLAYER_IMAGE: SpriteImage = SpriteImage {
    image_asset: Player,
    sprite: PLAYER_SPRITE,
};

pub const KING_RHOBAR_2_IMAGE: SpriteImage = SpriteImage {
    image_asset: KingRhobar2,
    sprite: KING_RHOBAR_2_SPRITE,
};

pub const ORC_IMAGE: SpriteImage = SpriteImage {
    image_asset: Orc,
    sprite: ORC_SPRITE,
};

pub const CROSSBONES_IMAGE: SpriteImage = SpriteImage {
    image_asset: Crossbones,
    sprite: CROSSBONES_SPRITE,
};

