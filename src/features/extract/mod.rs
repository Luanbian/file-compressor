use std::{error::Error, fs::File, io, path::Path};

use zip::ZipArchive;

pub fn main(extract_file: &str, extracted_name: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&extract_file);
    if !path.exists() {
        return Err(Box::from(format!("file {} to extract not found", extract_file)));
    }

    let file = File::open(&path)?;

    let mut archive = ZipArchive::new(file)?;
    let extraction_dir = Path::new(extracted_name);

    if !extraction_dir.exists() {
        std::fs::create_dir(extraction_dir)?;
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_owned();

        let target_path = extraction_dir.join(file_name);
        if let Some(parent_dir) = target_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let mut output_file = File::create(&target_path)?;

        io::copy(&mut file, &mut output_file)?;
    }
    println!("Files successfully extracted to {:?}", extraction_dir);
    Ok(())
}