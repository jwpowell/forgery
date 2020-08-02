use std::cell::{Ref, RefCell};
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::error::Error;
use std::fmt;

use crate::engine;
use crate::engine::{
    debug, Cell, CellCoord, Hex, HexLayout, HexOrientation, Layout, Point, Rectangle, World,
};

use super::belt::BeltView;
use super::building::{BuildingState, BuildingView};

use web_sys::{Document, Element, Event, MouseEvent, SvgElement, SvgsvgElement};

// We cannot have mutable statics by default so we use this to enable it.
thread_local! {
    pub static WORLD: RefCell<World<Hex, HexLayout>> = RefCell::new(
        create_hex_world()
    );
}

fn create_hex_world() -> World<Hex, HexLayout> {
    let hex_layout = HexLayout::new(
        HexOrientation::flat(),
        Rectangle::new(10.0, 10.0),
        Point::new(0.0, 0.0),
    );
    let hex_world = World::new(hex_layout);

    hex_world
}

#[derive(Debug)]
pub struct GameStateView {
    buildings_map: HashMap<CellCoord, BuildingView>,
    belts_map: HashMap<CellCoord, Vec<BeltView>>,
    pub collision_set: HashSet<CellCoord>,
}

impl GameStateView {
    pub fn new(buildings: &[BuildingView], belts: &[BeltView]) -> GameStateView {
        let mut buildings_map: HashMap<CellCoord, BuildingView> = HashMap::new();
        let mut belts_map: HashMap<CellCoord, Vec<BeltView>> = HashMap::new();
        let mut collision_set: HashSet<CellCoord> = HashSet::new();

        for building in buildings {
            buildings_map.insert(building.coord.clone(), building.clone());
            collision_set.insert(building.coord.clone());
        }

        for belt in belts {
            for (coord, _) in &belt.contents {
                let cell_belts = belts_map.entry(coord.clone()).or_insert(Vec::new());
                cell_belts.push(belt.clone());
                collision_set.insert(coord.clone());
            }
        }

        GameStateView {
            buildings_map,
            belts_map,
            collision_set,
        }
    }

    pub fn belts_at(&self, coord: &CellCoord) -> Option<&[BeltView]> {
        if let Some(v) = self.belts_map.get(&coord) {
            return Some(v.as_slice());
        }
        return None;
    }

    pub fn building_at(&self, coord: &CellCoord) -> Option<&BuildingView> {
        if let Some(v) = self.buildings_map.get(&coord) {
            return Some(v);
        }
        return None;
    }
}

// pub fn generate_hexgon(&mut self, radius: i32) {
//     self.cells.clear();

//     for q in -radius..=radius {
//         let r1 = cmp::max(-radius, -q - radius);
//         let r2 = cmp::min(radius, -q + radius);

//         for r in r1..=r2 {
//             let cell = C::new(q as f32, r as f32, (-q - r) as f32);
//             self.cells.insert(cell.coord(), cell);
//         }
//     }
// }
