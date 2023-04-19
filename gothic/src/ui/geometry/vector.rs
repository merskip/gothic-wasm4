use core::ops::{Add, AddAssign};
use libm::sqrtf;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Vector {
    /// Returns the length of this vector
    pub fn magnitude(&self) -> f32 {
        sqrtf(self.x * self.x + self.y * self.y)
    }

    pub fn normalized(&self) -> Self {
        let length = self.magnitude();
        if length == 0.0 {
            return *self;
        }
        let inverted_length = 1.0 / self.magnitude();
        Vector {
            x: self.x * inverted_length,
            y: self.y * inverted_length,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}