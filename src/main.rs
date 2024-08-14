mod compact;

fn main() {
    println!("Hello, world!");
    match compact::compact::main() {
        Ok(_) => println!("Compact with success"),
        Err(e) => eprintln!("Error to compile: {:?}", e)
    }
}
