use crate::ui::geometry::Vector;
use crate::updatable::{Controls, Updatable, UpdateContext};

pub struct Player {
    pub position: Vector,
}

impl Player {
    pub fn new(position: Vector) -> Self {
        Self { position }
    }
}

const MOVE_SPEED: f32 = 1.0;

impl Updatable for Player {
    fn update(&mut self, context: &mut UpdateContext) {
        self.position += self.get_movement(context.controls);
    }
}

impl Player {
    fn get_movement(&self, controls: &dyn Controls) -> Vector {
        let mut movement = Vector::new(0.0, 0.0);
        if controls.arrow_left().is_pressed() {
            movement.x -= MOVE_SPEED;
        }
        if controls.arrow_top().is_pressed() {
            movement.y -= MOVE_SPEED;
        }
        if controls.arrow_right().is_pressed() {
            movement.x += MOVE_SPEED;
        }
        if controls.arrow_down().is_pressed() {
            movement.y += MOVE_SPEED;
        }
        return movement.normalized();
    }
}
