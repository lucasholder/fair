//! # provably fair dice game
//!

/*
The game event number is calculated by multiplying the float by the possible outcomes in the reel.
The first 4 reels have a length of 30 possible outcomes, whilst the last reel has 41. The game
event determines the central stop position for each reel. This game consists of 5 game event
numbers, until the case of a bonus round, where more are generated.
*/

pub use crate::rng::{ProvablyFairConfig, ProvablyFairRNG};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
// There are 5 reels
// One outcome per reel (an index into the reel's outcomesl)
// First 4 reels have 30 possible outcomes
// 5th reel has 41 possible outcomes
pub struct SimulationResult {
    pub outcomes: [usize; 5],
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.outcomes)
    }
}

pub fn simulate(config: ProvablyFairConfig, round: usize) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);
    // skip previous rounds...
    for _ in 0..round {
        for _ in 0..5 {
            rng.next();
        }
    }
    let reel1 = rng.range(0, 30);
    let reel2 = rng.range(0, 30);
    let reel3 = rng.range(0, 30);
    let reel4 = rng.range(0, 30);
    let reel5 = rng.range(0, 41);
    SimulationResult {
        outcomes: [reel1, reel2, reel3, reel4, reel5],
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate_slots() {
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        let result = simulate(config, 0);
        assert_eq!(result.outcomes, [22, 19, 7, 27, 23]);
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        let result = simulate(config, 1);
        assert_eq!(result.outcomes, [26, 20, 19, 28, 5]);
    }
}
