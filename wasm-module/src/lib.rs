use wasm_bindgen::prelude::*;

mod scripting;

#[wasm_bindgen]
pub extern "C" fn run_script(
    script: String,
    print_callback: js_sys::Function,
    debug_callback: js_sys::Function,
) -> Result<String, JsValue> {
    console_error_panic_hook::set_once();

    Ok(scripting::run_script(
        &script,
        move |s| {
            let _ = print_callback.call1(&JsValue::null(), &JsValue::from_str(s));
        },
        move |s| {
            let _ = debug_callback.call1(&JsValue::null(), &JsValue::from_str(s));
        },
    )?)
}
