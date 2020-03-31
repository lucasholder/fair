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

fn default_wheel_segments() -> i32 {
    10
}

fn default_wheel_risk() -> String {
    "low".to_string()
}

#[derive(Deserialize)]
struct PlinkoOpts {
    #[serde(default = "default_plinko_rows")]
    rows: i32,
    #[serde(default = "default_plinko_risk")]
    risk: String,
}

#[derive(Deserialize)]
struct WheelOpts {
    #[serde(default = "default_wheel_segments")]
    segments: i32,
    #[serde(default = "default_wheel_risk")]
    risk: String,
}

fn default_mines_mines() -> i32 {
    3
}

#[derive(Deserialize)]
struct MinesOpts {
    #[serde(default = "default_mines_mines")]
    mines: i32,
}

fn default_slots_round() -> i32 {
    0
}

#[derive(Deserialize)]
struct SlotsOpts {
    #[serde(default = "default_slots_round")]
    round: i32,
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
            let opts: MinesOpts = opts.into_serde().unwrap();
            let mines = opts.mines as u8;
            let res = mines::simulate(config, mines).to_string();
            // format!("Rows: {} Risk: {:?}\n{}", rows, risk, res)
            res
        }
        "video_poker" => video_poker::simulate(config).to_string(),
        "wheel" => {
            let opts: WheelOpts = opts.into_serde().unwrap();
            let segments = opts.segments as u8;
            let risk = wheel::Risk::from_str(&opts.risk);
            let res = wheel::simulate(config, Some(wheel::Opts::new(segments, risk))).to_string();
            res
        }
        "slots" => {
            let opts: SlotsOpts = opts.into_serde().unwrap();
            let round = opts.round as usize;
            let res = slots::simulate(config, round).to_string();
            res
        }
        _ => panic!("This branch should never execute. Unimplemented game?"),
    };
    result
}
