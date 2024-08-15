use std::error::Error;
use std::io::{self, Read};
use std::fs::{read_dir, File};
use std::io::Write;
use std::path::Path;
use std::u64;
use zip::write::{ExtendedFileOptions, FileOptions, ZipWriter};

pub fn main() -> Result<(), Box<dyn Error>> {
    let dir_path = Path::new("to_compress");
    if !dir_path.exists() {
        return Err(Box::from("Directory to_compress not found"));
    }

    let zip_path = Path::new("compressed_files.zip");
    let zip_file: File = File::create(&zip_path)?;

    let mut zip = ZipWriter::new(zip_file);

    let options: FileOptions<ExtendedFileOptions> = FileOptions::default().compression_method(zip::CompressionMethod::DEFLATE);

    for child in read_dir(dir_path)? {
        let child = child?;
        let file_path = child.path();

        if file_path.is_file() {
            let file = File::open(&file_path)?;
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            zip.start_file(file_name, options.clone())?;

            let mut buffer = Vec::new();
            io::copy(&mut file.take(u64::MAX), &mut buffer)?;

            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;

    println!("Files compressed successfully to {:?}", zip_path);

    Ok(())
}