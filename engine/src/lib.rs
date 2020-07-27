use std::collections::HashMap;

mod cell;
mod game_view;
mod layout;
mod log;
mod renderer;
mod utils;
mod world;

use cell::{CellCoord, Hex, Point, Rectangle};
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

    let document = renderer::get_document().unwrap();

    let window = web_sys::window().expect("no global `window` exists");
    let body = document.body().expect("document should have a body");
    /*
        // TODO: cannot borrow world because the closure can outlive it. world needs to be a &'static so JS callback closures and operate on it.
        add_event(&body, "mousedown", |e: Event| {
            let btn = e.clone().dyn_into::<MouseEvent>().unwrap().button();
            if btn == 0 {
                alert(format!("left click").as_str());
            } else if btn == 2 {
                alert(format!("right click").as_str());
            }

            let event = e.clone().dyn_into::<MouseEvent>().unwrap();

            alert(format!("mousedown: {:?},{:?}", event.screen_x(), event.screen_y()).as_str());

            let point = Point::new(event.screen_x() as f32, event.screen_y() as f32);
            world.layout.pixel_to_cell(&point);
        });
    */
    Ok(())
}

pub fn add_event<H>(el: &HtmlElement, event_type: &str, event_listener: H)
where
    H: 'static + FnMut(Event),
{
    let cl = Closure::wrap(Box::new(event_listener) as Box<dyn FnMut(_)>);
    el.add_event_listener_with_callback("click", cl.as_ref().unchecked_ref())
        .unwrap();
    cl.forget();
}

fn create_hex_world() -> WorldMap<Hex, HexLayout> {
    let hex_layout = HexLayout::new(
        HexOrientation::flat(),
        Rectangle::new(20.0, 20.0),
        Point::origin(),
    );

    let mut factory_1_nodes: HashMap<CellCoord, bool> = HashMap::new();
    factory_1_nodes.insert(CellCoord::new(1, 1, 1), false);

    let factory_1 = BuildingView::new(
        CellCoord::new(-4, -2, 6),
        factory_1_nodes,
        BuildingState::Working,
    );

    let mut factory_2_nodes: HashMap<CellCoord, bool> = HashMap::new();
    factory_2_nodes.insert(CellCoord::new(0, 1, 1), false);

    let factory_2 = BuildingView::new(
        CellCoord::new(-1, -4, 5),
        factory_2_nodes,
        BuildingState::Working,
    );

    let game_state_view = GameStateView::new(&[factory_1, factory_2], &[]);

    let hex_world = WorldMap::new(hex_layout, game_state_view);

    hex_world
}
