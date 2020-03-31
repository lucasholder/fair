//! # provably wheel game
//!

mod payouts;

use payouts::*;

pub use crate::rng::{ProvablyFairConfig, ProvablyFairRNG};
use std::fmt;

#[derive(Debug)]
pub struct SimulationResult {
    pub payout: f64,
    pub index: usize,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Segment: {} Payout: {}", self.index, self.payout)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Risk {
    Low,
    Medium,
    High,
}
impl Risk {
    pub fn from_str(s: &str) -> Risk {
        match &s.to_lowercase()[..] {
            "low" => Risk::Low,
            "medium" => Risk::Medium,
            "high" => Risk::High,
            _ => panic!("invalid risk string {}", s),
        }
    }
}

pub struct Opts {
    risk: Risk,
    segments: u8,
}

pub fn validate_segments(segments: u8) -> bool {
    match segments {
        10 | 20 | 30 | 40 | 50 => true,
        _ => false,
    }
}

impl Opts {
    pub fn default() -> Opts {
        Self::new(10, Risk::Low)
    }
    pub fn new(segments: u8, risk: Risk) -> Opts {
        assert!(validate_segments(segments));
        Opts { risk, segments }
    }
}

fn get_payout_slice(segments: u8, risk: Risk) -> &'static [f64] {
    let risk_idx = match risk {
        Risk::Low => 0,
        Risk::Medium => 1,
        Risk::High => 2,
    };
    match segments {
        10 => &PAYOUT_10[risk_idx][..],
        20 => &PAYOUT_20[risk_idx][..],
        30 => &PAYOUT_30[risk_idx][..],
        40 => &PAYOUT_40[risk_idx][..],
        50 => &PAYOUT_50[risk_idx][..],
        _ => panic!("segments ({}) must be one of 10,20,30,40,50", segments),
    }
}

pub fn simulate(config: ProvablyFairConfig, opts: Option<Opts>) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);
    let opts = opts.unwrap_or(Opts::default());

    let payouts = get_payout_slice(opts.segments, opts.risk);

    let idx = (rng.next().unwrap() * payouts.len() as f64) as usize;
    let payout = payouts[idx];

    SimulationResult {
        payout,
        index: idx as usize,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate_wheel_test() {
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        let SimulationResult { payout, index } = simulate(config, Some(Opts::new(10, Risk::Low)));
        assert_eq!(index, 7);
        assert_eq!(payout, 1.2);

        let config = ProvablyFairConfig::new("client seed", "server seed", 2);
        let SimulationResult { payout, index } = simulate(config, Some(Opts::new(10, Risk::Low)));
        assert_eq!(index, 5);
        assert_eq!(payout, 1.2);

        let config = ProvablyFairConfig::new("client seed", "server seed", 2);
        let SimulationResult { payout, index } = simulate(config, Some(Opts::new(20, Risk::Low)));
        assert_eq!(index, 10);
        assert_eq!(payout, 1.5);

        let config = ProvablyFairConfig::new("client seed", "server seed", 2);
        let SimulationResult { payout, index } =
            simulate(config, Some(Opts::new(40, Risk::Medium)));
        assert_eq!(index, 21);
        assert_eq!(payout, 0.);
    }
}
