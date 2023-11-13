use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Docker image to download
    image: String,
}


fn main() {
    let cli = Cli::parse();

    println!("Downloading image '{}'", cli.image);
}