use color_eyre::eyre::Result;
use clap::Parser;
use evrart::docker_hub::download_image;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Docker image to download
    image: String,
}


fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    download_image(&cli.image)?;

    Ok(())
}