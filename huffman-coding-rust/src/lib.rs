
// https://doc.rust-lang.org/getopts/getopts/index.html

// From std library
use std::io::prelude::*;
use std::fs::File;

// Internal modules
pub mod codebook;
pub mod compress;
pub mod util;

pub fn read_file_to_string (filename: &str) -> String {
  let mut input_string = String::new();

  // match on Result, if I/O succeeded, access via Ok() which converts to Options
  // Results succeed (Ok) or fail (Err)
  // Options have values (Some) or do not (None)

  let mut file = match File::open(filename) {
    Ok(f) => f,
    Err(_) => { panic!("Cannot open input file") } // '_' for when a variable is returned, but not used, same as in go
  };

  // Giving a method a mutable reference (unused return value is number of bytes read)
  let _ = file.read_to_string(&mut input_string);
  input_string
}

pub fn print_summary(compression_results: Vec<compress::CompressionResult>, original_size: usize) {
  println!("Done! Threads used: {}", compression_results.len());
  let compressed_size = compression_results.iter().fold(0, |acc, ref result|  acc + result.bytes.len());
  let compression_ratio = compressed_size as f32 / original_size as f32;
  println!("Compressed bytes size {:?}, from {:?}. Ratio: {:?}", compressed_size, original_size, compression_ratio);
}