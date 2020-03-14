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

    let outcome = (rng.next().unwrap() * 10001.) as u32;
    let outcome = outcome as f64 / 100.;
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
        assert_eq!(result.outcome, 74.67);
        let nonce = 2;
        let result = simulate(client_seed, server_seed, nonce);
        // println!("{:?}", result);
        assert_eq!(result.outcome, 53.86);
    }
}
