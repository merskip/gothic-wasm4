use wasm4::framebuffer::PaletteIndex;
use wasm4::geometry::Point;

use crate::game::game_world::GameWorld;
use crate::game::player::Player;
use crate::renderable::{Renderable, RenderContext};
use crate::sprites::PLAYER_SPRITE;
use crate::updatable::{Updatable, UpdateContext};

pub fn make_game_scene() -> GameScene {
    let player = Player::new(Point::new(50.0, 50.0));
    let game_world = GameWorld::new(player);
    GameScene::new(game_world)
}

pub struct GameScene {
    game_world: GameWorld,
}

impl GameScene {
    pub fn new(game_world: GameWorld) -> Self {
        Self { game_world }
    }
}

impl Updatable for GameScene {
    fn update(&mut self, context: &mut UpdateContext) {
        self.game_world.player.update(context);
    }
}

impl Renderable for GameScene {
    fn render(&self, context: &mut RenderContext) {
        context.framebuffer.set_draw_colors([
            PaletteIndex::Palette2,
            PaletteIndex::Transparent,
            PaletteIndex::Transparent,
            PaletteIndex::Transparent,
        ]);
        context.framebuffer.text("Gothic", Point::new(0, 0));
        self.render_player(context);
    }
}

impl GameScene {
    fn render_player(&self, context: &mut RenderContext) {
        context.framebuffer.set_draw_colors([
            PaletteIndex::Palette2,
            PaletteIndex::Palette3,
            PaletteIndex::Palette4,
            PaletteIndex::Transparent,
        ]);
        context.framebuffer.sprite(PLAYER_SPRITE, context.frame.origin + Point::new(
            (self.game_world.player.position.x - PLAYER_SPRITE.size().width as f32 / 2.0) as i32,
            (self.game_world.player.position.y - PLAYER_SPRITE.size().height as f32 / 2.0) as i32,
        ), );
    }
}
