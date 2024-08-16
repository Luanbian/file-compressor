use std::io::{self, Write};

pub fn show_menu(itens: &[&str], exit: bool) -> u32 {
    clean_terminal();

    let welcome = String::from("Welcome to my file compressor");
    println!("{}", welcome);
    println!("{}", String::from("=").repeat(welcome.len()));

    for (i, item) in itens.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }

    println!("{}", if exit {"* - Sair"} else {""});
    println!("\nEscolha uma opção: ");
    std::io::stdout().flush().unwrap();

    let mut user_response = String::new();
    std::io::stdin().read_line(&mut user_response).unwrap();

    let selected_option: Result<u32, _> = user_response.trim().parse();

    match selected_option {
        Ok(option) => option,
        _ => 0,
    }

}

pub fn clean_terminal() {
    print!("{}c", 27 as char);
}

pub fn read_input(prompt: &str) -> io::Result<String> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}