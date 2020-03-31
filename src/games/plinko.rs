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
const direction = CARDS[Math.floor(float * 2)]; */

pub use crate::rng::{ProvablyFairConfig, ProvablyFairRNG};
use std::cmp;
use std::collections::HashMap;
use std::fmt;

static GEM_ORDER: [Gem; 7] = [Green, Purple, Yellow, Red, Cyan, Orange, Blue];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Gem {
    Green,
    Purple,
    Yellow,
    Red,
    Cyan,
    Orange,
    Blue,
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    PlayerWin,
    DealerWin,
    Draw,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "dealer: {}\nplayer: {}", self.dealer, self.player)
        write!(
            f,
            "{}",
            match self {
                Self::PlayerWin => "Player Wins",
                Self::DealerWin => "Dealer Wins",
                Self::Draw => "Draw",
            }
        )
    }
}

use Gem::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    gems: Vec<Gem>,
    hand_type: HandType,
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    Nothing,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind, // TODO: 5 of a kind??
}

impl HandType {
    fn to_ranking(&self) -> usize {
        match self {
            Nothing => 0,
            Pair => 1,
            TwoPairs => 2,
            ThreeOfAKind => 3,
            FullHouse => 4,
            FourOfAKind => 5,
        }
    }
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "dealer: {}\nplayer: {}", self.dealer, self.player)
        write!(
            f,
            "{}",
            match self {
                Nothing => "Nothing",
                Pair => "Pair",
                TwoPairs => "2 Pairs",
                ThreeOfAKind => "3 Of A Kind",
                FullHouse => "Full House",
                FourOfAKind => "4 Of A Kind",
            }
        )
    }
}

use HandType::*;

impl Hand {
    fn new() -> Hand {
        Hand {
            gems: Vec::with_capacity(5),
            hand_type: Nothing,
        }
    }

    fn analyze(&mut self) {
        let count = self.gems.iter().fold(HashMap::new(), |mut acc, gem| {
            *acc.entry(gem).or_insert(0) += 1;
            acc
        });
        let pair_count = count.iter().filter(|(_, &val)| val == 2).count();
        let triple_count = count.iter().filter(|(_, &val)| val == 3).count();
        let quadruple_count = count.iter().filter(|(_, &val)| val >= 4).count();

        let hand_type = if quadruple_count == 1 {
            FourOfAKind
        } else if pair_count == 1 && triple_count == 1 {
            FullHouse
        } else if triple_count == 1 {
            ThreeOfAKind
        } else if pair_count == 2 {
            TwoPairs
        } else if pair_count == 1 {
            Pair
        } else {
            Nothing
        };
        self.hand_type = hand_type;
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "dealer: {}\nplayer: {}", self.dealer, self.player)
        write!(f, "{:?}", self.gems)
    }
}

#[derive(Debug)]
pub struct SimulationResult {
    pub outcome: f64,
    pub index: usize,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.outcome)
    }
}

// TODO: refactor player/dealer arrays into a Hand struct which implements partial eq? so we can do
// let winner = match cmp::sasldfjasf(playerHand, dealerHand) { case cmp::Greater => Winner::Player
// ...} etc.
/*
fn evaluate_hand(hand: &[Gem; 5]) -> u32 {
    // give a score to a hand...
}
*/

fn draw_hand(rng: &mut ProvablyFairRNG<f64>) -> Hand {
    let mut hand = Hand::new();
    for _ in 0..5 {
        let idx = (rng.next().unwrap() * 7.) as usize;
        hand.gems.push(GEM_ORDER[idx]);
    }
    hand.analyze();
    hand
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

    println!("idx: {}", idx);

    SimulationResult {
        outcome: 0.,
        index: idx as usize,
    }
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
    fn simulate_dice_roll() {
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
