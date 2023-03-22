use wasm_bindgen::prelude::*;

// Calling external functions in JavaScript from Rust
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// Producing Rust functions that JavaScript can call
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
