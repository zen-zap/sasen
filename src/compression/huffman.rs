#![allow(unused)]
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn compress_file(input_file_path: &String, output_file_path: &String) {
    let in_path = PathBuf::from(input_file_path);
    let mut file = File::open(in_path).expect("Unable to open the file");
    let mut buffer = Vec::new();
    let bytes_read = file.read_to_end(&mut buffer).expect("Failed to read file");
    println!("Bytes read: {}", bytes_read);

    for byte in buffer.iter() {
        print!("{} ", byte);
    }

    println!("\nMoving on to Frequency Counting");
    // A byte is 8 bits, so 256 different values
    let mut counts = [0u32; 256];
    for byte in &buffer {
        counts[*byte as usize] += 1;
    }


    for (byte, &count) in counts.iter().enumerate() {
        if count > 0 {
            // If it's a printable ASCII character (like a-z, /, etc)
            if byte == (b' ' as usize)  || char::from(byte as u8).is_ascii_graphic() {
                println!("Byte: {:>3} ('{}') → Count: {}", byte, byte as u8 as char, count);
            } else {
                println!("Byte: {:>3}      → Count: {}", byte, count);
            }
        }
    }

    println!("");
}

// So, we serialize it as bits first, or just characters, then do frequency counting of the
// characters, then group them in a bottom-up approach? write the bits
// read them as bytes so that we can work with any type of data not just strings


pub fn decompress_file(input_file_path: &String, output_file_path: &String) {
    unimplemented!("Compression algorithm is not yet complete")
}
