use std::f32::consts::PI;

use crate::cell::{Cell, Hex, Point, Rectangle};

pub trait Layout {
    type T: Cell;
    fn cell_to_pixel(&self, cell: &Self::T) -> Point;
    fn pixel_to_cell(&self, point: &Point) -> Self::T;
    fn cell_corner_offset(&self, corner: i32) -> Point;
    fn polygon_corners(&self, cell: &Self::T) -> Vec<Point>;
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
    type T = Hex;
    fn cell_to_pixel(&self, hex: &Hex) -> Point {
        &self.origin
            + &Point::new(
                (self.orientation.f0 * hex.q + self.orientation.f1 * hex.r) * self.size.width,
                (self.orientation.f2 * hex.q + self.orientation.f3 * hex.r) * self.size.height,
            )
    }

    fn pixel_to_cell(&self, point: &Point) -> Hex {
        let pt = Point::new(
            (point.x - self.origin.x) / self.size.width,
            (point.y - self.origin.y) / self.size.height,
        );
        let q = pt.x * self.orientation.b0 + pt.y * self.orientation.b1;
        let r = pt.x * self.orientation.b2 + pt.y * self.orientation.b3;

        Hex::new(q, r, -q - r)
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
        let center = self.cell_to_pixel(&cell);

        for i in 0..6 {
            let offset = self.cell_corner_offset(i);
            corners.push(&center + &offset);
        }

        corners
    }
}
