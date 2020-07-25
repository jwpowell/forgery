use std::cmp;
use std::collections::HashMap;

use crate::cell::{Cell, CellCoords};
use crate::layout::Layout;

pub struct WorldMap<T: Cell, L: Layout<T = T>> {
    pub map: HashMap<CellCoords, T>,
    pub layout: L,
}

impl<T: Cell, L: Layout<T = T>> WorldMap<T, L> {
    pub fn new(layout: L) -> WorldMap<T, L> {
        WorldMap {
            map: HashMap::new(),
            layout: layout,
        }
    }

    pub fn generate_hexgon(&mut self, radius: i32) {
        self.map.clear();

        for q in -radius..=radius {
            let r1 = cmp::max(-radius, -q - radius);
            let r2 = cmp::min(radius, -q + radius);

            for r in r1..=r2 {
                let cell = T::new(q as f32, r as f32, (-q - r) as f32);
                self.map.insert(cell.coords(), cell);
            }
        }
    }
}
