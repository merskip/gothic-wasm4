use wasm4::framebuffer::{Framebuffer, PaletteIndex};
use wasm4::geometry::{Point, Rect};
use wasm4::inputs::Inputs;
use crate::dispatcher::Dispatcher;
use crate::game::game_world::GameWorld;
use crate::renderable::Renderable;
use crate::sprites::CHARACTER_SPRITE;
use crate::updatable::Updatable;

pub struct GameScene {
    game_world: GameWorld,
}

impl GameScene {
    pub fn new(game_world: GameWorld) -> Self {
        Self { game_world }
    }
}

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
        framebuffer.sprite(&CHARACTER_SPRITE, frame.origin + Point::new(
            (self.game_world.player.position.x - CHARACTER_SPRITE.size().width as f32 / 2.0) as i32,
            (self.game_world.player.position.y - CHARACTER_SPRITE.size().height as f32 / 2.0) as i32,
        ), );
    }
}
