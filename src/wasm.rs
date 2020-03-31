use crate::games::*;
use crate::ProvablyFairConfig;
use wasm_bindgen::prelude::*;

use serde::Deserialize;

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

fn default_plinko_risk() -> String {
    "low".to_string()
}

fn default_plinko_rows() -> i32 {
    8
}

#[derive(Deserialize)]
struct PlinkoOpts {
    #[serde(default = "default_plinko_rows")]
    rows: i32,
    #[serde(default = "default_plinko_risk")]
    risk: String,
}

fn default_keno_mines() -> i32 {
    3
}

#[derive(Deserialize)]
struct KenoOpts {
    #[serde(default = "default_keno_mines")]
    mines: i32,
}

#[wasm_bindgen]
pub fn simulate(
    game: &str,
    client_seed: &str,
    server_seed: &str,
    nonce: u32,
    opts: &JsValue,
) -> String {
    let config = ProvablyFairConfig::new(client_seed, server_seed, nonce as u64);
    let result = match game {
        "baccarat" => baccarat::simulate(config).to_string(),
        "dice" => dice::simulate(config).to_string(),
        "limbo" => limbo::simulate(config).to_string(),
        "hilo" => hilo::simulate(config).to_string(),
        "blackjack" => blackjack::simulate(config).to_string(),
        "diamond_poker" => diamond_poker::simulate(config).to_string(),
        "roulette" => roulette::simulate(config).to_string(),
        "keno" => keno::simulate(config).to_string(),
        "plinko" => {
            let opts: PlinkoOpts = opts.into_serde().unwrap();
            let rows = opts.rows as u8;
            let risk = plinko::Risk::from_str(&opts.risk);
            let res = plinko::simulate(config, Some(plinko::Opts::new(rows, risk))).to_string();
            // format!("Rows: {} Risk: {:?}\n{}", rows, risk, res)
            res
        }
        "mines" => {
            let opts: KenoOpts = opts.into_serde().unwrap();
            let mines = opts.mines as u8;
            let res = mines::simulate(config, mines).to_string();
            // format!("Rows: {} Risk: {:?}\n{}", rows, risk, res)
            res
        }
        "video_poker" => video_poker::simulate(config).to_string(),
        _ => panic!("This branch should never execute. Unimplemented game?"),
    };
    result
}
