//! # provably fair dice game
//!

pub use crate::rng::ProvablyFairRNG;
use std::fmt;

#[derive(Debug)]
pub struct SimulationResult {
    pub outcome: f64,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.outcome)
    }
}
/// Simulates a game of dice.
///
/// # Example
///
/// ```
///
/// let client_seed = "some client seed";
/// let server_seed = "some server seed";
/// let nonce = 1;
/// let result = fair::games::dice::simulate(
///   client_seed,
///   server_seed,
///   nonce,
/// );
/// ```
///
pub fn simulate(client_seed: &str, server_seed: &str, nonce: u64) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::new(client_seed, server_seed, nonce);

    let float = rng.next().unwrap();
    // Game event translation with houseEdge of 0.99 (1%)
    let house_edge = 0.99;
    let max_multiplier = 10_000_000_f64;
    let float_point = float * max_multiplier;
    let float_point = max_multiplier / float_point * house_edge;

    // Crash point rounded down to required denominator
    let crash_point = ((float_point * 100_f64) as u32) as f64 / 100_f64;

    let outcome = crash_point;
    SimulationResult { outcome }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate_dice_roll() {
        let client_seed = "client seed";
        let server_seed = "server seed";
        let nonce = 1;
        let result = simulate(client_seed, server_seed, nonce);
        // println!("{:?}", result);
        assert_eq!(result.outcome, 1.32);
        let nonce = 2;
        let result = simulate(client_seed, server_seed, nonce);
        // println!("{:?}", result);
        assert_eq!(result.outcome, 1.83);
        let nonce = 3;
        let result = simulate(client_seed, server_seed, nonce);
        // println!("{:?}", result);
        assert_eq!(result.outcome, 4.28);
    }
}
