//! # provably fair hilo
//!
//! Deterministically simulates a game of hilo. Returns shuffled deck.

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
    deck: Deck,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deck)
    }
}

pub fn simulate(config: ProvablyFairConfig) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);
    let deck = Deck::from_rng(&mut rng, 52);

    SimulationResult { deck }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate_hilo_1() {
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        let result = simulate(config);
        // println!("{:?}", result);

        assert_eq!(format!("{}", result), "♠J - ♥10 - ♥5 - ♣K - ♥9 - ♥K - ♠10 - ♥10 - ♦A - ♠3 - ♠2 - ♣J - ♠A - ♥A - ♣5 - ♦A - ♥A - ♥J - ♦2 - ♣4 - ♦Q - ♠4 - ♣6 - ♣J - ♣2 - ♦7 - ♣9 - ♦6 - ♥2 - ♥8 - ♦Q - ♥8 - ♥10 - ♠10 - ♦Q - ♣7 - ♥8 - ♦2 - ♣9 - ♥4 - ♦10 - ♥2 - ♣7 - ♥10 - ♣Q - ♠Q - ♠9 - ♣A - ♥J - ♣6 - ♣8 - ♦J");
    }

    #[test]
    fn simulate_hilo_2() {
        let config = ProvablyFairConfig::new("other client seed", "server seed", 1);
        let result = simulate(config);
        // println!("{:?}", result);

        assert_eq!(format!("{}", result), "♦9 - ♠9 - ♦A - ♠A - ♦J - ♠K - ♦Q - ♣A - ♦3 - ♥10 - ♥10 - ♥5 - ♦J - ♦7 - ♦K - ♣6 - ♠Q - ♥7 - ♦4 - ♠3 - ♣3 - ♠Q - ♠5 - ♠8 - ♦10 - ♠3 - ♥Q - ♣8 - ♣10 - ♠9 - ♥7 - ♣J - ♥5 - ♠K - ♣2 - ♦3 - ♦A - ♣J - ♣3 - ♥A - ♦10 - ♠5 - ♣K - ♥K - ♣4 - ♦8 - ♦10 - ♠9 - ♣K - ♠9 - ♣3 - ♦5");
    }
}
