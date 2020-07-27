use std::collections::{HashMap, HashSet};

use crate::cell::CellCoord;

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

#[derive(Debug, Clone)]
pub enum BuildingState {
    // One or more belt nodes are not connected to a belt.
    Disconnected,
    // The internal storage for the input belts are not completely full, and the ones that aren't full have materials available on the belt.
    Transferring,
    // The internal storage for the input belts are not completely full, and at least one of the ones that aren't have empty belts.
    Starved,
    // The internal storage for the input belts is full and the internal storage for the output belts are not.
    Working,
    // The internal storage for the output belts are not completely empty and their belts are full.
    Blocked,
    // The building is manually disabled.
    Disabled,
}

impl From<&BuildingState> for String {
    fn from(state: &BuildingState) -> Self {
        format!("{:?}", state)
    }
}

#[derive(Debug, Clone)]
pub struct BuildingView {
    pub coord: CellCoord,
    pub state: BuildingState,
    nodes: HashMap<CellCoord, bool>,
}

impl BuildingView {
    pub fn new(
        coord: CellCoord,
        nodes: HashMap<CellCoord, bool>,
        state: BuildingState,
    ) -> BuildingView {
        BuildingView {
            coord,
            state,
            nodes,
        }
    }

    pub fn connect(&mut self, coord: &CellCoord) {
        if let Some(v) = self.nodes.get_mut(&coord) {
            *v = true;
        }
    }

    pub fn disconnect(&mut self, coord: &CellCoord) {
        if let Some(v) = self.nodes.get_mut(&coord) {
            *v = false;
        }
    }

    pub fn is_connected(&self, coord: &CellCoord) -> bool {
        if let Some(_) = self.nodes.get(&coord) {
            return true;
        }
        return false;
    }
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
