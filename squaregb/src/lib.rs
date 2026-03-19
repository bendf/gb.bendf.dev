use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("No global 'window' object");
    let document = window.document().expect("No 'document' object on window");
    let _body = document.body().expect("No 'body' object on document");

    let _root = document
        .get_element_by_id("squaregb-root")
        .expect("No 'squaregb-root' element");

    Ok(())
}

#[wasm_bindgen]
pub fn say_hello() -> String {
    return String::from("Hello, World!");
}

pub fn example() {}
