use std::{fs::File, io::Write, path::Path};

use zip::{write::{ExtendedFileOptions, FileOptions}, ZipWriter};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("example.zip");
    let file = File::create(&path)?;

    let mut zip = ZipWriter::new(file);

    zip.start_file::<_, ExtendedFileOptions>("readme.txt", FileOptions::default())?;
    zip.write_all(b"Hello world!\n")?;
    zip.finish()?;

    println!("Zip file created!");

    Ok(())
}