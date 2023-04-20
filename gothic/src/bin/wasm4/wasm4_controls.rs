use gothic::updatable::{Button, Controls};
use wasm4::gamepad::{Gamepad, GamepadButton};
use wasm4::inputs::Inputs;

pub struct Wasm4Controls<'a> {
    button_x: Wasm4Button<'a>,
    button_y: Wasm4Button<'a>,
    arrow_left: Wasm4Button<'a>,
    arrow_top: Wasm4Button<'a>,
    arrow_right: Wasm4Button<'a>,
    arrow_down: Wasm4Button<'a>,
}

impl<'a> Wasm4Controls<'a> {
    pub fn new(inputs: &'a Inputs) -> Self {
        Self {
            button_x: Wasm4Button::new(&inputs.gamepad1, GamepadButton::ButtonX),
            button_y: Wasm4Button::new(&inputs.gamepad1, GamepadButton::ButtonY),
            arrow_left: Wasm4Button::new(&inputs.gamepad1, GamepadButton::DPadLeft),
            arrow_top: Wasm4Button::new(&inputs.gamepad1, GamepadButton::DPadUp),
            arrow_right: Wasm4Button::new(&inputs.gamepad1, GamepadButton::DPadRight),
            arrow_down: Wasm4Button::new(&inputs.gamepad1, GamepadButton::DPadDown),
        }
    }
}

impl<'a> Controls for Wasm4Controls<'a> {
    fn button_x(&self) -> &dyn Button {
        &self.button_x
    }

    fn button_y(&self) -> &dyn Button {
        &self.button_y
    }

    fn arrow_left(&self) -> &dyn Button {
        &self.arrow_left
    }

    fn arrow_top(&self) -> &dyn Button {
        &self.arrow_top
    }

    fn arrow_right(&self) -> &dyn Button {
        &self.arrow_right
    }

    fn arrow_down(&self) -> &dyn Button {
        &self.arrow_down
    }
}

struct Wasm4Button<'a> {
    gamepad: &'a Gamepad,
    button: GamepadButton,
}

impl<'a> Wasm4Button<'a> {
    pub fn new(gamepad: &'a Gamepad, button: GamepadButton) -> Self {
        Self { gamepad, button }
    }
}

impl<'a> Button for Wasm4Button<'a> {
    fn is_pressed(&self) -> bool {
        self.gamepad.is_held(self.button)
    }

    fn is_just_pressed(&self) -> bool {
        self.gamepad.is_pressed(self.button)
    }

    fn is_just_released(&self) -> bool {
        self.gamepad.is_released(self.button)
    }
}