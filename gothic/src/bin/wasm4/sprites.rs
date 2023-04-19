use wasm4::sprite::Flags::*;
use wasm4::sprite::Sprite;

/// Player
pub const PLAYER_SPRITE: &Sprite = &Sprite::new(
    12,
    16,
    BLIT_2BPP,
    &[0xfe,0xaa,0xaf,0xf9,0x55,0x5b,0xe5,0x55,0x56,0x95,0x55,0x56,0x95,0x00,0x56,0xe0,0x82,0x0b,0xf8,0x82,0x2b,0xf8,0x00,0xaf,0xea,0x82,0xaf,0xa0,0x00,0x2a,0x82,0x00,0x82,0xea,0x00,0xab,0xfe,0x00,0xbf,0xfe,0x00,0xbf,0xfe,0x08,0xbf,0xfe,0xaa,0xbf],
);

/// King_Rhobar_2
pub const KING__RHOBAR_2_SPRITE: &Sprite = &Sprite::new(
    16,
    18,
    BLIT_1BPP,
    &[0x0d,0xb0,0x0a,0x50,0x18,0x18,0x38,0x1c,0x7f,0xfe,0x67,0xe6,0x60,0x06,0x6c,0x36,0x66,0x66,0x64,0x26,0x60,0x06,0x33,0xcc,0x4e,0x72,0x4d,0xb2,0x3d,0xbc,0x11,0x88,0x1f,0xf8,0x0e,0x70],
);

/// Orc
pub const ORC_SPRITE: &Sprite = &Sprite::new(
    24,
    22,
    BLIT_2BPP,
    &[0xaa,0xaa,0xea,0xaa,0xea,0xaa,0xaa,0xab,0x3f,0xff,0x3a,0xaa,0xaa,0xab,0x30,0x03,0x3a,0xaa,0xaa,0xaa,0xc0,0x00,0xea,0xaa,0xaa,0xaa,0xc0,0x00,0xea,0xaa,0xaa,0xaa,0xc0,0xcc,0xea,0xaa,0xaa,0xaf,0xc0,0x00,0x3a,0xaa,0xaa,0xb5,0xc3,0xff,0x3e,0xaa,0xaa,0xd5,0xc0,0x00,0x37,0xaa,0xab,0x55,0x70,0x00,0xd5,0xea,0xab,0x55,0x5c,0x03,0x15,0xea,0xab,0x15,0x50,0x00,0x15,0xea,0xac,0x05,0x40,0x00,0x14,0x3a,0xac,0x00,0x00,0x00,0x30,0x3a,0xb0,0x00,0xc0,0x00,0x30,0x0e,0xb0,0x03,0x00,0x00,0x30,0x0e,0xb0,0x0f,0x00,0x00,0x30,0x0e,0xb1,0x5f,0xc0,0x00,0x00,0x3a,0xb5,0x53,0xb0,0x03,0xc0,0x3a,0xad,0x03,0xb3,0xf3,0xbf,0xea,0xab,0xfe,0xb3,0xb3,0xaa,0xaa,0xaa,0xaa,0xbf,0xbf,0xaa,0xaa],
);

/// Crossbones
pub const CROSSBONES_SPRITE: &Sprite = &Sprite::new(
    16,
    16,
    BLIT_1BPP,
    &[0x1c,0x38,0x22,0x44,0x72,0x4e,0x94,0x29,0x82,0x41,0x91,0x89,0x68,0x96,0x04,0x60,0x06,0x20,0x6b,0x16,0x91,0x89,0x82,0x41,0x94,0x29,0x72,0x4e,0x22,0x44,0x1c,0x38],
);
