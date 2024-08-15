use std::io::{self, Write};
mod features;

fn read_input(prompt: &str) -> io::Result<String> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn compress_and_read(folder: &str, compressed_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    features::compact::main(folder, compressed_file)?;
    println!("Compact with success");
    features::read::main(compressed_file)?;
    Ok(())
}

fn main() {
    match (read_input("Digite o nome da pasta que deseja compactar"), read_input("Digite o nome do arquivo comprimido")) {
        (Ok(folder_to_compress_name), Ok(compressed_file_name)) => {
            if let Err(e) = compress_and_read(&folder_to_compress_name, &compressed_file_name) {
                eprintln!("Error: {:?}", e);
            }
        },
        (Err(e), _) | (_, Err(e)) => eprintln!("Erro ao ler o que digitou: {:?}", e),
    }
}
