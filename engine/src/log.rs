
pub fn debug(message: String) {
    use web_sys::console;

    console::debug_1(&message.into());
}

pub fn info(message: String) {
    use web_sys::console;

    console::info_1(&message.into());
}

pub fn warn(message: String) {
    use web_sys::console;

    console::warn_1(&message.into());
}

pub fn error(message: String) {
    use web_sys::console;

    console::error_1(&message.into());
}

