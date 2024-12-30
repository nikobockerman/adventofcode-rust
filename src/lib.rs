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

mod answer;
mod data;
mod inputs;
mod problem;
mod solver;

type SolverFunc = fn(&'static str) -> anyhow::Result<crate::answer::Answer>;

#[macro_use]
extern crate log;
