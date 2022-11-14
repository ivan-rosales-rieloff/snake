use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(a: f64, b:f64) -> f64{
    return a + b;
}


#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
