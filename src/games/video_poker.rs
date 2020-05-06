//! # provably fair video poker
//!

/*
use std::env;
use std::error::Error; use std::fs;
*/

use crate::card::Deck;
pub use crate::rng::{ProvablyFairConfig, ProvablyFairRNG};

use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct SimulationResult {
    initial_hand: Deck,
    coming_cards: Deck,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Initial Hand: {}\nComing Cards: {}",
            self.initial_hand, self.coming_cards
        )
    }
}

/// Simulates a game of video poker.
pub fn simulate(config: ProvablyFairConfig) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);
    let initial_hand = Deck::from_rng(&mut rng, 5);
    let coming_cards = Deck::from_rng(&mut rng, 5);

    // keep track of drawn cards
    SimulationResult {
        initial_hand,
        coming_cards,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate_video_poker() {
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        let res = simulate(config);
        assert_eq!(res.initial_hand.to_string(), "♠J - ♥10 - ♥5 - ♣K - ♥9");
        assert_eq!(res.coming_cards.to_string(), "♥K - ♠10 - ♥10 - ♦A - ♠3");
    }
}
