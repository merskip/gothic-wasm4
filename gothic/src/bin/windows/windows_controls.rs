use gothic::updatable::{Button, Controls};

pub struct WindowsControls {
    button_x: WindowsButton,
    button_y: WindowsButton,
    arrow_left: WindowsButton,
    arrow_top: WindowsButton,
    arrow_right: WindowsButton,
    arrow_down: WindowsButton,
}

impl WindowsControls {
    pub fn new() -> Self {
        Self {
            button_x: WindowsButton,
            button_y: WindowsButton,
            arrow_left: WindowsButton,
            arrow_top: WindowsButton,
            arrow_right: WindowsButton,
            arrow_down: WindowsButton,
        }
    }
}


impl Controls for WindowsControls {
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

pub struct WindowsButton;

impl Button for WindowsButton {
    fn is_pressed(&self) -> bool {
        false
    }

    fn is_just_pressed(&self) -> bool {
        false
    }

    fn is_just_released(&self) -> bool {
        false
    }
}