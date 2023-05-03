use core::ops::Add;

use crate::ui::geometry::{Point, Size};

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    pub fn centered(&self, size: Size) -> Self {
        Self {
            origin: Point::new(
                self.origin.x + ((self.size.width - size.width) / 2) as i32,
                self.origin.y + ((self.size.height - size.height) / 2) as i32,
            ),
            size,
        }
    }
}

impl Rect {
    pub fn min_x(&self) -> i32 {
        self.origin.x
    }

    pub fn max_x(&self) -> i32 {
        self.origin.x + self.size.width as i32
    }

    pub fn min_y(&self) -> i32 {
        self.origin.y
    }

    pub fn max_y(&self) -> i32 {
        self.origin.y + self.size.height as i32
    }
}

impl Add<Point> for Rect {
    type Output = Rect;

    fn add(self, rhs: Point) -> Self::Output {
        Rect::new(self.origin + rhs, self.size)
    }
}