#![allow(unused)]
use crate::{Node, CompressedFile};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::collections::{BinaryHeap, HashMap};
use std::sync::LazyLock;

pub fn compress_file(input_file_path: &String, output_file_path: &String) {

    let in_path = PathBuf::from(input_file_path);
    let file_size = get_file_size(Path::new(input_file_path));
    println!("File Size Before Compression: {}", file_size);
    let mut file = File::open(in_path).expect("Unable to open the file");
    let mut buffer = Vec::new();
    let bytes_read = file.read_to_end(&mut buffer).expect("Failed to read file");
    println!("Bytes read: {}", bytes_read);

    // for byte in buffer.iter() {
    //     print!("{} ", byte);
    // }

    println!("\n=====Moving on to Frequency Counting=====\n");
    // A byte is 8 bits, so 256 different values
    let mut counts = [0u32; 256];
    for byte in &buffer {
        counts[*byte as usize] += 1;
    }

    println!("Non Zero Frequency Counts...");
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

    println!("\n=====Making the Huffman Tree=====\n");

    let mut heap = BinaryHeap::new();

    for (byte, &count) in counts.iter().enumerate() {
        if count > 0 {
            heap.push(Node {
                cnt: count as usize,
                leaf: Some(byte as u8),
                children: None,
            });
        }
    }

    // //checking the heap
    // while (!heap.is_empty()) {
    //     println!("HEAP ELEMENT: {:?}", heap.pop());
    // }
    //
    // Yep ... it works!

    while(heap.len() >= 2) {
        // pop the first 2 nodes
        let node1 = heap.pop().expect("Unable to get Node from heap");
        let node2 = heap.pop().expect("Unable to get Node from heap");

        let new_cnt = node1.cnt + node2.cnt;
        let new_node = Node {
            cnt: new_cnt,
            leaf: None,
            children: Some(Box::new((node1, node2))), // the smaller node goes to the left, the larger to the right
        };

        heap.push(new_node);
    } 

    let root = heap.pop().unwrap();
    //println!("Root Node: {:#?}", root);
    println!("Root Node has count: {}", root.cnt);

    println!("Walking the Tree Now~");
    // get the lookup table for the encoder
    let mut lookup_table: HashMap<u8, Vec<bool>> = HashMap::new();
    let mut level_encode: Vec<bool> = Vec::new();
    // This one updates our HashMap in-place
    walk_huffman_tree(&root, &mut level_encode, &mut lookup_table);


    for (byte, code) in &lookup_table {
        let repr: String = code.iter().map(|&b| if b { '1' } else { '0' }).collect();
        println!("Byte: {:?} → {}", *byte as char, repr);
    }

    println!("Moving on to bitstream generation");
    // we take our original file content and swap it with a bitstream from the lookup_table
    let mut bitstream: Vec<bool> = Vec::new();

    for byte in &buffer {
        if let Some(code) = lookup_table.get(byte) {
            bitstream.extend(code); // code is a Vec<bool> so we gotta se extend
        } else {
            panic!("Byte {} not found in huffman tree", byte);
        }
    }

    println!("Total bits in compressed stream: {}", bitstream.len());
    println!("Approx size in bytes: {:.2}", bitstream.len() as f64 / 8.0);

    // byte-packing 
    let mut packed_bytes: Vec<u8> = Vec::new();
    let mut curr_byte = 0u8;
    let mut bit_cnt = 0;

    for &bit in &bitstream {

        curr_byte <<= 1; // shifting the current bits to left
        if bit {
            // If bit is true, set the lowest bit to 1
            curr_byte |= 1;
        }
        bit_cnt += 1;

        if bit_cnt == 8 {
            packed_bytes.push(curr_byte);
            curr_byte = 0;
            bit_cnt = 0;
        }
    }

    // leftover bits
    if bit_cnt > 0 {
        curr_byte <<= (8-bit_cnt);
        packed_bytes.push(curr_byte);
    }

    let mut output = File::create(Path::new("../../compressed.icf")).expect("Failed to create intermediate compressed file representation");
    // write just the compressed output to an intermediate compressed output file just to test
    output.write_all(&packed_bytes).expect("Failed to write compressed data");

    println!("Compressed size (in bytes): {}", packed_bytes.len());
    println!("Compression ratio: {:.2}%", (packed_bytes.len() as f64 / buffer.len() as f64) * 100.0);

    println!("\n=====STARTING SERIALIZATION=====\n");

    let compressed = CompressedFile {
        tree: root,
        data: packed_bytes,
    };

    let mut output = File::create(&output_file_path).expect("Failed to create output file");
    bincode::serialize_into(&mut output, &compressed).expect("Failed to serialize content");
    
    println!("FINAL OUTPUT SIZE: {}", get_file_size(Path::new(output_file_path)));
}

/// Walks the huffman tree and returns a lookup table `HashMap<u8, Vec<bool>>` containing the
/// encoding at each level of the tree.
fn walk_huffman_tree(
    node: &Node,
    path: &mut Vec<bool>,
    map: &mut HashMap<u8, Vec<bool>>
) {
    // NOTE: make sure to work with references and not violate ownership rules anywhere
    if let Some(val) = node.leaf {
        // reached a leaf node .. push the encoding into the map
        map.insert(val, path.clone());
    } else {
        if let Some(children) = &node.children {
            let left_child = &children.0;
            let right_child = &children.1;

            // the path is passed a reference
            // deal with the left child first
            path.push(false);
            walk_huffman_tree(&left_child, path, map);
            path.pop();
            // deal with the right child
            path.push(true);
            walk_huffman_tree(&right_child, path, map);
            path.pop();
        }
    }

}

pub fn decompress_file(input_file_path: &String, output_file_path: &String) {

    println!("\n=====STARTING DECOMPRESSION=====\n");

    let mut file = File::open(Path::new(input_file_path)).expect("Failed to open file for decompression");
    let compressed_file: CompressedFile = bincode::deserialize_from(&mut file).expect("Failed to deserialize the input file");

    let root = compressed_file.tree;
    let compressed_bytes = compressed_file.data;

    println!("Compressed file loaded from given path. Byte Count: {}", compressed_bytes.len());

    // bit-unpacking
    let mut bitstream: Vec<bool> = Vec::new();

    for byte in compressed_bytes {
        for i in (0..8).rev() { // MSB then LSB -- this is unclear
            let bit = (byte >> i) & 1;
            bitstream.push(bit == 1);
        }
    }

    // bitstream decode
    let mut output_stream = Vec::new();
    let mut current = &root;
    for bit in bitstream {
        if let Some(children) = &current.children {
            current = if bit {
                &children.1
            } else {
                &children.0
            };
        }

        if let Some(byte) = current.leaf {
            output_stream.push(byte);
            current = &root; // restart from top once you reach a leaf
        }
    }


    let mut output = File::create(Path::new(output_file_path)).expect("Failed to create decompressed file");
    output.write_all(&output_stream).expect("Failed to write decompressed data");
}

fn get_file_size(file_path: &Path) -> u64 {
    let metadata = std::fs::metadata(file_path);
    return metadata.expect("Failed to fetch file size").len();
}
