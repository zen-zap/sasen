#![allow(unused)]
use crate::Node;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::collections::{BinaryHeap, HashMap};

pub fn compress_file(input_file_path: &String, output_file_path: &String) {

    let in_path = PathBuf::from(input_file_path);
    let mut file = File::open(in_path).expect("Unable to open the file");
    let mut buffer = Vec::new();
    let bytes_read = file.read_to_end(&mut buffer).expect("Failed to read file");
    println!("Bytes read: {}", bytes_read);

    // for byte in buffer.iter() {
    //     print!("{} ", byte);
    // }

    println!("=====Moving on to Frequency Counting=====");
    // A byte is 8 bits, so 256 different values
    let mut counts = [0u32; 256];
    for byte in &buffer {
        counts[*byte as usize] += 1;
    }


    // for (byte, &count) in counts.iter().enumerate() {
    //     if count > 0 {
    //         // If it's a printable ASCII character (like a-z, /, etc)
    //         if byte == (b' ' as usize)  || char::from(byte as u8).is_ascii_graphic() {
    //             println!("Byte: {:>3} ('{}') → Count: {}", byte, byte as u8 as char, count);
    //         } else {
    //             println!("Byte: {:>3}      → Count: {}", byte, count);
    //         }
    //     }
    // }

    println!("=====Making the Huffman Tree=====");

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
    let encoding = walk_huffman_tree(&root);
}


// Hashmap<u8, Vec<bool>> is the lookup table for the encoder
fn walk_huffman_tree(
    node: &Node,
    path: &mut Vec<bool>,
    map: &mut HashMap<u8, Vec<bool>>
) {

}

pub fn decompress_file(input_file_path: &String, output_file_path: &String) {
    unimplemented!("DeCompression algorithm is not yet complete!")
}
