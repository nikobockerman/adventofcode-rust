use std::process::ExitCode;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

use crate::{
    inputs::get_input,
    problem::{Id, Part},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    All,
    Day(DayArgs),
    Single(SingleArgs),
}

#[derive(Args)]
struct DayArgs {
    #[arg(long, short, default_value_t = 2024)]
    year: u16,
    day: u8,
}

#[derive(Args, Clone, Copy)]
struct SingleArgs {
    #[arg(long, short, default_value_t = 2024)]
    year: u16,
    day: u8,
    #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,
}

#[allow(clippy::missing_errors_doc)]
pub fn cli() -> Result<ExitCode> {
    let cli = Cli::parse();
    if let Some(level) = cli.verbose.log_level() {
        simple_logger::init_with_level(level)?;
    }

    match &cli.command {
        Commands::All => {
            println!("TODO: Run all solutions");
        }
        Commands::Day(day_args) => {
            println!("TODO: Run day {}", day_args.day);
        }
        Commands::Single(single_args) => {
            single(*single_args)?;
        }
    }

    trace!("TRACE");
    debug!("DEBUG");
    info!("INFO");
    warn!("WARN");
    error!("ERROR");
    Ok(ExitCode::SUCCESS)
}

fn single(args: SingleArgs) -> Result<ExitCode> {
    let SingleArgs {
        year: year_arg,
        day,
        part: part_arg,
    } = args;
    let year = year_arg;
    let id = Id {
        year,
        day,
        part: match part_arg {
            1 => Part::P1,
            2 => Part::P2,
            _ => unreachable!(),
        },
    };
    debug!("Problem: {}", id);

    let input = get_input(year, day).ok_or_else(|| anyhow::anyhow!("Input not found for {id}"))?;

    #[allow(clippy::match_single_binding)]
    let correct_answer = match id {
        _ => None,
    };

    let result = match id {
        Id {
            year: 2024,
            day: 5,
            part: Part::P1,
        } => crate::y2024::d5::p1(input)?,
        _ => anyhow::bail!("Solver not found for {id}"),
    };

    let (is_correct, is_incorrect) = match correct_answer {
        None => (false, false),
        Some(answer) => (result == answer, result != answer),
    };

    if is_incorrect {
        eprintln!(
            "Incorrect answer: {}. Correct is {}",
            result,
            correct_answer.unwrap()
        );
        return Ok(ExitCode::from(2));
    }
    if is_correct {
        println!("Answer is still correct: {result}");
    } else {
        println!("{result}");
    }
    Ok(ExitCode::SUCCESS)
}
