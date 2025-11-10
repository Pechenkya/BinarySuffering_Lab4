#![allow(dead_code, non_snake_case, unused_imports, unused_variables)]
mod LZWCoder;

use std::time::Instant;

fn encode_file_with_timer(input_path: &'static str, output_path: &'static str) {
    println!("Encoding file: {}", input_path);
    let start = Instant::now();
    let encoding_handle = std::thread::spawn(move || {
        LZWCoder::encode_file(input_path, output_path, true);
        start.elapsed()
    });
    
    // Progress time
    while !encoding_handle.is_finished() {
        print!("\rEncoding time: {:?}", start.elapsed());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    let encode_duration = encoding_handle.join().unwrap();
    println!("\rEncoding time: {:?}", encode_duration);
}

fn decode_file_with_timer(input_path: &'static str, output_path: &'static str) {
    println!("Decoding file: {}", input_path);
    let start = Instant::now();
    let decoding_handle = std::thread::spawn(move || {
        LZWCoder::decode_file(input_path, output_path);
        start.elapsed()
    });
    
    // Progress time
    while !decoding_handle.is_finished() {
        print!("\rDecoding time: {:?}", start.elapsed());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    let decode_duration = decoding_handle.join().unwrap();
    println!("\rDecoding time: {:?}", decode_duration);
}

fn main() {
    encode_file_with_timer("test_data/test_file.txt", "test_data/test_file.txt.lzw");
    encode_file_with_timer("test_data/test_pdf.pdf", "test_data/test_pdf.pdf.lzw");
    encode_file_with_timer("test_data/test_pdf_2.pdf", "test_data/test_pdf_2.pdf.lzw");

    decode_file_with_timer("test_data/test_file.txt.lzw", "test_data/decoded_file.txt");
    decode_file_with_timer("test_data/test_pdf.pdf.lzw", "test_data/decoded_pdf.pdf");
    decode_file_with_timer("test_data/test_pdf_2.pdf.lzw", "test_data/decoded_pdf_2.pdf");
}
