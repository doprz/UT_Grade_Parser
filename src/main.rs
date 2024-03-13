use crate::network::fetch_and_download_grade_distributions;
use crate::parse::parse_csv_directory;

use clap::{Parser, Subcommand};

mod network;
mod parse;

#[derive(Subcommand)]
enum Commands {
    /// Fetch and download grade distributions
    Download {
        // /// The semester range to download grade distributions for
        // #[clap(short, long)]
        // years: Vec<u16>,
    },
    /// Parse CSV files
    Parse {
        // /// The input directory containing CSV files
        // #[clap(short, long)]
        // input: std::path::PathBuf,
        // /// The output directory to write parsed CSV files
        // #[clap(short, long)]
        // output: std::path::PathBuf,
    },
    /// Run all commands
    All,
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download {} => {
            fetch_and_download_grade_distributions().await?;
            println!("fetch_and_download_grade_distributions()");
        }
        Commands::Parse {} => {
            parse_csv_directory("out", "out_parsed");
            println!("parse_csv_directory()");
        }
        Commands::All => {
            fetch_and_download_grade_distributions().await?;
            parse_csv_directory("out", "out_parsed");
            println!("fetch_and_download_grade_distributions()");
            println!("parse_csv_directory()");
        }
    }

    Ok(())
}
