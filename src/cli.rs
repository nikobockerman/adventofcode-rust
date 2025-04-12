use std::fmt::Write as _;
use std::process::ExitCode;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::debug;

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

#[derive(Args, Clone, Copy)]
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
        Commands::All => all(),
        Commands::Day(day_args) => day(*day_args),
        Commands::Single(single_args) => single(*single_args),
    }
}

fn all() -> Result<ExitCode> {
    multiple(crate::data::get_all_known_solver_ids())
}

fn day(args: DayArgs) -> Result<ExitCode> {
    let DayArgs { year, day } = args;
    multiple(crate::data::get_known_solver_ids_for_day(year, day))
}

fn multiple<I>(ids: I) -> Result<ExitCode>
where
    I: Iterator<Item = crate::problem::Id>,
{
    let mut all_passed = None;
    for id in ids {
        let input = crate::solver::Input::new(id)?;
        let output = input.solve();
        let analysis = output.analyze();

        let passed = !analysis.is_incorrect();
        *all_passed.get_or_insert(true) &= passed;

        let mut msg = format!("{id}: ");
        if analysis.is_incorrect() {
            let _ = write!(
                msg,
                "FAIL: Incorrect answer: {}. Correct is: {}",
                analysis.answer,
                analysis.correct_answer.unwrap()
            );
        } else {
            msg += "PASS";
        }
        println!("{msg}");
    }

    Ok(match all_passed {
        None => {
            println!("No answers known");
            ExitCode::SUCCESS
        }
        Some(true) => {
            println!("Finished with all passing");
            ExitCode::SUCCESS
        }
        Some(false) => {
            println!("Finished with failures");
            ExitCode::FAILURE
        }
    })
}

fn single(args: SingleArgs) -> Result<ExitCode> {
    let SingleArgs {
        year: year_arg,
        day,
        part: part_arg,
    } = args;
    let year = year_arg;
    let id = crate::problem::Id {
        year,
        day,
        part: match part_arg {
            1 => crate::problem::Part::P1,
            2 => crate::problem::Part::P2,
            _ => unreachable!(),
        },
    };
    debug!("Problem: {id}");

    let input = crate::solver::Input::new(id)?;
    let output = input.solve();
    let analysis = output.analyze();

    if analysis.is_incorrect() {
        assert!(analysis.correct_answer.is_some());
        eprintln!(
            "FAIL: Incorrect answer: {}. Correct is {}",
            analysis.answer,
            analysis.correct_answer.unwrap()
        );
        return Ok(ExitCode::from(2));
    }
    if analysis.is_correct() {
        println!("Answer is still correct: {}", analysis.answer);
    } else {
        println!("{}", analysis.answer);
    }
    Ok(ExitCode::SUCCESS)
}
