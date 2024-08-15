use std::{error::Error, fs::File, path::Path};
use std::io::Read;
use zip::ZipArchive;

pub fn main(compressed_file: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&compressed_file);

    let file = File::open(&path)?;
    let mut archive = ZipArchive::new(&file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        println!("File name: {}", file.name());
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        if let Ok(content) = String::from_utf8(buffer) {
            println!("File content: {}", content);
        } else {
            println!("File content isn't UTF-8");
        }
    }
    Ok(())
}