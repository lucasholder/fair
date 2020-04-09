//! # provably fair dice game
//!

pub use crate::rng::{ProvablyFairConfig, ProvablyFairRNG};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
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
/// use fair::{games, ProvablyFairConfig};
///
/// let config = ProvablyFairConfig::new("some client seed", "some server seed", 1);
/// let result = games::dice::simulate(config);
/// ```
///
pub fn simulate(config: ProvablyFairConfig) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);

    let outcome = (rng.next().unwrap() * 10001.) as u32;
    let outcome = outcome as f64 / 100.;
    SimulationResult { outcome }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate_dice_roll() {
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        let result = simulate(config);
        // println!("{:?}", result);
        assert_eq!(result.outcome, 74.67);
        let config = ProvablyFairConfig::new("client seed", "server seed", 2);
        let result = simulate(config);
        // println!("{:?}", result);
        assert_eq!(result.outcome, 53.86);
    }
}
