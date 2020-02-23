use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub extern "C" fn add(x: u32, y: u32) -> u32 {
    x + y
}
