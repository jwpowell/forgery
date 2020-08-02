use std::collections::HashMap;

use crate::engine::CellCoord;

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
