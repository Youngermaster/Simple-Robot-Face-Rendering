// Suppress dead-code warnings from the lib crate view — all these are used
// either by main.rs (native) or by wasm_main below (WASM).
#![allow(dead_code)]

mod app;
mod robot_face;
mod state;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    // Redirect panics to browser console for easier debugging
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("logger init failed");

    app::run();
}
