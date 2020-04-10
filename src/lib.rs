//! # provably fair baccarat
//!
//! Deterministically simulates a game of baccarat. Assumes an inifinite amount of card decks.

/*
use std::env;
use std::error::Error; use std::fs;
*/
// use std::process;

mod card;
pub mod games;
mod rng;
pub mod utils;
mod wasm;

pub use rng::{ProvablyFairConfig, ProvablyFairRNG};
