use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VIRTUAL_KEY, VK_DOWN, VK_LEFT, VK_RIGHT, VK_UP, VK_X, VK_Z};
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
            button_x: WindowsButton::new(VK_X),
            button_y: WindowsButton::new(VK_Z),
            arrow_left: WindowsButton::new(VK_LEFT),
            arrow_top: WindowsButton::new(VK_UP),
            arrow_right: WindowsButton::new(VK_RIGHT),
            arrow_down: WindowsButton::new(VK_DOWN),
        }
    }

    pub fn late_update(&mut self) {
        self.button_x.late_update();
        self.button_y.late_update();
        self.arrow_left.late_update();
        self.arrow_top.late_update();
        self.arrow_right.late_update();
        self.arrow_down.late_update();
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

pub struct WindowsButton {
    virtual_key: VIRTUAL_KEY,
    last_pressed: bool,
}

impl WindowsButton {
    pub fn new(virtual_key: VIRTUAL_KEY) -> Self {
        Self { virtual_key, last_pressed: false }
    }

    pub fn is_pressed(&self) -> bool {
        let state = unsafe { GetKeyState(self.virtual_key.0 as i32) } as u16;
        (state & 0x8000) != 0
    }

    pub fn late_update(&mut self) {
        self.last_pressed = self.is_pressed()
    }
}

impl Button for WindowsButton {
    fn is_pressed(&self) -> bool {
        self.is_pressed()
    }

    fn is_just_pressed(&self) -> bool {
        !self.last_pressed && self.is_pressed()
    }

    fn is_just_released(&self) -> bool {
        self.last_pressed && !self.is_pressed()
    }
}