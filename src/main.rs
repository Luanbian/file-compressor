mod features;

fn main() {
    println!("Hello, world!");
    match  features::compact::main() {
        Ok(_) => println!("Compact with success"),
        Err(e) => eprintln!("Error to compile: {:?}", e)
    }
}
