//! # provably fair baccarat
//!
//! Deterministically simulates a game of baccarat. Assumes an inifinite amount of card decks.

/*
use std::env;
use std::error::Error; use std::fs;
*/

use crate::card::Card;
pub use crate::rng::ProvablyFairRNG;

use std::cmp::Ordering;
use std::fmt;
use BaccaratCardRecipient::*;

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Banker,
    Player,
    Tie,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Outcome::Banker => "Banker won",
            Outcome::Player => "Player won",
            Outcome::Tie => "It's a tie",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
struct SimulationResultTotals {
    player: u32,
    banker: u32,
}

#[derive(Debug)]
struct Step(BaccaratCardRecipient, Card);

#[derive(Debug)]
pub struct SimulationResult {
    outcome: Outcome,
    totals: SimulationResultTotals,
    steps: Vec<Step>,
}

impl SimulationResult {
    fn from_steps(steps: Vec<Step>) -> SimulationResult {
        let totals = SimulationResultTotals {
            player: sum_cards_player(&steps),
            banker: sum_cards_banker(&steps),
        };
        let outcome = match totals.player.cmp(&totals.banker) {
            Ordering::Less => Outcome::Banker,
            Ordering::Greater => Outcome::Player,
            Ordering::Equal => Outcome::Tie,
        };
        SimulationResult {
            outcome,
            totals,
            steps,
        }
    }
}
impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn pretty_print_steps(recipient: &BaccaratCardRecipient, steps: &Vec<Step>) -> String {
            let step_str = steps
                .iter()
                .filter_map(|Step(r, c)| {
                    if r == recipient {
                        Some(c.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join(" - ");
            let total = sum_cards(recipient, steps);
            format!("{} ({}): {}", recipient, total, step_str)
        }
        let banker = pretty_print_steps(&BANKER, &self.steps);
        let player = pretty_print_steps(&PLAYER, &self.steps);

        write!(f, "{}\n\n{}\n{}", self.outcome, player, banker)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum BaccaratCardRecipient {
    BANKER,
    PLAYER,
}
impl fmt::Display for BaccaratCardRecipient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BANKER => f.write_str("Banker"),
            PLAYER => f.write_str("Player"),
        }
    }
}

fn baccarat_add(left: u32, right: u32) -> u32 {
    (left + right) % 10
}

fn sum_cards(for_recipient: &BaccaratCardRecipient, steps: &Vec<Step>) -> u32 {
    steps
        .iter()
        .filter(|Step(recipient, _)| recipient == for_recipient)
        .fold(0, |acc, Step(_, card)| {
            baccarat_add(acc, card.to_baccarat_value() as u32)
        })
}

fn sum_cards_player(steps: &Vec<Step>) -> u32 {
    sum_cards(&PLAYER, &steps)
}
fn sum_cards_banker(steps: &Vec<Step>) -> u32 {
    sum_cards(&BANKER, &steps)
}

/// Simulates a game of baccarat.
///
/// # Example
///
/// ```
///
/// let client_seed = "some client seed";
/// let server_seed = "some server seed";
/// let nonce = 1;
/// let result = fair::games::baccarat::simulate(
///   client_seed,
///   server_seed,
///   nonce,
/// );
/// // assert_eq!(result, vec!["todo", "todo"]);
/// ```
///
pub fn simulate(client_seed: &str, server_seed: &str, nonce: u64) -> SimulationResult {
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::new(client_seed, server_seed, nonce);

    // keep track of drawn cards
    let mut steps: Vec<Step> = vec![];

    steps.push(Step(PLAYER, Card::random(&mut rng)));
    steps.push(Step(PLAYER, Card::random(&mut rng)));
    steps.push(Step(BANKER, Card::random(&mut rng)));
    steps.push(Step(BANKER, Card::random(&mut rng)));

    // If either The player or banker or both achieve a total of 8 or 9
    // at this stage, the coup is finished and the result is announced:
    // a player win, a banker win, or tie.
    if sum_cards_banker(&steps) >= 8 || sum_cards_player(&steps) >= 8 {
        // This is called a "natural win"
        return SimulationResult::from_steps(steps);
    }

    // If neither hand has eight or nine, the drawing rules are applied
    // to determine whether the player should receive a third card.

    // If the player has an initial total of 6 or 7, he stands pat.
    if sum_cards_player(&steps) > 5 {
        // If the Player stands pat (or draws no new cards), the Banker draws with
        // a hand total of 0-5 and stays pat with a hand total of 6 or 7.
        if sum_cards_banker(&steps) <= 5 {
            steps.push(Step(BANKER, Card::random(&mut rng)));
        }
        return SimulationResult::from_steps(steps);
    }

    // If the player has an initial total of 0–5, he draws a third card.
    let player_third_card = Card::random(&mut rng);
    steps.push(Step(PLAYER, player_third_card));

    fn banker_should_draw_third_card(banker_total: u32, player_third_card: Card) -> bool {
        let rank = player_third_card.to_baccarat_value();

        match banker_total {
            0 | 1 | 2 => true,
            3 => rank != 8,
            4 => match rank {
                2 | 3 | 4 | 5 | 6 | 7 => true,
                _ => false,
            },
            5 => match rank {
                4 | 5 | 6 | 7 => true,
                _ => false,
            },
            6 => match rank {
                6 | 7 => true,
                _ => false,
            },
            7 => false,
            _ => {
                panic!(
                    "got an impossible value \"{}\" for banker total (>7), something with library!",
                    banker_total
                );
            }
        }
    }

    if banker_should_draw_third_card(sum_cards_banker(&steps), player_third_card) {
        steps.push(Step(BANKER, Card::random(&mut rng)));
    }

    SimulationResult::from_steps(steps)
}

#[cfg(test)]
mod test {
    use super::*;

    fn pretty_print_steps(steps: &Vec<Step>) -> Vec<String> {
        steps
            .iter()
            .map(|Step(recipient, card)| format!("{}: {}", recipient, card))
            .collect::<Vec<String>>()
    }

    #[test]
    fn simulate_five_cards_drawn() {
        let client_seed = "some client seed";
        let server_seed = "some server seed";
        let nonce = 2;
        let result = simulate(client_seed, server_seed, nonce);
        // println!("{:?}", result);
        assert_eq!(result.outcome, Outcome::Banker);

        assert_eq!(
            pretty_print_steps(&result.steps),
            vec![
                "Player: ♥Q",
                "Player: ♣Q",
                "Banker: ♥4",
                "Banker: ♥3",
                "Player: ♣10"
            ]
        );
    }

    #[test]
    fn simulate_four_cards_drawn() {
        let client_seed = "some client seed";
        let server_seed = "some server seed";
        let nonce = 1;
        let result = simulate(client_seed, server_seed, nonce);
        // println!("{:?}", result);
        assert_eq!(result.totals.player, 9);
        assert_eq!(result.totals.banker, 9);
        assert_eq!(result.outcome, Outcome::Tie);

        assert_eq!(
            pretty_print_steps(&result.steps),
            vec!["Player: ♠9", "Player: ♠Q", "Banker: ♦4", "Banker: ♠5"]
        );
    }
}
