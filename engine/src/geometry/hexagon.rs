// Based on: https://www.redblobgames.com/grids/hexagons/implementation.html

use std::ops;

const HEX_DIRECTIONS: &'static [(i32, i32, i32); 6] = &[
    (1, 0, -1),
    (1, -1, 0),
    (0, -1, 1),
    (-1, 0, 1),
    (-1, 1, 0),
    (0, 1, -1),
];

#[derive(Debug)]
pub struct FractionalHex {
    q: f32,
    r: f32,
    s: f32,
}

impl FractionalHex {
    pub fn new(q: f32, r: f32, s: f32) -> FractionalHex {
        if q + r + s != 0.0 {
            panic!("invalid fractional hex coordinates");
        }

        FractionalHex { q, r, s }
    }
}

impl ops::Add<FractionalHex> for FractionalHex {
    type Output = FractionalHex;

    fn add(self, rhs: FractionalHex) -> FractionalHex {
        FractionalHex::new(self.q + rhs.q, self.r + rhs.r, self.s + rhs.s)
    }
}

impl ops::Sub<FractionalHex> for FractionalHex {
    type Output = FractionalHex;

    fn sub(self, rhs: FractionalHex) -> FractionalHex {
        FractionalHex::new(self.q - rhs.q, self.r - rhs.r, self.s - rhs.s)
    }
}

impl ops::Mul<FractionalHex> for FractionalHex {
    type Output = FractionalHex;

    fn mul(self, rhs: FractionalHex) -> FractionalHex {
        FractionalHex::new(self.q * rhs.q, self.r * rhs.r, self.s * rhs.s)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl Hex {
    fn new(q: i32, r: i32, s: i32) -> Hex {
        if q + r + s != 0 {
            panic!("invalid hex coordinates");
        }

        Hex { q, r, s }
    }

    fn length(&self) -> i32 {
        (self.q.abs() + self.r.abs() + self.s.abs()) / 2
    }

    fn distance(&self, to: &Hex) -> i32 {
        (self - to).length()
    }

    fn direction(&self, direction: i32) -> Hex {
        if !(0 <= direction && direction < 6) {
            panic!("invalid hex direction");
        }

        let (q, r, s) = HEX_DIRECTIONS[direction as usize];
        Hex::new(q, r, s)
    }

    fn neighbor(&self, direction: i32) -> Hex {
        self + &self.direction(direction)
    }

    fn rotate_left(&self) -> Hex {
        Hex::new(-self.s, -self.q, -self.r)
    }

    fn rotate_right(&self) -> Hex {
        Hex::new(-self.r, -self.s, -self.q)
    }

    fn lerp(&self, rhs: &Hex, t: f32) -> FractionalHex {
        use super::math;

        FractionalHex::new(
            math::lerp(self.q as f32, rhs.q as f32, t),
            math::lerp(self.r as f32, rhs.r as f32, t),
            math::lerp(self.s as f32, rhs.s as f32, t),
        )
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
}

impl From<FractionalHex> for Hex {
    fn from(item: FractionalHex) -> Self {
        let mut q = item.q.round();
        let mut r = item.r.round();
        let mut s = item.s.round();

        let q_diff = (q - item.q).abs();
        let r_diff = (r - item.r).abs();
        let s_diff = (s - item.s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s;
        } else {
            s = -q - r;
        }

        Hex::new(q as i32, r as i32, s as i32)
    }
}

impl ops::Add<&Hex> for &Hex {
    type Output = Hex;

    fn add(self, rhs: &Hex) -> Hex {
        Hex::new(self.q + rhs.q, self.r + rhs.r, self.s + rhs.s)
    }
}

impl ops::Sub<&Hex> for &Hex {
    type Output = Hex;

    fn sub(self, rhs: &Hex) -> Hex {
        Hex::new(self.q - rhs.q, self.r - rhs.r, self.s - rhs.s)
    }
}

impl ops::Mul<&Hex> for &Hex {
    type Output = Hex;

    fn mul(self, rhs: &Hex) -> Hex {
        Hex::new(self.q * rhs.q, self.r * rhs.r, self.s * rhs.s)
    }
}
