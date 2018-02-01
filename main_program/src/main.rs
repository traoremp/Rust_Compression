extern crate lzw;
extern crate huffman_coding;

// External crates, via Cargo.toml
extern crate num_cpus;
// extern crate getopts;
extern crate image; 
extern crate cpuprofiler;

use std::io::Read;
// use getopts::Options;
use std::fs::File;
use huffman_coding::codebook::Codebook;
use huffman_coding::util::string_to_substrings;
use huffman_coding::compress::parallel_compress;
use huffman_coding::print_summary;
use lzw::{LsbWriter, Encoder};
use cpuprofiler::PROFILER;
// use image::DynamicImage;

fn encode_lzw( compressed : &mut Vec<u8>, dict_size : u8, data_to_encode : &[u8]){
    
    let mut encoder = Encoder::new(LsbWriter::new(compressed), dict_size).unwrap();
    encoder.encode_bytes(data_to_encode).unwrap();
}

fn main(){
    /*TO-DO LIST
        * Open three types of file: .mp4, .mp3, .txt
        * Convert to binary
        * encode - decode
    */
    let file_name = "Addison.Wesley.Effective.CPP.3rd.Edition.May.2005.pdf";
    let mut file = match File::open(file_name){

        Ok(file) => file,
        Err(e) => panic!("{:?}", e)
    };
    let mut my_string_buffer = Vec::<u8>::new(); 
    let _ = file.read_to_end(&mut my_string_buffer);

    //lzw encoding
    // let dict_size = 8;
    let mut compressed : Vec<u8> = vec!();
    {
        encode_lzw(&mut compressed, 15, &my_string_buffer[..] );
    }
    // println!("{}", my_string_buffer.len());
    // println!("{}", compressed.len() );

    //let compressed_size = compression_results.iter().fold(0, |acc, ref result|  acc + result.bytes.len());
    let compression_rate = compressed.len() as f32 / my_string_buffer.len() as f32 ;
    let compression_ratio = my_string_buffer.len() as f32 / compressed.len() as f32;
    println!("\nLZW Compressed bytes size {:?}, from {:?}. Ratio: {:?}, Rate {:?}",  compressed.len(), my_string_buffer.len(), compression_ratio, compression_rate);

    //huffman encoding
    //let my_string_content = String::from_utf8(my_string_buffer).unwrap();
    let substrings = string_to_substrings(&my_string_buffer[..], 1);
    //println!("{:?}", my_string_content);
    //println!("{:?}", substrings);
    let codebook = Codebook::new(&substrings);
    //println!("{:?}", codebook.character_map);
    let compression_results = parallel_compress(&substrings, &codebook);
    // print_summary(compression_results, my_string_content.len());
    let compressed_size_huffman = compression_results.iter().fold(0, |acc, ref result|  acc + result.bytes.len());
    let compression_rate_huffman = compressed_size_huffman as f32 / my_string_buffer.len() as f32;
    let compression_ratio_huffman =   my_string_buffer.len() as f32/  compressed_size_huffman as f32;
    println!("\nHuffman Compressed bytes size {:?}, from {:?}. Ratio: {:?}, Rate {:?}", compressed_size_huffman, my_string_buffer.len(), compression_ratio_huffman, compression_rate_huffman);

    //IMAGE 
    let my_image = image::open("huffman_david.99-10-11.bmp").unwrap();
    let my_rgb_image = my_image.to_rgb();
    let my_image_as_vec = my_rgb_image.into_raw();
    let mut compressed : Vec<u8> = vec!();
    // LZW image encoding
    {
        PROFILER.lock().unwrap().start("./my-prof.profile").expect("Couldn't start");
        encode_lzw(&mut compressed, 8, &my_image_as_vec[..] ); 
        PROFILER.lock().unwrap().stop().expect("Couldn't stop");        
    }
    
    let compression_rate = compressed.len() as f32 / my_image_as_vec.len() as f32 ;
    let compression_ratio = my_image_as_vec.len() as f32 / compressed.len() as f32;
    println!("\nIMAGE LZW Compressed bytes size {:?}, from {:?}. Ratio: {:?}, Rate {:?}",  compressed.len(), my_image_as_vec.len(), compression_ratio, compression_rate);

    // Huffman image encoding

    // //huffman encoding
    //let my_string_content = String::from_utf8(my_image_as_vec).unwrap();
    let substrings = string_to_substrings(&my_image_as_vec[..], 1);
    //println!("{:?}", my_string_content);
    //println!("{:?}", substrings);
    let codebook = Codebook::new(&substrings);
    //println!("{:?}", codebook.character_map);
    let compression_results = parallel_compress(&substrings, &codebook);
    //print_summary(compression_results, my_string_content.len());
     let compressed_size_huffman = compression_results.iter().fold(0, |acc, ref result|  acc + result.bytes.len());
    let compression_rate_huffman = compressed_size_huffman as f32 / my_image_as_vec.len() as f32;
    let compression_ratio_huffman =   my_image_as_vec.len() as f32/  compressed_size_huffman as f32;
    println!("\nHuffman Compressed bytes size {:?}, from {:?}. Ratio: {:?}, Rate {:?}\n", compressed_size_huffman, my_image_as_vec.len(), compression_ratio_huffman, compression_rate_huffman);
}
