//! # provably fair plinko
//!

/*
For any game of Plinko, the generated outcome is based on the path of the falling ball. The
game event determines the direction of the falling ball for each level in the falling process.
Players can choose between 8 and 16 pins of play, which determines the number of game events
required to generate a complete path from top to bottom. Since there are only two possible
directions (left or right) the translation is done by multiplying each float by 2, which maps
to the following index:

// Index of 0 to 1 : left to right
const DIRECTIONS = [ left, right ];
// Game event translation
const direction = CARDS[Math.floor(float * 2)];
*/

pub use crate::rng::{ProvablyFairConfig, ProvablyFairRNG};
use std::fmt;

#[derive(Debug)]
pub struct SimulationResult {
    pub payout: f64,
    pub index: usize,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Slot: {}\nPayout: {}x", self.index + 1, self.payout)
    }
}

pub enum Risk {
    Low,
    Medium,
    High,
}

pub struct Opts {
    risk: Risk,
    rows: u8,
}

impl Opts {
    pub fn default() -> Opts {
        Self::new(8, Risk::Low)
    }
    pub fn new(rows: u8, risk: Risk) -> Opts {
        assert!(rows >= 8);
        assert!(rows <= 16);
        Opts { risk, rows }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

use Direction::*;

fn get_direction(rng: &mut ProvablyFairRNG<f64>) -> Direction {
    let directions = [Left, Right];
    directions[(rng.next().unwrap() * 2.) as usize]
}

/// Simulates a game of diamond poker.
///
/// # Example
///
/// ```
/// use fair::{games, ProvablyFairConfig};
///
/// let config = ProvablyFairConfig::new("some client seed", "some server seed", 1);
/// let result = games::plinko::simulate(config, None);
/// ```
///
pub fn simulate(config: ProvablyFairConfig, opts: Option<Opts>) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);
    let opts = opts.unwrap_or(Opts::default());

    let total: usize = (3 + opts.rows as usize) * 2 - 1;
    let middle: usize = (total / 2) + 1;

    let mut idx: i32 = middle as i32;
    for _ in 0..opts.rows {
        idx += match get_direction(&mut rng) {
            Left => -1,
            Right => 1,
        }
    }
    idx = idx / 2 - 1;

    let payout = get_payout(opts.rows as usize, opts.risk, idx as usize);

    SimulationResult {
        payout,
        index: idx as usize,
    }
}

// Payout matrixes
static PAYOUT_8: [[f64; 5]; 3] = [
    // low risk
    [5.6, 2.1, 1.1, 1., 0.5],
    // medium risk
    [13., 3., 1.3, 0.7, 1.4],
    // high risk
    [29., 4., 1.5, 0.3, 0.2],
];
static PAYOUT_9: [[f64; 5]; 3] = [
    // low risk
    [5.6, 2., 1.6, 1., 0.7],
    // medium risk
    [18., 4., 1.7, 0.9, 0.5],
    // high risk
    [43., 7., 2., 0.6, 0.2],
];
static PAYOUT_10: [[f64; 6]; 3] = [
    // low risk
    [8.9, 3., 1.4, 1.1, 1., 0.5],
    // medium risk
    [22., 5., 2., 1.4, 0.6, 0.4],
    // high risk
    [76., 10., 3., 0.9, 0.3, 0.2],
];
static PAYOUT_11: [[f64; 6]; 3] = [
    // low risk
    [8.4, 3., 1.9, 1.3, 1., 0.7],
    // medium risk
    [24., 6., 3., 1.8, 0.7, 0.5],
    // high risk
    [120., 14., 5.2, 1.4, 0.4, 0.2],
];
static PAYOUT_12: [[f64; 7]; 3] = [
    // low risk
    [10., 3., 1.6, 1.4, 1.1, 1., 0.5],
    // medium risk
    [33., 11., 4., 2., 1.1, 0.6, 0.3],
    // high risk
    [170., 24., 8.1, 2., 0.7, 0.2, 0.2],
];
static PAYOUT_13: [[f64; 7]; 3] = [
    // low risk
    [8.1, 4., 3., 1.9, 1.2, 0.9, 0.7],
    // medium risk
    [43., 13., 6., 3., 1.3, 0.7, 0.4],
    // high risk
    [260., 37., 11., 4., 1., 0.2, 0.2],
];
static PAYOUT_14: [[f64; 8]; 3] = [
    // low risk
    [7.1, 4., 1.9, 1.4, 1.3, 1.1, 1., 0.5],
    // medium risk
    [58., 15., 7., 4., 1.9, 1., 0.5, 0.2],
    // high risk
    [420., 56., 18., 5., 1.9, 0.3, 0.2, 0.2],
];
static PAYOUT_15: [[f64; 8]; 3] = [
    // low risk
    [15., 8., 3., 2., 1.5, 1.1, 1., 0.7],
    // medium risk
    [88., 18., 11., 5., 3., 1.3, 0.5, 0.3],
    // high risk
    [620., 83., 27., 8., 3., 0.5, 0.2, 0.2],
];
static PAYOUT_16: [[f64; 9]; 3] = [
    // low risk
    [16., 9., 2., 1.4, 1.4, 1.2, 1.1, 1., 0.5],
    // medium risk
    [110., 41., 10., 5., 3., 1.5, 1., 0.5, 0.3],
    // high risk
    [1000., 130., 26., 9., 4., 2., 0.2, 0.2, 0.2],
];

