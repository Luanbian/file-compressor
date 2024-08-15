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

fn extract(folder: &str, extracted_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    features::extract::main(folder, extracted_name)?;
    println!("Extracted with success");
    Ok(())
}

fn main() {
    match read_input("O que deseja fazer? \n 1 - Compactar \n 2 - Extrair") {
        Ok(selected) => {
            match selected.as_str() {
                "1" => {
                    match (read_input("Qual o nome da pasta que deseja compactar?"), read_input("Qual será o nome do arquivo compactado?")) {
                        (Ok(folder), Ok(compressed_file)) => {
                            if let Err(e) = compress_and_read(&folder, &compressed_file) {
                                eprintln!("Error: {:?}", e);
                            }
                        },
                        (Err(e), _) | (_, Err(e)) => eprintln!("Erro ao ler o que digitou: {:?}", e),
                    }
                },
                "2" => {
                    match (read_input("Qual o nome da pasta que deseja extrair?"), read_input("Qual será o nome da pasta extraida?")) {
                        (Ok(extract_file), Ok(extracted_name)) => {
                            if let Err(e) = extract(&extract_file, &extracted_name) {
                                eprint!("Error: {:?}", e);
                            }
                        },
                        (Err(e), _) | (_, Err(e)) => eprint!("Erro ao ler o que digitou: {:?}", e)
                    }
                }
                _ => {
                    eprintln!("Invalid option");
                },
            }
        },
        Err(_e) => eprintln!("Selecione uma opção válida")
    }
}
