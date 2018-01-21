extern crate lzw;
extern crate huffman_coding;
// External crates, via Cargo.toml
extern crate num_cpus;
extern crate getopts;

use std::io::Read;
use getopts::Options;
use std::fs::File;
use huffman_coding::codebook::Codebook;
use huffman_coding::util::string_to_substrings;
use lzw::{LsbWriter, Encoder};
fn main(){
    /*TO-DO LIST
        * Open three types of file: .mp4, .mp3, .txt
        * Convert to binary
        * encode - decode
    */
    let file_name = "textfile.txt";
    let mut file = match File::open(file_name){
        Ok(file) => file,
        Err(e) => panic!("{:?}", e)
    };
    let mut my_string_buffer = Vec::<u8>::new(); 
    let mut my_string_content = String::new();
    let _ = file.read_to_end(&mut my_string_buffer);
    let _ = file.read_to_string(&mut my_string_content);

    //lzw encoding
    let dict_size = 8;
    let mut compressed : Vec<u8> = vec!();
    {
        let mut encoder = Encoder::new(LsbWriter::new(&mut compressed), dict_size).unwrap();
        encoder.encode_bytes(&my_string_buffer[..]).unwrap();

    }
    println!("{}", my_string_buffer.len());
    println!("{}", compressed.len() );

    //let compressed_size = compression_results.iter().fold(0, |acc, ref result|  acc + result.bytes.len());
    let compression_ratio = my_string_buffer.len() as f32 / compressed.len() as f32;
    println!("LZW Compressed bytes size {:?}, from {:?}. Ratio: {:?}",  compressed.len(), my_string_buffer.len(), compression_ratio);

    //huffman encoding
    let substrings = string_to_substrings(&my_string_content, 1);
    let codebook = Codebook::new(&substrings);
}
