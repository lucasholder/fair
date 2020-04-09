//! # provably fair limbo game
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

pub fn simulate(config: ProvablyFairConfig) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);

    let m = 1e8;
    let house_edge = 0.99;

    // Documentation on Stake is wrong! It says:
    //
    // const floatPoint = 1e8 / (float * 1e8) * houseEdge;
    //
    // but the correct JS formula actually is:
    //
    // const floatPoint = 1e8 / (Math.floor(float * 1e8) + 1) * houseEdge;
    //
    // Found by reverse engineering the client side JavaScript verification source code.

    // let float = rng.next().unwrap();
    // let crash_point = m / ((float * m).floor() + 1.) * house_edge;

    let n = rng.range(1, m as usize + 1) as f64;
    let crash_point = m / n * house_edge;

    // Round to 2 decimals
    let crash_point = (crash_point * 100.).floor() / 100.;

    let outcome = crash_point;
    SimulationResult { outcome }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_limbo() {
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

    #[test]
    fn test_limbo_bug_when_tiny_float() {
        // https://stake.com/provably-fair/calculation?clientSeed=83e27f682128eb1852b048203dfd6931&game=limbo&nonce=1942124&serverSeed=e8df2cc3b9ccb583ce5ea92336842387
        let config = ProvablyFairConfig::new(
            "83e27f682128eb1852b048203dfd6931",
            "e8df2cc3b9ccb583ce5ea92336842387",
            1942124,
        );
        let result = simulate(config);
        assert_eq!(result.outcome, 3807692.3);
    }
}
