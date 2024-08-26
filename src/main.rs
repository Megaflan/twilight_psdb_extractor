mod extraction;
mod insertion;
mod binary_helper;

use std::fs::{read_dir, File};
use std::io::{Read, Write};
use std::{env, fs, io};
use std::path::Path;

use extraction::read_file;
use insertion::repack;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <mode> <file>", args[0]);
        eprintln!("Modes:");
        eprintln!("-e: Extract the PSDB file.");        
        eprintln!("-i: Repack the PSDB file.");        
        std::process::exit(1);
    }

    let path = Path::new(&args[2]);
    let mode = &args[1];

    match mode.as_str() {
        "-e" => {
            // Extraction
            let entries = read_file(&path);
            // Write files
            let filename_no_ext = Path::new(path).file_stem().unwrap().to_str().unwrap();
            let dir_path = Path::new(filename_no_ext);
            fs::create_dir_all(&dir_path).expect("Failed to create directory");

            for entry in entries {
                let output_path = dir_path.join(format!("{}_{}.{}", filename_no_ext, format!("{:08}", entry.offset), &entry.format));
                let mut output_file = File::create(&output_path).expect("Failed to create output file");
                output_file.write_all(&entry.data).expect("Failed to write to output file");
            }
        }
        "-i" => {
            let mut existing_file = File::open(path).expect("Failed to open the existing file");
            let mut existing_data = Vec::new();
            existing_file.read_to_end(&mut existing_data).expect("Failed to read the existing file");

            let entries = read_dir(path.parent().expect("Invalid path")).expect("Failed to read directory");

            let output_path = path.join(".modded");
            let mut output_file = File::create(&output_path).expect("Failed to create output file");
            output_file.write_all(&existing_data).expect("Failed to write to output file");
            repack(output_file, entries);
        }
        _ => {
            eprintln!("Unknown mode: {}", mode);
        }
    }
    Ok(())
}
