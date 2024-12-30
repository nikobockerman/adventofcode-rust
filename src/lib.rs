#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

pub mod cli;

mod answer;
mod data;
mod inputs;
mod problem;
mod solver;

mod y2024;

type SolverFunc = fn(&'static str) -> crate::answer::Answer;

#[macro_use]
extern crate log;
