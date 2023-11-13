use color_eyre::eyre::Result; // new!
use clap::Parser;
use evrart::docker_hub::download_image;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Docker image to download
    image: String,
}


fn main() -> Result<()> {
    //    ^^^^^^^^^^^^^ new!
    color_eyre::install()?;  // also new!

    let cli = Cli::parse();

    download_image(&cli.image)?;
    //                        ^ new!

    Ok(())  // we have to return a result now
}