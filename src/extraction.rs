use std::io::{Seek, SeekFrom};
use std::fs::File;
use std::io::Read;

use crate::binary_helper;
use binary_helper::*;


#[derive(Debug)]
pub struct Entry {
    pub offset: u32,
    pub length: u32,
    pub format: String,
    pub data: Vec<u8>
}

pub fn read_file(file_path: &std::path::Path) -> Vec<Entry> {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut entries: Vec<Entry> = Vec::new();
    let mut pointers = Vec::new();

    let magic = read_uint(&mut file);
    if magic != 0x20534d54 {
        println!("Magic number does not match. Exiting.");
        return entries;
    }

    file.seek(SeekFrom::Start(0x8)).expect("Failed to seek entry position");
    
    loop {
        let offset = read_uint(&mut file);
        if offset == 0 { break; }
        pointers.push(offset);
    }

    for i in 0..pointers.len() as usize {
        let offset = pointers[i];
        let length = if i < pointers.len() - 1 {
            pointers[i + 1] - offset
        } else {
            let end_of_file = file.seek(SeekFrom::End(0)).expect("Failed to seek to end of file");
            end_of_file as u32 - offset
        };

        file.seek(SeekFrom::Start(offset as u64)).expect("Failed to seek to entry position");

        let data = read_bytes(&mut file, length as usize);

        entries.push(Entry {
            offset,
            length,
            format: identify_format(data[..4].to_vec()),
            data,
        });
    }
    
    entries
}

fn identify_format(data: Vec<u8>) -> String {    
    if data.starts_with(&[0x00, 0x10, 0x80, 0x00]) {
        "tim".to_string() //TIM (Sony PlayStation Typical image Format )
    } else {        
        "bin".to_string() //BIN (Binary Data)
    }
}