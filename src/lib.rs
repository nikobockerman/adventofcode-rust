#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

pub mod cli;
pub mod y2024;

mod inputs;
mod problem;

#[macro_use]
extern crate log;
