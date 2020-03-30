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
    pub dealer: Hand,
    pub player: Hand,
    pub outcome: Outcome, // pub winner: Winner,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "dealer: {}\nplayer: {}", self.dealer, self.player)
        write!(
            f,
            "Dealer ({}): {}\nPlayer ({}): {}\nOutcome: {}",
            self.dealer.hand_type, self.dealer, self.player.hand_type, self.player, self.outcome
        )
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

/// Simulates a game of diamond poker.
///
/// # Example
///
/// ```
/// use fair::{games, ProvablyFairConfig};
///
/// let config = ProvablyFairConfig::new("some client seed", "some server seed", 1);
/// let result = games::plinko::simulate(config);
/// ```
///
pub fn simulate(config: ProvablyFairConfig) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);

    let dealer = draw_hand(&mut rng);
    let player = draw_hand(&mut rng);

    // let winner = evaluate_winner(dealer, player);

    // let outcome = (rng.next().unwrap() * 10001.) as u32;
    let outcome = match player
        .hand_type
        .to_ranking()
        .cmp(&dealer.hand_type.to_ranking())
    {
        cmp::Ordering::Greater => Outcome::PlayerWin,
        cmp::Ordering::Less => Outcome::DealerWin,
        cmp::Ordering::Equal => Outcome::Draw,
    };

    SimulationResult {
        dealer,
        player,
        outcome,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulate_dice_roll() {
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        let result = simulate(config);
        // println!("{:?}", result);
        assert_eq!(result.dealer.gems, vec![Orange, Cyan, Purple, Blue, Red]);
        assert_eq!(result.player.gems, vec![Blue, Cyan, Cyan, Blue, Green]);
    }
}
