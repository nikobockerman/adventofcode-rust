#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

use anyhow::Result;

fn main() -> Result<()> {
    aoc::cli::cli()?;
    Ok(())
}
