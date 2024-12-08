use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod prepare;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Process song data for Hitrelease
    Prepare {
        /// CSV file with song title, artist, and year of release
        #[arg(short, long, value_name = "FILE")]
        from: PathBuf,

        /// Output path for the Hitrelease data file
        #[arg(short, long, value_name = "FILE", default_value_t = String::from("hitrelease.json"))]
        output: String,

        /// Output directory for downloaded songs
        #[arg(short, long, value_name = "DIR", default_value_t = String::from("hitrelease-songs"))]
        download_dir: String,
    },
    /// Generate game cards using Typst
    Typst {
        /// Hitrelease data file
        #[arg(short, long, value_name = "FILE")]
        from: PathBuf,

        /// Output path for the game cards PDF
        #[arg(short, long, value_name = "FILE", default_value_t = String::from("hitrelease.pdf"))]
        output: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Prepare {
            from,
            output,
            download_dir,
        }) => prepare::start(from, output, download_dir)?,
        Some(Commands::Typst { from: _, output: _ }) => todo!(),
        None => todo!(),
    };

    Ok(())
}
