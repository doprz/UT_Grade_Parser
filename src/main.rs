use crate::network::fetch_and_download_grade_distributions;
use crate::parse::parse_csv_directory;

mod network;
mod parse;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fetch_and_download_grade_distributions().await?;
    parse_csv_directory("out", "out_parsed");

    Ok(())
}
