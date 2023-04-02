use crate::game::player::Player;

pub struct GameWorld {
    pub player: Player,
}

impl GameWorld {
    pub fn new(player: Player) -> Self {
        Self { player }
    }
}

