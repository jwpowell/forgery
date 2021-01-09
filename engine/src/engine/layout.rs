use std::f32::consts::PI;

use super::cell::{Cell, CellCoord, Hex};
use std::ops::{Add, Div, Mul, Sub};

pub trait Layout {
    type C: Cell;
    fn cell_from_coord(&self, coord: &CellCoord) -> Self::C;
    fn cell_to_pixel(&self, cell: &Self::C) -> Point;
    fn pixel_from_coord(&self, coord: &CellCoord) -> Point;
    fn pixel_to_cell(&self, point: &Point) -> Self::C;
    fn cell_corner_offset(&self, corner: i32) -> Point;
    fn polygon_corners(&self, cell: &Self::C) -> Vec<Point>;
    fn polygon_edge_center(&self, cell: &Self::C, direction: i32) -> Point;
    fn origin(&self) -> &Point;
}

pub struct HexOrientation {
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    b3: f32,
    start_angle: f32, // In multiples of 60 degrees.
}

impl HexOrientation {
    pub fn pointy() -> HexOrientation {
        HexOrientation {
            f0: (3.0 as f32).sqrt(),
            f1: (3.0 as f32).sqrt() / 2.0,
            f2: 0.0,
            f3: 3.0 / 2.0,
            b0: (3.0 as f32).sqrt() / 3.0,
            b1: -1.0 / 3.0,
            b2: 0.0,
            b3: 2.0 / 3.0,
            start_angle: 0.5, // In multiples of 60 degrees.
        }
    }

    pub fn flat() -> HexOrientation {
        HexOrientation {
            f0: 3.0 / 2.0,
            f1: 0.0,
            f2: (3.0 as f32).sqrt() / 2.0,
            f3: (3.0 as f32).sqrt(),
            b0: 2.0 / 3.0,
            b1: 0.0,
            b2: -1.0 / 3.0,
            b3: (3.0 as f32).sqrt() / 3.0,
            start_angle: 0.0, // In multiples of 60 degrees.
        }
    }
}

const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

impl Div<f32> for &Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Point {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

impl From<&Point> for String {
    fn from(point: &Point) -> Self {
        format!("{},{}", point.x, point.y)
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

pub struct HexLayout {
    orientation: HexOrientation,
    size: Rectangle,
    origin: Point,
}

impl HexLayout {
    pub fn new(orientation: HexOrientation, size: Rectangle, origin: Point) -> HexLayout {
        HexLayout {
            orientation: orientation,
            size: size,
            origin: origin,
        }
    }
}

impl Layout for HexLayout {
    type C = Hex;

    fn cell_from_coord(&self, coord: &CellCoord) -> Self::C {
        Hex::new(coord.x as f32, coord.y as f32, coord.z as f32)
    }

    fn cell_to_pixel(&self, hex: &Hex) -> Point {
        &self.origin
            + &Point::new(
                (self.orientation.f0 * hex.q + self.orientation.f1 * hex.r) * self.size.width,
                (self.orientation.f2 * hex.q + self.orientation.f3 * hex.r) * self.size.height,
            )
    }

    fn pixel_from_coord(&self, coord: &CellCoord) -> Point {
        self.cell_to_pixel(&self.cell_from_coord(coord))
    }

    fn pixel_to_cell(&self, point: &Point) -> Hex {
        let pt = Point::new(
            (point.x - self.origin.x) / self.size.width,
            (point.y - self.origin.y) / self.size.height,
        );
        let q = pt.x * self.orientation.b0 + pt.y * self.orientation.b1;
        let r = pt.x * self.orientation.b2 + pt.y * self.orientation.b3;

        Hex::new(q, r, -q - r).round()
    }

    fn cell_corner_offset(&self, corner: i32) -> Point {
        let angle = 2.0 * PI * (self.orientation.start_angle + corner as f32) / 6.0;

        Point::new(
            self.size.width * angle.cos(),
            self.size.height * angle.sin(),
        )
    }

    fn polygon_corners(&self, cell: &Hex) -> Vec<Point> {
        let mut corners: Vec<Point> = Vec::with_capacity(6);
        let center = self.origin(); // self.cell_to_pixel(&cell); // To use cell_to_pixel instead, you must not use "transform" on the cells.

        for i in 0..6 {
            let offset = self.cell_corner_offset(i);
            corners.push(center + &offset);
        }

        corners
    }

    fn polygon_edge_center(&self, cell: &Hex, direction: i32) -> Point {
        let neighbor_cell = cell.neighbor(direction);
        let neighbor_center = &self.cell_to_pixel(&neighbor_cell);
        let cell_center = &self.cell_to_pixel(cell);

        &(cell_center - neighbor_center) / 2.0
    }

    fn origin(&self) -> &Point {
        &self.origin
    }
}
