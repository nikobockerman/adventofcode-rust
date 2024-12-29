#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

use std::process::ExitCode;

use anyhow::Result;

fn main() -> Result<ExitCode> {
    aoc::cli::cli()
}
