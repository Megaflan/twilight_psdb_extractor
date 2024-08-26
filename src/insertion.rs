use std::fs::{File, ReadDir};
use std::io::{Read, Seek, SeekFrom, Write};

use crate::binary_helper::read_uint;

pub fn repack(mut stream: File, dir_entries: ReadDir) {
    let mut pointer_list: Vec<u64> = Vec::new();
    let entries: Vec<_> = dir_entries.collect();

    // Preparations
    stream.seek(SeekFrom::Start(0x0c)).expect("Could not seek position");
    let data_position = read_uint(&mut stream);
    stream.seek(SeekFrom::Start(data_position as u64)).expect("Could not seek position");

    // Write data
    for entry_result in entries {
        pointer_list.push(stream.stream_position().expect("Could not read stream position"));
        let entry = entry_result.expect("Failed to read directory entry");

        let mut data = Vec::new();
        let mut file = File::open(entry.path()).expect("Failed to open input file");
        file.read_to_end(&mut data).expect("Failed to read input file");

        stream.write_all(&data).expect("Could not write data into stream");        
    }

    // Write pointers
    stream.seek(SeekFrom::Start(0x0c)).expect("Could not seek position");
    for pointer in pointer_list {
        stream.write(&pointer.to_le_bytes()).expect("Could not write data into stream");        
    }
}
