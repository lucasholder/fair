//! # provably fair keno
//!

/*
Keno
Traditional Keno games require the selection of 10 possible game events in the form of hits on a
board. To achieve this, we multiply each float by the number of possible unique squares that exist.
Once a hit has been placed, it cannot be chosen again, which changes the pool size of the possible
outcomes. This is done by subtracting the size of possible maximum outcomes by 1 for each iteration
of game event result generated using the corresponding float provided, using the following index:

// Index of 0 to 39 : 1 to 40
const SQUARES = [
  1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
  11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
  21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
  31, 32, 33, 34, 35, 36, 37, 38, 39, 40 ];

const hit = SQUARES[Math.floor(float * 40)];

The fisher-yates shuffle implementation is utilised to prevent duplicate possible hits being generated.
*/

pub use crate::rng::{ProvablyFairConfig, ProvablyFairRNG};
use std::fmt;

#[derive(Debug)]
pub struct SimulationResult {
    pub squares: Vec<u8>,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Squares: {:?}", self.squares)
    }
}

pub fn simulate(config: ProvablyFairConfig) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);

    let mut remaining_squares: Vec<u8> = (1..41).collect();

    let picked_squares: Vec<_> = (0..10)
        .map(|_| {
            let f = rng.next().unwrap();
            let idx = (f * remaining_squares.len() as f64) as usize;
            // println!("idx: {}, remaining: {:?}", idx, remaining_squares);
            let n = *remaining_squares.get(idx).unwrap();
            remaining_squares.remove(idx);
            n
        })
        .collect();

    SimulationResult {
        squares: picked_squares,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate_plinko_test() {
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        assert_eq!(
            simulate(config).squares,
            vec![30, 26, 10, 37, 22, 35, 25, 24, 39, 4]
        );
        let config = ProvablyFairConfig::new("client seed", "server seed", 2);
        assert_eq!(
            simulate(config).squares,
            vec![22, 26, 8, 4, 3, 19, 9, 2, 34, 10]
        );
    }
}
