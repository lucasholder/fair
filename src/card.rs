use std::fmt;

use crate::rng::ProvablyFairRNG;
use CardRank::*;
use CardSuite::*;

pub const CARD_RANK_ORDER: &[CardRank; 13] = &[
    TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, QUEEN, KING, ACE,
];

pub const CARD_SUITE_ORDER: &[CardSuite; 4] = &[DIAMOND, HEART, SPADE, CLUB];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CardRank {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

impl CardRank {
    fn to_int(&self) -> u8 {
        match self {
            TWO => 2,
            THREE => 3,
            FOUR => 4,
            FIVE => 5,
            SIX => 6,
            SEVEN => 7,
            EIGHT => 8,
            NINE => 9,
            TEN => 10,
            JACK => 11,
            QUEEN => 12,
            KING => 13,
            ACE => 14,
        }
    }
}

impl fmt::Display for CardRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_str = self.to_int().to_string();
        let s = match self {
            JACK => "J",
            QUEEN => "Q",
            KING => "K",
            ACE => "A",
            _ => &rank_str,
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CardSuite {
    DIAMOND,
    CLUB,
    SPADE,
    HEART,
}

impl fmt::Display for CardSuite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DIAMOND => "♦",
            CLUB => "♣",
            SPADE => "♠",
            HEART => "♥",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card {
    rank: CardRank,
    suite: CardSuite,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.suite, self.rank)
    }
}

// draw random card from an inifinite deck
impl Card {
    // deterministically returns a random card using rng
    pub fn random(rng: &mut ProvablyFairRNG<f64>) -> Card {
        let val = rng.next().unwrap();
        let idx = (val * 52.) as usize;
        Card::at_index(idx)
    }

    // Returns a card from a virtual ordered deck of 52 cards
    // the order is the following: ♦2, ♥2, ♠2, ♣2, ♦3, ♥3, ♠3...
    fn at_index(idx: usize) -> Card {
        let suite_idx = idx % 4;
        let suite = CARD_SUITE_ORDER[suite_idx];

        let rank_idx = idx / 4;
        let rank = CARD_RANK_ORDER[rank_idx];
        let card = Card { suite, rank };
        card
    }

    // returns baccarat value of card
    pub fn to_baccarat_value(&self) -> u8 {
        let rank = self.rank;
        match rank {
            ACE => 1,
            JACK | QUEEN | KING => 0,
            _ => self.rank.to_int(),
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" - ");
        write!(f, "{}", s)
    }
}

impl Deck {
    pub fn from_rng(rng: &mut ProvablyFairRNG<f64>, count: u32) -> Deck {
        let mut cards: Vec<Card> = vec![];
        for _ in 0..count {
            let card = Card::random(rng);
            cards.push(card);
        }

        Deck { cards }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn index_into_deck() {
        // println!("{}", deck[0]);
        assert_eq!(
            Card::at_index(0),
            Card {
                suite: DIAMOND,
                rank: TWO
            }
        );
        let expected_deck_order = vec![
            "♦2", "♥2", "♠2", "♣2", "♦3", "♥3", "♠3", "♣3", "♦4", "♥4", "♠4", "♣4", "♦5", "♥5",
            "♠5", "♣5", "♦6", "♥6", "♠6", "♣6", "♦7", "♥7", "♠7", "♣7", "♦8", "♥8", "♠8", "♣8",
            "♦9", "♥9", "♠9", "♣9", "♦10", "♥10", "♠10", "♣10", "♦J", "♥J", "♠J", "♣J", "♦Q", "♥Q",
            "♠Q", "♣Q", "♦K", "♥K", "♠K", "♣K", "♦A", "♥A", "♠A", "♣A",
        ];
        for i in 0..52 {
            assert_eq!(Card::at_index(i).to_string(), expected_deck_order[i]);
        }
    }
}
