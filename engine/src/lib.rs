use std::collections::HashMap;

mod log;
mod utils;

mod cell;
mod layout;
mod renderer;
mod world;

use cell::{Hex, Point, Rectangle};
use layout::{HexLayout, HexOrientation};
use renderer::*;
use world::WorldMap;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run(target_id: &str) -> Result<(), JsValue> {
    utils::set_panic_hook();
    log::debug(format!("target_id: {:?}", target_id));

    //alert(format!("target_id: {}", target_id).as_str());

    let mut world = create_hex_world();
    world.generate_hexgon(10);

    world.render(target_id)?;

    Ok(())
}

fn create_hex_world() -> WorldMap<Hex, HexLayout> {
    let hex_layout = HexLayout::new(
        HexOrientation::flat(),
        Rectangle::new(20.0, 20.0),
        Point::origin(),
    );

    let hex_world = WorldMap::new(hex_layout);

    hex_world
}
