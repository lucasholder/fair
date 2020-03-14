//! # provably fair hilo
//!
//! Deterministically simulates a game of hilo. Returns shuffled deck.

/*
use std::env;
use std::error::Error; use std::fs;
*/

use crate::card::Deck;
pub use crate::rng::ProvablyFairRNG;

use std::fmt;

#[derive(Debug)]
pub struct SimulationResult {
    dealer: Deck,
    player: Deck,
    deck: Deck,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Dealer: {}\nPlayer: {}\nDeck: {}",
            self.dealer, self.player, self.deck
        )
    }
}

pub fn simulate(client_seed: &str, server_seed: &str, nonce: u64) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::new(client_seed, server_seed, nonce);
    let player = Deck::from_rng(&mut rng, 2);
    let dealer = Deck::from_rng(&mut rng, 2);
    let deck = Deck::from_rng(&mut rng, 52 - 4);
    SimulationResult {
        dealer,
        player,
        deck,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate_hilo_1() {
        let client_seed = "client seed";
        let server_seed = "server seed";
        let nonce = 1;
        let result = simulate(client_seed, server_seed, nonce);
        // println!("{:?}", result);

        assert_eq!(format!("{}", result), "");
    }
}
