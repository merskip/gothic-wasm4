use crate::dialogue::Dialogue;
use crate::game::dialogue::dialogue_diego_first_meet::DIALOGUE_DIEGO_FIRST_MEET;
use crate::game::dialogue::dialogue_intro::DIALOGUE_INTRO;
use crate::game::game_world::GameWorld;
use crate::game::player::Player;
use crate::image_asset::ImageAsset;
use crate::renderable::{Color, Renderable, RenderContext};
use crate::system::get_image;
use crate::ui::dialogue::dialogue_overlay::DialogueOverlay;
use crate::ui::geometry::{Point, Vector};
use crate::updatable::{Updatable, UpdateContext};

pub fn make_game_scene() -> GameScene {
    let player = Player::new(Vector::new(100.0, 100.0));
    let game_world = GameWorld::new(player);
    GameScene::new(game_world)
}

pub struct GameScene {
    game_world: GameWorld,
    dialogue_overlay: Option<DialogueOverlay>,
}

impl GameScene {
    pub fn new(game_world: GameWorld) -> Self {
        Self {
            game_world,
            dialogue_overlay: Some(DialogueOverlay::new(&DIALOGUE_INTRO)),
        }
    }
}

impl Updatable for GameScene {
    fn update(&mut self, context: &mut UpdateContext) {
        if let Some(dialogue_overlay) = &mut self.dialogue_overlay {
            dialogue_overlay.update(context);

            if dialogue_overlay.finished() {
                if dialogue_overlay.dialogue() as *const Dialogue == &DIALOGUE_INTRO as *const Dialogue {
                    self.dialogue_overlay = Some(DialogueOverlay::new(&DIALOGUE_DIEGO_FIRST_MEET));
                } else {
                    self.dialogue_overlay = None;
                }
            }
        } else {
            self.game_world.player.update(context);
        }
    }
}

impl Renderable for GameScene {
    fn render(&self, context: &mut RenderContext) {
        self.render_player(context);
    }
}

impl GameScene {
    fn render_player(&self, context: &mut RenderContext) {
        let player_image = get_image(ImageAsset::Player);
        let position = context.frame.origin + Point::new(
            (self.game_world.player.position.x - (player_image.size().width as f32) / 2.0) as i32,
            (self.game_world.player.position.y - (player_image.size().height as f32) / 2.0) as i32,
        );
        context.canvas.set_image_colors([
            Color::Primary,
            Color::Secondary,
            Color::Tertiary,
            Color::Transparent,
        ]);
        context.canvas.draw_image(player_image, position);

        if let Some(dialogue_overlay) = &self.dialogue_overlay {
            dialogue_overlay.render(context);
        }
    }
}
