use super::super::geometry::{
    hexagon::FractionalHex, hexagon::Hex, point::Point, rectangle::Rectangle,
};

use std::f32::consts::PI;

struct HexOrientation {
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
    fn pointy() -> HexOrientation {
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

    fn flat() -> HexOrientation {
        HexOrientation {
            f0: (3.0 as f32).sqrt() / 2.0,
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

enum LayoutOrientation {
    Pointy,
    Flat,
}

struct Layout {
    orientation: HexOrientation,
    size: Rectangle,
    origin: Point,
}

impl Layout {
    fn new(orientation: LayoutOrientation, size: Rectangle, origin: Point) -> Layout {
        let oriented = match orientation {
            LayoutOrientation::Pointy => HexOrientation::pointy(),
            LayoutOrientation::Flat => HexOrientation::flat(),
        };
        Layout {
            orientation: oriented,
            size: size,
            origin: origin,
        }
    }

    fn hex_to_pixel(&self, hex: Hex) -> Point {
        &self.origin
            + &Point::new(
                (self.orientation.f0 * hex.q as f32 + self.orientation.f1 * hex.r as f32)
                    * self.size.width,
                (self.orientation.f2 * hex.q as f32 + self.orientation.f3 * hex.r as f32)
                    * self.size.height,
            )
    }

    fn pixel_to_hex(&self, point: Point) -> FractionalHex {
        let pt = Point::new(
            (point.x - self.origin.x) / self.size.width,
            (point.y - self.origin.y) / self.size.height,
        );
        let q = pt.x * self.orientation.b0 + pt.y * self.orientation.b1;
        let r = pt.x * self.orientation.b2 + pt.y * self.orientation.b3;

        FractionalHex::new(q, r, -q - r)
    }

    fn hex_corner_offset(&self, corner: i32) -> Point {
        let angle = 2.0 * PI * (self.orientation.start_angle + corner as f32) / 6.0;

        Point::new(
            self.size.width * angle.cos(),
            self.size.height * angle.sin(),
        )
    }

    fn polygon_corners(&self, hex: Hex) -> Vec<Point> {
        let mut corners: Vec<Point> = Vec::with_capacity(6);
        let center = self.hex_to_pixel(hex);

        for i in 0..6 {
            let offset = self.hex_corner_offset(i);
            corners.push(&center + &offset);
        }

        corners
    }
}

// class HexMap {
//     constructor(type) {
//         READONLY(self, "map", {});
//     }

//     get(hex) {
//         DEBUG && ASSERT_INSTANCE_OF(hex, Hex);

//         return self.map[hex.hashCode()];
//     }

//     insert(hex) {
//         DEBUG && ASSERT_INSTANCE_OF(hex, Hex);

//         self.map[hex.hashCode()] = hex;
//     }

//     remove(hex) {
//         DEBUG && ASSERT_INSTANCE_OF(hex, Hex);

//         self.map.delete(hex.hashCode());
//     }

//     forEach(fn) {
//         const keys = Object.keys(self.map);

//         for (let i = 0; i < keys.length; ++i) {
//             const key = keys[i];
//             const hex = self.map[key];
//             fn(hex);
//         }
//     }

//     clear() {
//         const keys = Object.keys(self.map);

//         for (let i = 0; i < keys.length; ++i) {
//             const key = keys[i];
//             self.map.delete(key);
//         }
//     }

//     generateHexgon(radius) {
//         DEBUG && ASSERT_INTEGER(radius);

//         self.clear();

//         for (let q = -radius; q <= radius; ++q) {
//             const r1 = Math.max(-radius, -q - radius);
//             const r2 = Math.min(radius, -q + radius);

//             for (let r = r1; r <= r2; ++r) {
//                 self.insert(new Hex(q, r, -q - r));
//             }
//         }
//     }

//     generateRectangle(width, height) {
//         DEBUG && ASSERT_INTEGER(width);
//         DEBUG && ASSERT_INTEGER(height);

//         for (let r = 0; r < height; ++r) {
//             rOffset = Math.floor(r / 2);
//             for (let q = -rOffset; q < width - rOffset; ++q) {
//                 self.insert(new Hex(q, r, -q - r));
//             }
//         }
//     }
// }
