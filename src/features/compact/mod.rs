use std::error::Error;
use std::io::{self, Read};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::write::{ExtendedFileOptions, FileOptions, ZipWriter};

pub fn main() -> Result<(), Box<dyn Error>> {
    let zip_path = Path::new("compressed_files.zip");
    let zip_file: File = File::create(&zip_path)?;

    let mut zip = ZipWriter::new(zip_file);

    let files_to_comprass: Vec<PathBuf> = vec![
        PathBuf::from("image.png"),
        PathBuf::from("image2.png")
    ];

    let options: FileOptions<ExtendedFileOptions> = FileOptions::default().compression_method(zip::CompressionMethod::DEFLATE);

    for file_path in &files_to_comprass {
        let file = File::open(file_path)?;
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        zip.start_file(file_name, options.clone())?;

        let mut buffer = Vec::new();
        io::copy(&mut file.take(u64::MAX), &mut buffer)?;

        zip.write_all(&buffer)?;
    }

    zip.finish()?;

    println!("Files compressed successfully to {:?}", zip_path);

    Ok(())
}