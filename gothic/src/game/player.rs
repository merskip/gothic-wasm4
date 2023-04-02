use wasm4::gamepad::GamepadButton::{DPadDown, DPadLeft, DPadRight, DPadUp};
use wasm4::geometry::Point;
use wasm4::inputs::Inputs;
use crate::dispatcher::Dispatcher;
use crate::updatable::Updatable;

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
    fn update(&mut self, inputs: &Inputs, _dispatcher: &mut Dispatcher) {
        if inputs.gamepad1.is_held(DPadLeft) {
            self.position.x -= MOVE_SPEED;
        }
        if inputs.gamepad1.is_held(DPadUp) {
            self.position.y -= MOVE_SPEED;
        }
        if inputs.gamepad1.is_held(DPadRight) {
            self.position.x += MOVE_SPEED;
        }
        if inputs.gamepad1.is_held(DPadDown) {
            self.position.y += MOVE_SPEED;
        }
    }
}