fn get_payout(rows: usize, risk: Risk, slot_index: usize) -> f64 {
    let risk_idx = match risk {
        Risk::Low => 0,
        Risk::Medium => 1,
        Risk::High => 2,
    };
    let payout_row = match rows {
        8 => &PAYOUT_8[risk_idx][..],
        9 => &PAYOUT_9[risk_idx][..],
        10 => &PAYOUT_10[risk_idx][..],
        11 => &PAYOUT_11[risk_idx][..],
        12 => &PAYOUT_12[risk_idx][..],
        13 => &PAYOUT_13[risk_idx][..],
        14 => &PAYOUT_14[risk_idx][..],
        15 => &PAYOUT_15[risk_idx][..],
        16 => &PAYOUT_16[risk_idx][..],
        _ => panic!("rows ({}) must be between 8 and 16 inclusive", rows),
    };

    let len = payout_row.len();
    let last_idx = len - 1;
    // Passed the last payout in the array, we go from right to left because the payouts are
    // symmetric.
    let index = if slot_index > last_idx {
        // The middle payout for 8, 10, 12, 14, 16 rows is not repeated
        // For example for 8 rows, payout row is (1 is not repeated):
        // 5.6, 2.1, 1.1, 1, 0.5, 1, 1.1, 2.1, 5.6
        // Whereas for 9 rows, payout row is (0.7 is repeated):
        // 5.6, 2, 1.6, 1, 0.7, 0.7, 1, 1.6, 2, 5.6
        let repeat = if rows % 2 == 0 { 0 } else { 1 };

        last_idx - (slot_index - last_idx) + repeat
    } else {
        slot_index
    };
    payout_row[index]
}

fn fac(n: u32) -> u32 {
    let mut i = n;
    let mut res = 1;
    while i > 0 {
        res = res * i;
        i -= 1;
    }
    res
}

use num_integer;

fn slot_probability(rows: usize, slot_index: usize) -> f64 {
    // https://en.wikipedia.org/wiki/Bean_machine#Distribution_of_the_beads
    let p: f64 = 0.5;

    let n = rows as f64;
    let k = slot_index;
    let binom = num_integer::binomial(8, k) as f64;
    let k = k as f64;

    let prob = binom as f64 * p.powf(k) * (1. - p).powf(n - k);
    prob
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate_plinko_test() {
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        assert_eq!(simulate(config, None).index, 7);
        let config = ProvablyFairConfig::new("client seed", "server seed", 2);
        assert_eq!(simulate(config, None).index, 2);
        let config = ProvablyFairConfig::new("client seed", "server seed", 3);
        assert_eq!(simulate(config, None).index, 5);
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        assert_eq!(simulate(config, Some(Opts::new(9, Risk::Low))).index, 8);
        let config = ProvablyFairConfig::new("client seed", "server seed", 2);
        assert_eq!(simulate(config, Some(Opts::new(9, Risk::Low))).index, 3);
        let config = ProvablyFairConfig::new("client seed", "server seed", 3);
        assert_eq!(simulate(config, Some(Opts::new(9, Risk::Low))).index, 6);
    }

    #[test]
    fn simulate_plinko_test_payout() {
        assert_eq!(
            simulate(
                ProvablyFairConfig::new("client seed", "server seed", 1),
                Some(Opts::new(16, Risk::Low))
            )
            .payout,
            1.4
        );
        assert_eq!(
            simulate(
                ProvablyFairConfig::new("client seed", "server seed", 1),
                Some(Opts::new(8, Risk::Low))
            )
            .payout,
            2.1
        );
        assert_eq!(
            simulate(
                ProvablyFairConfig::new("client seed", "server seed", 1),
                Some(Opts::new(8, Risk::Medium))
            )
            .payout,
            3.
        );
        assert_eq!(
            simulate(
                ProvablyFairConfig::new("client seed", "server seed", 1),
                Some(Opts::new(8, Risk::High))
            )
            .payout,
            4.
        );
        assert_eq!(
            simulate(
                ProvablyFairConfig::new("client seed", "server seed", 1),
                Some(Opts::new(9, Risk::Low))
            )
            .payout,
            2.
        );
        assert_eq!(
            simulate(
                ProvablyFairConfig::new("client seed", "server seed", 1),
                Some(Opts::new(9, Risk::Medium))
            )
            .payout,
            4.
        );
        assert_eq!(
            simulate(
                ProvablyFairConfig::new("client seed", "server seed", 1),
                Some(Opts::new(10, Risk::Low))
            )
            .payout,
            1.4
        );
    }
    #[test]
    fn test_fac() {
        assert_eq!(fac(3), 6);
        assert_eq!(fac(4), 24);
        assert_eq!(fac(5), 120);
    }
    #[test]
    fn test_binomial_coefficient() {
        /*
        for i in 1..8 {
            println!("{}", num_integer::binomial(8, i));
        }
        */
        assert_eq!(num_integer::binomial(52, 5), 2_598_960);
    }
    #[test]
    fn test_slot_probabily() {
        assert_eq!(slot_probability(8, 0), 0.00390625);
        assert_eq!(slot_probability(8, 1), 0.03125);
        assert_eq!(slot_probability(8, 2), 0.109375);
        assert_eq!(slot_probability(8, 3), 0.218750);
        assert_eq!(slot_probability(8, 4), 0.2734375);
        assert_eq!(slot_probability(8, 5), 0.218750);
        assert_eq!(slot_probability(8, 6), 0.109375);
        assert_eq!(slot_probability(8, 7), 0.03125);
        assert_eq!(slot_probability(8, 8), 0.00390625);
    }
}
