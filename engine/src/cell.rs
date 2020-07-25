// Based on: https://www.redblobgames.com/grids/hexagons/implementation.html

use std::marker::Sized;
use std::ops::{Add, Mul, Sub};

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub trait Cell: Sized {
    fn new(x: f32, y: f32, z: f32) -> Self;
    fn length(&self) -> i32;
    fn distance(&self, to: &Self) -> i32;
    fn direction(&self, direction: i32) -> Self;
    fn neighbor(&self, direction: i32) -> Self;
    fn lerp(&self, rhs: &Self, t: f32) -> Self;
    fn round(&self) -> Self;
    fn linedraw(&self, to: &Self) -> Vec<Self>;
    fn coords(&self) -> CellCoords;
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub struct CellCoords {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<CellCoords> for String {
    fn from(coords: CellCoords) -> Self {
        format!("{},{},{}", coords.x, coords.y, coords.z)
    }
}

const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn origin() -> Point {
        ORIGIN
    }

    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for &Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl From<Point> for String {
    fn from(point: Point) -> Self {
        format!("{},{}", point.x, point.y)
    }
}

const HEX_DIRECTIONS: &'static [(f32, f32, f32); 6] = &[
    (1.0, 0.0, -1.0),
    (1.0, -1.0, 0.0),
    (0.0, -1.0, 1.0),
    (-1.0, 0.0, 1.0),
    (-1.0, 1.0, 0.0),
    (0.0, 1.0, -1.0),
];

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Hex {
    pub q: f32,
    pub r: f32,
    pub s: f32,
}

impl Hex {
    // fn rotate_left(&self) -> Hex {
    //     Hex::new(-self.s, -self.q, -self.r)
    // }

    // fn rotate_right(&self) -> Hex {
    //     Hex::new(-self.r, -self.s, -self.q)
    // }
}

impl Cell for Hex {
    fn new(x: f32, y: f32, z: f32) -> Hex {
        if x + y + z != 0.0 {
            panic!("invalid hex coordinates");
        }

        Hex { q: x, r: y, s: z }
    }

    fn length(&self) -> i32 {
        ((self.q.abs() + self.r.abs() + self.s.abs()) / 2.0) as i32
    }

    fn distance(&self, to: &Self) -> i32 {
        (self - to).length()
    }

    fn direction(&self, direction: i32) -> Hex {
        if !(0 <= direction && direction < 6) {
            panic!("invalid hex direction");
        }

        let (q, r, s) = HEX_DIRECTIONS[direction as usize];
        Hex::new(q, r, s)
    }

    fn neighbor(&self, direction: i32) -> Self {
        self + &self.direction(direction)
    }

    fn lerp(&self, rhs: &Hex, t: f32) -> Hex {
        Hex::new(
            lerp(self.q, rhs.q, t),
            lerp(self.r, rhs.r, t),
            lerp(self.s, rhs.s, t),
        )
    }

    fn round(&self) -> Hex {
        let mut q = self.q.round();
        let mut r = self.r.round();
        let mut s = self.s.round();

        let q_diff = (q - self.q).abs();
        let r_diff = (r - self.r).abs();
        let s_diff = (s - self.s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s;
        } else {
            s = -q - r;
        }

        Hex::new(q as i32 as f32, r as i32 as f32, s as i32 as f32)
    }

    fn linedraw(&self, rhs: &Hex) -> Vec<Hex> {
        use std::cmp;

        let n = self.distance(rhs);
        let step = 1.0 / cmp::max(n, 1) as f32;

        let mut results: Vec<Hex> = Vec::new();
        for i in 0..=n {
            let vertex = self.lerp(rhs, step * i as f32);
            results.push(vertex.into());
        }

        results
    }

    fn coords(&self) -> CellCoords {
        CellCoords {
            x: self.q as i32,
            y: self.r as i32,
            z: self.s as i32,
        }
    }
}

impl Add<&Hex> for &Hex {
    type Output = Hex;

    fn add(self, rhs: &Hex) -> Hex {
        Hex::new(self.q + rhs.q, self.r + rhs.r, self.s + rhs.s)
    }
}

impl Sub<&Hex> for &Hex {
    type Output = Hex;

    fn sub(self, rhs: &Hex) -> Hex {
        Hex::new(self.q - rhs.q, self.r - rhs.r, self.s - rhs.s)
    }
}

impl Mul<&Hex> for &Hex {
    type Output = Hex;

    fn mul(self, rhs: &Hex) -> Hex {
        Hex::new(self.q * rhs.q, self.r * rhs.r, self.s * rhs.s)
    }
}

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Rectangle {
        Rectangle { width, height }
    }
}
