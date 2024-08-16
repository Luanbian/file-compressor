use std::process::exit;

use utils::{show_menu, read_input};
mod features;
mod utils;


fn compress_and_read(folder: &str, compressed_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    features::compact::main(folder, compressed_file)?;
    println!("Compact with success");
    features::read::main(compressed_file)?;
    Ok(())
}

fn extract(folder: &str, extracted_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    features::extract::main(folder, extracted_name)?;
    println!("Extracted with success");
    Ok(())
}

fn main() {
    loop {
        let itens = ["Compactar", "Extrair"];
        let selected = show_menu(&itens, true);

        match selected {
            1 => {
                match (read_input("Qual o nome da pasta que deseja compactar?"), read_input("Qual será o nome do arquivo compactado?")) {
                    (Ok(folder), Ok(compressed_file)) => {
                        if let Err(e) = compress_and_read(&folder, &compressed_file)  {
                            eprintln!("Erro ao comprimir: {:?}", e);
                        }
                    },
                   (Err(_e),_) | (_, Err(_e)) => eprintln!("Erro ao ler o que você digitou")
                }
            },
            2 => {
                match (read_input("Qual é o nome do arquivo que será extraido?"), read_input("Qual será o nome do arquivo extraido?")) {
                    (Ok(folder), Ok(extracted_name)) => {
                        if let Err(e) = extract(&folder, &extracted_name) {
                            eprintln!("Erro ao extrair: {:?}", e);
                        }
                    },
                    (Err(_e), _) | (_,Err(_e)) => eprintln!("Erro ao ler o que digitou")
                }
            }
            _ => exit(0)
        }
    }
}
