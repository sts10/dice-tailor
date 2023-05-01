use clap::{Parser, Subcommand};
use dicetailor::*;

/// Provides guidance on how to fit dice values to each word
/// in a word list.
#[derive(Parser)]
#[clap(version, about, name = "dice-tailor")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Given list length, recommend a "fit"
    Measure {
        /// Set as a constant the number of dice sides (Optional)
        #[clap(short = 's', long = "sides")]
        sides: Option<i32>,

        /// Length of initial list
        #[clap(name = "Length of initial list", required = true)]
        list_length: i32,
    },

    /// Draw charts
    Draw {
        /// Set as a constant the number of dice sides (Optional)
        #[clap(short = 's', long = "sides")]
        sides: Option<i32>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Measure { sides, list_length } => measure(*list_length, *sides),
        Commands::Draw { sides } => make_plots(*sides),
    }
}
