use std::collections::HashMap;

use crate::engine::{CellCoord, rng};


#[derive(Debug, Clone)]
pub enum Material {
    Mat1,
    Mat2,
    Mat3,
    Mat4,
    Mat5,
}

#[derive(Debug, Clone)]
pub struct Belt {
    pub id: u32,
    pub contents: HashMap<CellCoord, Option<Material>>,
}

impl Belt {
    pub fn new(contents: HashMap<CellCoord, Option<Material>>) -> Belt {
        Belt {
            id: rng::uid(),
            contents: contents,
        }
    }

    pub fn material_at(&self, coord: &CellCoord) -> &Option<Material> {
        match self.contents.get(coord) {
            Some(v) => v,
            None => &None,
        }
    }
}
