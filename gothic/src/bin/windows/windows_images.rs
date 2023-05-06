use gothic::image_asset::ImageAsset::*;

use crate::windows_image::WindowsImage;

pub static mut PLAYER_IMAGE: WindowsImage = WindowsImage {
    image_asset: Player,
    bytes: include_bytes!(r"../../../resources/player.png"),
    native_size: gothic::ui::geometry::Size::new(12, 16),
    size: gothic::ui::geometry::Size::new(48, 64),
    cached_bitmap: None,
};

pub static mut KING_RHOBAR_2_IMAGE: WindowsImage = WindowsImage {
    image_asset: KingRhobar2,
    bytes: include_bytes!(r"../../../resources/king_rhobar_2.png"),
    native_size: gothic::ui::geometry::Size::new(16, 18),
    size: gothic::ui::geometry::Size::new(64, 72),
    cached_bitmap: None,
};

pub static mut ORC_IMAGE: WindowsImage = WindowsImage {
    image_asset: Orc,
    bytes: include_bytes!(r"../../../resources/orc.png"),
    native_size: gothic::ui::geometry::Size::new(24, 22),
    size: gothic::ui::geometry::Size::new(96, 88),
    cached_bitmap: None,
};

pub static mut CROSSBONES_IMAGE: WindowsImage = WindowsImage {
    image_asset: Crossbones,
    bytes: include_bytes!(r"../../../resources/crossbones.png"),
    native_size: gothic::ui::geometry::Size::new(16, 16),
    size: gothic::ui::geometry::Size::new(64, 64),
    cached_bitmap: None,
};

