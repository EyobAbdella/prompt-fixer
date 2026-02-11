use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn test() -> String {
    "Hello From Rust WASM".to_string()
}
