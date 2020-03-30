//! # provably fair hilo
//!
//! Deterministically simulates a game of hilo. Returns shuffled deck.

/*
use std::env;
use std::error::Error; use std::fs;
*/

use crate::card::Deck;
pub use crate::rng::{ProvablyFairConfig, ProvablyFairRNG};

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

pub fn simulate(config: ProvablyFairConfig) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);
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
    fn simulate_blackjack() {
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        let result = simulate(config);
        // println!("{:?}", result);

        assert_eq!(format!("{}", result), "Dealer: ♥5 - ♣K\nPlayer: ♠J - ♥10\nDeck: ♥9 - ♥K - ♠10 - ♥10 - ♦A - ♠3 - ♠2 - ♣J - ♠A - ♥A - ♣5 - ♦A - ♥A - ♥J - ♦2 - ♣4 - ♦Q - ♠4 - ♣6 - ♣J - ♣2 - ♦7 - ♣9 - ♦6 - ♥2 - ♥8 - ♦Q - ♥8 - ♥10 - ♠10 - ♦Q - ♣7 - ♥8 - ♦2 - ♣9 - ♥4 - ♦10 - ♥2 - ♣7 - ♥10 - ♣Q - ♠Q - ♠9 - ♣A - ♥J - ♣6 - ♣8 - ♦J");
    }
}
