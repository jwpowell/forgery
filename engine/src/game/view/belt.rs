use std::collections::HashMap;

use crate::engine::CellCoord;

#[derive(Debug, Clone)]
pub enum Material {
    Mat1,
    Mat2,
    Mat3,
    Mat4,
    Mat5,
}

#[derive(Debug, Clone)]
pub struct BeltView {
    pub contents: HashMap<CellCoord, Option<Material>>,
}

impl BeltView {
    pub fn new(contents: HashMap<CellCoord, Option<Material>>) -> BeltView {
        BeltView { contents }
    }

    pub fn material_at(&self, coord: &CellCoord) -> &Option<Material> {
        match self.contents.get(coord) {
            Some(v) => v,
            None => &None,
        }
    }
}
