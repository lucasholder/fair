//! # provably fair dice game
//!

/*
Our Roulette is derived from the European version of the game where the wheel consists of 37 possible different pockets, ranging from 0 to 36. The game event is calculated by multiplying the float by 37 and then translated into a corresponding pocket using the following index:

// Index of 0 to 36
const POCKETS = [
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
  10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
  20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
  30, 31, 32, 33, 34, 35, 36 ];

// Game event translation
const pocket = POCKETS[Math.floor(float * 37)];
*/

pub use crate::rng::{ProvablyFairConfig, ProvablyFairRNG};
use std::fmt;

#[derive(Debug)]
pub struct SimulationResult {
    pub pocket: u8,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pocket)
    }
}

pub fn simulate(config: ProvablyFairConfig) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);

    let pocket = (rng.next().unwrap() * 37.) as u8;
    SimulationResult { pocket }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate_roulette() {
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        let result = simulate(config);
        assert_eq!(result.pocket, 27);
        let config = ProvablyFairConfig::new("client seed", "server seed", 2);
        let result = simulate(config);
        assert_eq!(result.pocket, 19);
    }
}
