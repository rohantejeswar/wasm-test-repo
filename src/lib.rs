mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: Option<String>) {
    let mut greeting_string = "".to_string();

    if !name.is_none() {
        greeting_string = format!("Hello, {param_name}! Welcome to the wasm-game-of-life!", param_name=name.unwrap()).to_owned()
    }  else {
        greeting_string = "Hello there! Welcome to the wasm-game-of-life!".to_string()
    };

    let greeting_str: &str = &greeting_string[..];

    alert(greeting_str);
}

// #[wasm_bindgen]
// pub fn greet(name: Option<&str>) {
//     let greeting = if !name.is_none() { format!("Hello, {param_name}! Welcome to the wasm-game-of-life!", param_name=name) } else { format!("Hello there! Welcome to the wasm-game-of-life!") };

//     alert(greeting);
// }
