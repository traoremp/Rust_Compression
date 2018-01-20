extern crate lzw;
extern crate huffman_coding;
// External crates, via Cargo.toml
extern crate num_cpus;
extern crate getopts;

use std::io::Read;
use getopts::Options;
use std::fs::File;

use lzw::{LsbWriter, Encoder};

fn main(){
    println!("Hello World");
    /*TO-DO LIST
        * Open three types of file: .mp4, .mp3, .txt
        * Convert to binary
        * encode - decode
    */
    let fileName = "textfile.txt";
    let mut file = match File::open(fileName){
        Ok(file) => file,
        Err(e) => panic!("{:?}", e)
    };
    let mut my_string_buffer = Vec::<u8>::new(); 
    file.read_to_end(&mut my_string_buffer);

    //lzw encoding
    let dict_size = 9;
    let mut compressed = vec!();
    {
        let mut encoder = Encoder::new(LsbWriter::new(&mut compressed), dict_size).unwrap();
        encoder.encode_bytes(&my_string_buffer[..]).unwrap();

    }
    println!("{}", my_string_buffer.len());
    println!("{}", compressed.len() );

    //let compressed_size = compression_results.iter().fold(0, |acc, ref result|  acc + result.bytes.len());
    let compression_ratio = my_string_buffer.len() as f32 / compressed.len() as f32;
    println!("Compressed bytes size {:?}, from {:?}. Ratio: {:?}",  compressed.len(), my_string_buffer.len(), compression_ratio);

}
