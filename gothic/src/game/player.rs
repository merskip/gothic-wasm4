use wasm4::gamepad::GamepadButton::{DPadDown, DPadLeft, DPadRight, DPadUp};
use wasm4::geometry::{Point, Vector};
use wasm4::inputs::Inputs;

use crate::updatable::{Updatable, UpdateContext};

pub struct Player {
    pub position: Point<f32>,
}

impl Player {
    pub fn new(position: Point<f32>) -> Self {
        Self { position }
    }
}

const MOVE_SPEED: f32 = 1.0;

impl Updatable for Player {
    fn update(&mut self, context: &mut UpdateContext) {
        self.position += self.get_movement(context.inputs);
    }
}

impl Player {
    fn get_movement(&self, inputs: &Inputs) -> Vector<f32> {
        let mut movement = Vector::new(0.0, 0.0);
        if inputs.gamepad1.is_held(DPadLeft) {
            movement.x -= MOVE_SPEED;
        }
        if inputs.gamepad1.is_held(DPadUp) {
            movement.y -= MOVE_SPEED;
        }
        if inputs.gamepad1.is_held(DPadRight) {
            movement.x += MOVE_SPEED;
        }
        if inputs.gamepad1.is_held(DPadDown) {
            movement.y += MOVE_SPEED;
        }
        return movement.normalized();
    }
}
