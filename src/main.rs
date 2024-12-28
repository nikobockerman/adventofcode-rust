use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

#[macro_use]
extern crate log;

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
    day: u8,
}

#[derive(Args)]
struct SingleArgs {
    day: u8,
    part: u8,
}

fn main() {
    let cli = Cli::parse();
    simple_logger::init_with_level(cli.verbose.log_level().unwrap()).unwrap();

    match &cli.command {
        Commands::All => {
            println!("TODO: Run all solutions");
        }
        Commands::Day(day_args) => {
            println!("TODO: Run day {}", day_args.day);
        }
        Commands::Single(single_args) => {
            println!(
                "TODO: Run single: day {} part {}",
                single_args.day, single_args.part
            );
        }
    }

    trace!("TRACE");
    debug!("DEBUG");
    info!("INFO");
    warn!("WARN");
    error!("ERROR");
}
