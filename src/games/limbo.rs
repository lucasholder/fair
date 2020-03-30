//! # provably fair limbo game
//!

pub use crate::rng::{ProvablyFairConfig, ProvablyFairRNG};
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
/// use fair::{games, ProvablyFairConfig};
///
/// let config = ProvablyFairConfig::new("some client seed", "some server seed", 1);
/// let result = games::limbo::simulate(config);
/// ```
///
pub fn simulate(config: ProvablyFairConfig) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);

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
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        let result = simulate(config);
        // println!("{:?}", result);
        assert_eq!(result.outcome, 1.32);
        let config = ProvablyFairConfig::new("client seed", "server seed", 2);
        let result = simulate(config);
        // println!("{:?}", result);
        assert_eq!(result.outcome, 1.83);
        let config = ProvablyFairConfig::new("client seed", "server seed", 3);
        let result = simulate(config);
        // println!("{:?}", result);
        assert_eq!(result.outcome, 4.28);
    }
}
