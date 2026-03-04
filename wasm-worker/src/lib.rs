use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(s: &str) -> String {
    return format!("Hello {}!", s);
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn fibonacci(num: i32) -> i32 {
    return match num {
        0 => 0,
        1 => 1,
        _ => fibonacci(num - 1) + fibonacci(num - 2),
    };
}
