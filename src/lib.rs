//! # provably fair baccarat
//!
//! Deterministically simulates a game of baccarat. Assumes an inifinite amount of card decks.

/*
use std::env;
use std::error::Error; use std::fs;
*/
use std::process;

mod card;
pub mod games;
mod rng;
mod wasm;

pub use rng::ProvablyFairRNG;

pub fn simulate(
    game: &str,
    client_seed: &str,
    server_seed: &str,
    nonce: u64,
) -> Result<String, String> {
    let result_str = match game {
        "baccarat" => {
            let result = games::baccarat::simulate(client_seed, server_seed, nonce);
            format!("{}", result)
        }
        "dice" => {
            let result = games::dice::simulate(client_seed, server_seed, nonce);
            format!("{}", result)
        }
        "limbo" => {
            let result = games::limbo::simulate(client_seed, server_seed, nonce);
            format!("{}", result)
        }
        "hilo" => {
            let result = games::hilo::simulate(client_seed, server_seed, nonce);
            format!("{}", result)
        }
        "blackjack" => {
            let result = games::blackjack::simulate(client_seed, server_seed, nonce);
            format!("{}", result)
        }
        "diamond_poker" => {
            let result = games::diamond_poker::simulate(client_seed, server_seed, nonce);
            format!("{}", result)
        }
        _ => {
            return Err(format!("{} is not a supported game.", game));
        }
    };
    Ok(result_str)
}
