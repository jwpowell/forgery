//use log;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub fn alert_js(message: String) {
    alert(message.as_str());
}

pub fn debug(message: String) {
    use web_sys::console;

    //log::debug!("{:?}", message);
    console::debug_1(&message.into());
}

pub fn info(message: String) {
    use web_sys::console;

    //log::info!("{:?}", message);
    console::info_1(&message.into());
}

pub fn warn(message: String) {
    use web_sys::console;

    //log::warn!("{:?}", message);
    console::warn_1(&message.into());
}

pub fn error(message: String) {
    use web_sys::console;

    //log::error!("{:?}", message);
    console::error_1(&message.into());
}
