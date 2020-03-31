use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
}
*/

/*
#[wasm_bindgen]
pub fn simulate(game: &str, client_seed: &str, server_seed: &str, nonce: u64) -> String {
    crate::simulate(game, client_seed, server_seed, nonce, None as Option<()>).unwrap_or_default()
}
*/
