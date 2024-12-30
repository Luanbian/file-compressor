use clap::Parser; 
use std::path::Path;

mod features;
mod interface;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long)]
    selected: String,
}

fn compress_and_read(path: &Path, compressed_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    features::compact::main(path, compressed_file)?;
    println!("Compact with success");
    features::read::main(compressed_file)?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    
    let input_path = Path::new(&cli.input);
    let output_file_name = &cli.output;
    let selected_option = &cli.selected;

    match selected_option.as_str() {
        "compress" => {
            compress_and_read(input_path, output_file_name).expect("Error compressing files");
        },
        _ => {
            eprintln!("Invalid option: {}", cli.selected);
            std::process::exit(1);
        }
    }
}
