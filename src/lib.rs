use wasm_bindgen::prelude::*;

// Calling external functions in JavaScript from Rust
#[wasm_bindgen]
extern {
    // pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Producing Rust functions that JavaScript can call
#[wasm_bindgen]
pub fn greet(name: &str) {
    // alert(&format!("Hello, {}!", name));
    log(&format!("Hello, {}!", name));
}
