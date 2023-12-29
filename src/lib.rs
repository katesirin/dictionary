mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, word-wasm!");
}

#[wasm_bindgen]
pub fn helloworld() -> String {
    String::from("A, aback, abacus, abalone")
}
