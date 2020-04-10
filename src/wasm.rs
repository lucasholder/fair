use crate::games::*;
use crate::ProvablyFairConfig;
use wasm_bindgen::prelude::*;

use crate::utils;
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
) -> JsValue {
    let config = ProvablyFairConfig::new(client_seed, server_seed, nonce as u64);
    let result = match game {
        "baccarat" => {
            let res = baccarat::simulate(config);
            JsValue::from_serde(&res).unwrap()
        }
        "dice" => {
            let res = dice::simulate(config);
            JsValue::from_serde(&res).unwrap()
        }
        "limbo" => {
            let res = limbo::simulate(config);
            JsValue::from_serde(&res).unwrap()
        }
        "hilo" => {
            let res = hilo::simulate(config);
            JsValue::from_serde(&res).unwrap()
        }
        "blackjack" => {
            let res = blackjack::simulate(config);
            JsValue::from_serde(&res).unwrap()
        }
        "diamond_poker" => {
            let res = diamond_poker::simulate(config);
            JsValue::from_serde(&res).unwrap()
        }
        "roulette" => {
            let res = roulette::simulate(config);
            JsValue::from_serde(&res).unwrap()
        }
        "keno" => {
            let res = keno::simulate(config);
            JsValue::from_serde(&res).unwrap()
        }
        "plinko" => {
            let opts: PlinkoOpts = opts.into_serde().unwrap();
            let rows = opts.rows as u8;
            let risk = plinko::Risk::from_str(&opts.risk);
            let res = plinko::simulate(config, Some(plinko::Opts::new(rows, risk)));
            // format!("Rows: {} Risk: {:?}\n{}", rows, risk, res)
            JsValue::from_serde(&res).unwrap()
        }
        "mines" => {
            let opts: MinesOpts = opts.into_serde().unwrap();
            let mines = opts.mines as u8;
            let res = mines::simulate(config, mines);
            // format!("Rows: {} Risk: {:?}\n{}", rows, risk, res)
            JsValue::from_serde(&res).unwrap()
        }
        "video_poker" => {
            let res = video_poker::simulate(config);
            JsValue::from_serde(&res).unwrap()
        }
        "wheel" => {
            let opts: WheelOpts = opts.into_serde().unwrap();
            let segments = opts.segments as u8;
            let risk = wheel::Risk::from_str(&opts.risk);
            let res = wheel::simulate(config, Some(wheel::Opts::new(segments, risk)));
            JsValue::from_serde(&res).unwrap()
        }
        "slots" => {
            let opts: SlotsOpts = opts.into_serde().unwrap();
            let round = opts.round as usize;
            let res = slots::simulate(config, round);
            JsValue::from_serde(&res).unwrap()
        }
        _ => unimplemented!(),
    };
    result
}

#[wasm_bindgen]
pub fn simulate_multiplayer(game: &str, game_hash: &str, _: JsValue) -> JsValue {
    let game_hash = crash::Hash::from_hex(game_hash);
    let config = crash::Config::for_stake();
    let result = match game {
        "crash" => {
            // crash::verify_hash(config, game_hash)
            let res = crash::simulate(config, game_hash);
            JsValue::from_serde(&res).unwrap()
        }
        _ => unimplemented!(),
    };
    result
}

#[wasm_bindgen]
pub fn verify_game_hash_stake(game_hash: &str) -> bool {
    let config = crash::Config::for_stake();
    return crash::verify_hash(config, crash::Hash::from_hex(game_hash));
}

#[wasm_bindgen]
pub fn hash_server_seed(server_seed: &str) -> String {
    utils::hash_server_seed(server_seed)
}
