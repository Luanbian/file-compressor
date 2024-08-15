use std::io;

mod features;

fn main() {
    let mut folder_to_compress_name = String::new();
    println!("Digite o nome da pasta que deseja compactar");
    match io::stdin().read_line(&mut folder_to_compress_name) {
        Ok(_) => {
            let mut compressed_file_name = String::new();
            println!("Digite o nome do arquivo comprimido");
            match io::stdin().read_line(&mut compressed_file_name)  {
                Ok(_) => {
                    match features::compact::main(folder_to_compress_name.trim(), compressed_file_name.trim()) {
                        Ok(_) => {
                            println!("Compact with success");
                            match  features::read::main(&compressed_file_name.trim()) {
                                Ok(_) => {},
                                Err(e) => eprintln!("Error to read file: {:?}", e)
                            }
                        },
                        Err(e) => eprintln!("Error to compile: {:?}", e)
                    }
                },
                Err(e) => eprintln!("Erro ao ler o que digitou: {:?}", e)
            }
            
        },
        Err(e) => eprintln!("Erro ao ler o que digitou: {:?}", e)
    }
}





