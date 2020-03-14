//! # provably fair baccarat
//!
//! Deterministically simulates a game of baccarat. Assumes an inifinite amount of card decks.

/*
use std::env;
use std::error::Error; use std::fs;
*/

mod card;
pub mod games;
mod rng;
mod wasm;

pub use rng::ProvablyFairRNG;
