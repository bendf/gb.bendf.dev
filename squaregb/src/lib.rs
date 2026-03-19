use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("No global 'window' object");
    let document = window.document().expect("No 'document' object on window");
    let body = document.body().expect("No 'body' object on document");

    // let root = document.get_element_by_id("squaregb-root");
    // let canvas = document.create_element("canvas");

    Ok(())
}

#[wasm_bindgen]
pub fn say_hello() -> String {
    return String::from("Hello, World!");
}
