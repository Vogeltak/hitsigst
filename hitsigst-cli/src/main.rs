use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod prepare;
mod typst;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Process song data for Hitsigst
    Prepare {
        /// CSV file with song title, artist, and year of release
        #[arg(short, long, value_name = "FILE")]
        from: PathBuf,

        /// Output path for the Hitsigst data file
        #[arg(short, long, value_name = "FILE", default_value_t = String::from("hitsigst.json"))]
        output: String,

        /// Output directory for downloaded songs
        #[arg(short, long, value_name = "DIR", default_value_t = String::from("hitsigst-songs"))]
        download_dir: String,

        /// Skip downloading songs
        #[arg(short, long, default_value_t = false)]
        no_download: bool,
    },
    /// Generate game cards using Typst
    Typst {
        /// Hitsigst data file
        #[arg(short, long, value_name = "FILE")]
        from: PathBuf,

        /// Output path for the game cards PDF
        #[arg(short, long, value_name = "FILE", default_value_t = String::from("hitsigst.pdf"))]
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
            no_download,
        }) => prepare::start(from, output, download_dir, *no_download)?,
        Some(Commands::Typst { from, output }) => typst::build(from, output)?,
        None => todo!(),
    };

    Ok(())
}
