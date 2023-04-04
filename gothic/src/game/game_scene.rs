use wasm4::framebuffer::{Framebuffer, PaletteIndex};
use wasm4::geometry::{Point, Rect, Size};
use wasm4::inputs::Inputs;
use wasm4::sprite::Sprite;
use crate::dispatcher::Dispatcher;
use crate::game::game_world::GameWorld;
use crate::renderable::Renderable;
use crate::updatable::Updatable;

pub struct GameScene {
    game_world: GameWorld,
}

impl GameScene {
    pub fn new(game_world: GameWorld) -> Self {
        Self { game_world }
    }
}

const CHARACTER_SPRITE: Sprite = Sprite::new(
    Size::new(12, 16),
    1, // BLIT_2BPP
    &[
        0xfe, 0xaa, 0xaf, 0xf9, 0x55, 0x5b, 0xe5, 0x55,
        0x56, 0x95, 0x55, 0x56, 0x95, 0x00, 0x56, 0xe0,
        0x82, 0x0b, 0xf8, 0x82, 0x2b, 0xf8, 0x00, 0xaf,
        0xea, 0x82, 0xaf, 0xa0, 0x00, 0x2a, 0x82, 0x00,
        0x82, 0xea, 0x00, 0xab, 0xfe, 0x00, 0xbf, 0xfe,
        0x00, 0xbf, 0xfe, 0x08, 0xbf, 0xfe, 0xaa, 0xbf
    ],
);

impl Updatable for GameScene {
    fn update(&mut self, inputs: &Inputs, dispatcher: &mut Dispatcher) {
        self.game_world.player.update(inputs, dispatcher);
    }
}

impl Renderable for GameScene {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect) {
        framebuffer.set_draw_colors([
            PaletteIndex::Palette2,
            PaletteIndex::Transparent,
            PaletteIndex::Transparent,
            PaletteIndex::Transparent,
        ]);
        framebuffer.text("Gothic", Point::new(0, 0));
        self.render_player(framebuffer, frame);
    }
}

impl GameScene {
    fn render_player(&self, framebuffer: &Framebuffer, frame: Rect) {
        framebuffer.set_draw_colors([
            PaletteIndex::Palette2,
            PaletteIndex::Palette3,
            PaletteIndex::Palette4,
            PaletteIndex::Transparent,
        ]);
        framebuffer.sprite(CHARACTER_SPRITE, frame.origin + Point::new(
            (self.game_world.player.position.x - CHARACTER_SPRITE.size().width as f32 / 2.0) as i32,
            (self.game_world.player.position.y - CHARACTER_SPRITE.size().height as f32 / 2.0) as i32,
        ), );
    }
}
