mod log;
mod utils;

mod geometry;
mod world;

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
pub fn greet() {
    use utils;

    utils::set_panic_hook();

    log::debug("test log".into());

    alert("Hello, engine!");
}

#[wasm_bindgen]
pub fn world() {}
