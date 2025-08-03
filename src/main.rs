//! crate sasen
//! in src/main.rs

#![allow(unused)]
use clap::{Parser, ValueEnum};
use sasen::compression as comp;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, value_enum, help="Compression algorithm to use")]
    a: Algorithm,
    #[arg(short, long, value_parser, help="Path to the input file")]
    i: String,
    #[arg(short, long, value_parser, help="Path to the output file")]
    o: String,
}

/// Compression Algorithm to use
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    Huffman,
    Auto,
    // Add LXW, LZ77, etc. later on
}

fn main() {
    let args = Args::parse();

    let input_path = args.i;
    let output_path = args.o;

    match args.a {
        Algorithm::Huffman | Algorithm::Auto => {
            println!("Using Huffman Compression");
            comp::huffman::compress_file(&input_path, &output_path);
            println!("\n\nFILE COMPRESSION DONE\n\nSTARTING DECOMPRESSION\n\n\n");
            comp::huffman::decompress_file(&output_path, &String::from("decompressed.hf"));
        }
    }
}
