use std::ops;

const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    fn origin() -> Point {
        ORIGIN
    }

    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

impl ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul<f32> for &Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}
