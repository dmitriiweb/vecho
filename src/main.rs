use clap::Parser;

mod path_detector;

/// Vecho is a CLI tool designed to get description of an image using the ollama API.
#[derive(Parser, Debug)]
#[command(version=None, about, long_about=None)]
struct Args {
    /// Image path, could be a local file path or an URL
    #[arg(short, long)]
    image: String,

    /// ollama url, url to the ollama API
    #[arg(short, long, default_value = "http://localhost:11434/api")]
    ollama: String,

    /// what model to use
    #[arg(short, long, default_value = "llava")]
    model: String,

    /// language of the output description
    #[arg(short, long, default_value = "American_English")]
    language: String,
}

fn main() {
    let args = Args::parse();
    let path_type = path_detector::path_detector(&args.image);
    println!("{:?}", args);
    println!("{:?}", path_type);
}
