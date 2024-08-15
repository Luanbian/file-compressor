mod features;

fn main() {
    match  features::compact::main() {
        Ok(_) => {
            println!("Compact with success");
            match  features::read::main() {
                Ok(_) => {},
                Err(e) => eprintln!("Error to read file: {:?}", e)
            }
        },
        Err(e) => eprintln!("Error to compile: {:?}", e)
    }
}
