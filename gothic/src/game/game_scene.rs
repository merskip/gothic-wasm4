use wasm4::framebuffer::Framebuffer;
use wasm4::geometry::{Point, Rect, Size};
use wasm4::inputs::Inputs;
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

const PLAYER_SIZE: Size<u32> = Size::new(10, 10);

impl Updatable for GameScene {
    fn update(&mut self, inputs: &Inputs, dispatcher: &mut Dispatcher) {
        self.game_world.player.update(inputs, dispatcher);
    }
}

impl Renderable for GameScene {
    fn render(&self, framebuffer: &Framebuffer, frame: Rect) {
        self.render_player(framebuffer, frame);
    }
}

impl GameScene {
    fn render_player(&self, framebuffer: &Framebuffer, frame: Rect) {
        framebuffer.rectangle(
            frame.origin + Point::new(
                (self.game_world.player.position.x - PLAYER_SIZE.width as f32 / 2.0) as i32,
                (self.game_world.player.position.y - PLAYER_SIZE.width as f32 / 2.0) as i32,
            ),
            PLAYER_SIZE,
        );
    }
}