use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use crate::engine::{CellCoord, Hex, HexLayout, HexOrientation, Point, Rectangle, World};

use super::belt::Belt;
use super::building::Building;
use super::map_hex::hex_map;

// We cannot have mutable statics by default so we use this to enable it.
thread_local! {
    pub static WORLD: RefCell<World<Hex, HexLayout>> = RefCell::new(
        create_hex_world()
    );

    pub static GAME_STATE: RefCell<GameState> = RefCell::new(GameState::new(load_world_map("map_hex")));
}

fn load_world_map(map_name: &str) -> HashSet<CellCoord> {
    hex_map()
    /*
    let input_path = format!("{}/examples/{}.ron", env!("CARGO_MANIFEST_DIR"), map_name);
    let f = File::open(&input_path).expect("Failed opening file");
    match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load world map ({}): {}", map_name, e);
            HashSet::new()
        }
    }
    */
}

fn create_hex_world() -> World<Hex, HexLayout> {
    let hex_layout = HexLayout::new(
        HexOrientation::flat(),
        Rectangle::new(10.0, 10.0),
        Point::new(0.0, 0.0),
    );
    let hex_world =
        World::new("workspace", hex_layout, 800, 700).expect("failed to create new world");

    hex_world
}

#[derive(Debug)]
pub enum UserAction {
    PlacingBelt {
        begin: CellCoord,
        end: Option<CellCoord>,
        previous_check: CellCoord,
    },
}

#[derive(Debug)]
pub struct GameState {
    pub world: HashSet<CellCoord>,
    pub buildings: HashMap<CellCoord, Building>,
    pub belts: HashMap<CellCoord, Vec<Belt>>,
    pub current_action: Option<UserAction>,
}

impl GameState {
    pub fn new(world: HashSet<CellCoord>) -> GameState {
        GameState {
            world: world,
            buildings: HashMap::new(),
            belts: HashMap::new(),
            current_action: None,
        }
    }

    pub fn add_building(&mut self, building: Building) {
        self.buildings.insert(building.coord.clone(), building);
    }

    pub fn remove_building(&mut self, coord: &CellCoord) {
        self.buildings.remove(coord);
    }

    pub fn add_belt(&mut self, belt: Belt) {
        for (coord, _) in &belt.contents {
            let cell_belts = self.belts.entry(coord.clone()).or_insert(Vec::new());
            cell_belts.push(belt.clone());
        }
    }

    pub fn remove_belt(&mut self, belt_id: u32) {
        for (_, cell_belts) in &mut self.belts {
            cell_belts.retain(|belt| belt.id != belt_id);
        }
    }

    pub fn collisions(&self) -> HashSet<CellCoord> {
        let mut collision_set: HashSet<CellCoord> = HashSet::new();
        for (coord, _) in &self.buildings {
            collision_set.insert(coord.clone());
        }
        for (coord, _) in &self.belts {
            collision_set.insert(coord.clone());
        }

        collision_set
    }

    pub fn belts_at(&self, coord: &CellCoord) -> Option<&[Belt]> {
        if let Some(v) = self.belts.get(coord) {
            return Some(v.as_slice());
        }
        return None;
    }

    pub fn building_at(&self, coord: &CellCoord) -> Option<&Building> {
        if let Some(v) = self.buildings.get(&coord) {
            return Some(v);
        }
        return None;
    }
}
