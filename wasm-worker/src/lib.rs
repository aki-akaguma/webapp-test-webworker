use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(s: &str) -> String {
    format!("Hello {}!", s)
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn fibonacci(num: i32) -> i32 {
    match num {
        0 => 0,
        1 => 1,
        _ => fibonacci(num - 1) + fibonacci(num - 2),
    }
}
