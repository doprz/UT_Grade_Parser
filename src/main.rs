mod database;
mod network;
mod parse;

use crate::database::insert_data_into_db_from_dir;
use crate::network::fetch_and_download_grade_distributions;
use crate::parse::parse_csv_directory;

use clap::{Parser, Subcommand};

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
    /// Create a sqlite3 database
    Database,
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

async fn download() -> Result<(), Box<dyn std::error::Error>> {
    println!("fetch_and_download_grade_distributions()");
    fetch_and_download_grade_distributions().await?;

    Ok(())
}

fn parse() {
    println!("parse_csv_directory()");
    parse_csv_directory("out", "out_parsed");
}

fn database() -> Result<(), Box<dyn std::error::Error>> {
    println!("insert_data_into_db_from_dir()");
    insert_data_into_db_from_dir("out_parsed")?;

    Ok(())
}

async fn all() -> Result<(), Box<dyn std::error::Error>> {
    download().await?;
    parse();
    database()?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download {} => download().await?,
        Commands::Parse {} => parse(),
        Commands::Database => database()?,
        Commands::All => all().await?,
    }

    Ok(())
}
