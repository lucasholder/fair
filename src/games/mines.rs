//! # provably fair Mines
//!

/*

Mines A mine game is generated with 24 separate game events, in the form of mines on the board.
Each float is multiplied by the number of possible unique tiles still remaining on the board.
This is done by subtracting the number of tiles remaining by 1 for each iteration of game event
result generated using the corresponding float provided. The location of the mine is plotted
using a grid position from left to right, top to bottom.

The fisher-yates shuffle implementation is utilised to prevent duplicate possible hits being
generated. Between 1 and 24 game event results are used, based on the settings chosen.
*/

pub use crate::rng::{ProvablyFairConfig, ProvablyFairRNG};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct SimulationResult {
    pub squares: Vec<u8>,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = format!("");
        for idx in 0..25 {
            if idx % 5 == 0 {
                grid = format!("{}\n", grid);
            }
            let is_mine = self.squares.iter().find(|&&x| x == idx).is_some();
            let icon = if is_mine { "💣 " } else { "💠" };
            grid = format!("{}{}\t ", grid, icon);
        }

        write!(f, "Squares: {:?}\n\n{}", self.squares, grid)
    }
}

pub fn simulate(config: ProvablyFairConfig, total_mines: u8) -> SimulationResult {
    assert!(total_mines >= 1);
    assert!(total_mines <= 24);
    let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::from_config(config);

    let mut remaining_squares: Vec<u8> = (0..25).collect();

    let picked_squares: Vec<_> = (0..total_mines)
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
        assert_eq!(simulate(config, 1).squares, vec![18]);
        let config = ProvablyFairConfig::new("client seed", "server seed", 1);
        assert_eq!(simulate(config, 3).squares, vec![18, 15, 5]);
    }
}
