use clap::{Parser, Subcommand};
use txtimg::*;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Text to image
    T {
        #[arg(required = true)]
        text: String,
        #[arg(short, long, default_value = "output.png")]
        output_path: String,
    },
    /// Image to text
    M {
        #[arg(required = true)]
        image_path: String,
    },
    /// File to image
    F {
        #[arg(required = true)]
        file_path: String,
        #[arg(short, long, default_value = "output.png")]
        output_path: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::T { text, output_path } => text_to_image(&text, &output_path),
        Commands::M { image_path } => {
            let decompressed_text = image_to_text(&image_path);
            println!("{}", decompressed_text);
        }
        Commands::F {
            file_path,
            output_path,
        } => {
            let text = std::fs::read_to_string(file_path).expect("Unable to read file");
            text_to_image(&text, &output_path);
        }
    }
}
