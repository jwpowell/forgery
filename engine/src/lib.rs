use std::cell::{Ref, RefCell};
use std::collections::HashMap;

mod cell;
mod game_view;
mod layout;
mod log;
mod renderer;
mod utils;
mod world;

use cell::{Cell, CellCoord, Hex, Point, Rectangle};
use game_view::{BeltView, BuildingState, BuildingView, GameStateView};
use layout::{HexLayout, HexOrientation, Layout};
use renderer::*;
use world::WorldMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;

use web_sys::{Document, Element, Event, HtmlElement, MouseEvent};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let target_id = "workspace";

    log::debug(format!("target_id: {:?}", target_id));

    //alert(format!("target_id: {}", target_id).as_str());
    /*
        let mut world = create_hex_world();

        WORLD.with(|w| {
            *w = RefCell::new(world);
        });

        world.generate_hexgon(10);

        world.render(target_id)?;
    */

    renderer::WORLD.with(|w| -> Result<(), JsValue> {
        log::debug(format!("generating world"));

        w.borrow_mut().generate_hexgon(10);
        w.borrow().render(target_id)?;
        Ok(())
    })?;

    let document = renderer::get_document().unwrap();

    let window = web_sys::window().expect("no global `window` exists");
    let body = document.body().expect("document should have a body");
    let target = renderer::get_target(&document, target_id)?;

    Ok(())
}
