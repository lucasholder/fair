//! # provably fair baccarat
//!
//! Deterministically simulates a game of baccarat. Assumes an inifinite amount of card decks.

/*
use std::env;
use std::error::Error; use std::fs;
*/
// use std::process;

mod card;
pub mod games;
mod rng;
mod wasm;

pub use rng::{ProvablyFairConfig, ProvablyFairRNG};

pub fn simulate(
    game: &str,
    client_seed: &str,
    server_seed: &str,
    nonce: u64,
) -> Result<String, String> {
    let config = ProvablyFairConfig::new(client_seed, server_seed, nonce);
    let result_str = match game {
        "baccarat" => {
            let result = games::baccarat::simulate(config);
            format!("{}", result)
        }
        "dice" => {
            let result = games::dice::simulate(config);
            format!("{}", result)
        }
        "limbo" => {
            let result = games::limbo::simulate(config);
            format!("{}", result)
        }
        "hilo" => {
            let result = games::hilo::simulate(config);
            format!("{}", result)
        }
        "blackjack" => {
            let result = games::blackjack::simulate(config);
            format!("{}", result)
        }
        "diamond_poker" => {
            let result = games::diamond_poker::simulate(config);
            format!("{}", result)
        }
        "plinko" => {
            let result = games::plinko::simulate(config);
            format!("{}", result)
        }
        _ => {
            return Err(format!("{} is not a supported game.", game));
        }
    };
    Ok(result_str)
}
