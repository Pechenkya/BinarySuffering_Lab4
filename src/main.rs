#![allow(dead_code, non_snake_case, unused_imports, unused_variables)]
mod LZWCoder;

use std::str::FromStr;
use std::io::{self, Write};
use std::time::{Instant, Duration};

fn print_usage_message() {
    println!("-------- LZW Encoder/Decoder --------");
    println!("Commands:");
    println!("1. Compress the file with LZW encoding:");
    println!("\tencode <input_file> [-c] [output_file]");
    println!("   If output_file is not provided, it defaults to <input_file>.lzw.");
    println!("   Use -c flag to use reitialization of dictionary in overfill.");
    println!("2. Decompress the file compressed with LZW encoding:");
    println!("\tdecode <input_file> [output_file]");
    println!("   If output_file is not provided, it defaults to: <input_file> with .lzw removed or .decoded added");
    println!("3. Exit the program:");
    println!("\texit");
    println!("-------------------------------------");
}

fn encode_file_with_timer(input_path: String, output_path: String, reinit_dict: bool) {
    println!("Encoding file: {}", input_path);
    let start = Instant::now();
    let encoding_handle = std::thread::spawn(move || {
        LZWCoder::encode_file(&input_path, &output_path, reinit_dict);
        start.elapsed()
    });
    
    // Progress time
    while !encoding_handle.is_finished() {
        print!("\rEncoding time: {:?}", start.elapsed());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(Duration::from_millis(100));
    }
    
    let encode_duration = encoding_handle.join().unwrap();
    println!("\rEncoding time: {:?}", encode_duration);
}

fn decode_file_with_timer(input_path: String, output_path: String) {
    println!("Decoding file: {}", input_path);
    let start = Instant::now();
    let decoding_handle = std::thread::spawn(move || {
        LZWCoder::decode_file(&input_path, &output_path);
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
    // encode_file_with_timer("test_data/test_file.txt", "test_data/test_file.txt.lzw", true);
    // encode_file_with_timer("test_data/test_pdf.pdf", "test_data/test_pdf.pdf.lzw", true);
    // encode_file_with_timer("test_data/test_pdf_2.pdf", "test_data/test_pdf_2.pdf.lzw", true);

    // decode_file_with_timer("test_data/test_file.txt.lzw", "test_data/decoded_file.txt");
    // decode_file_with_timer("test_data/test_pdf.pdf.lzw", "test_data/decoded_pdf.pdf");
    // decode_file_with_timer("test_data/test_pdf_2.pdf.lzw", "test_data/decoded_pdf_2.pdf");


    print_usage_message();

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");

        let mut parts: Vec<&str> = command.trim().split_whitespace().collect();

        // Check for reinitialization flag
        let reitialize_dict = parts.contains(&"-c");
        parts.retain(|&x| x != "-c");

        match parts.as_slice() {
            ["encode", input_file] => {
                encode_file_with_timer(String::from_str(input_file).unwrap(), format!("{input_file}.lzw"), reitialize_dict);
            }
            ["encode", input_file, output_file] => {
                encode_file_with_timer(String::from_str(input_file).unwrap(), String::from_str(output_file).unwrap(), reitialize_dict);
            }
            ["decode", input_file] => {
                let output_file = if input_file.ends_with(".lzw") {
                    input_file.trim_end_matches(".lzw").to_string()
                } else {
                    format!("{}.decoded", input_file)
                };

                decode_file_with_timer(String::from_str(input_file).unwrap(), output_file);
            }
            ["decode", input_file, output_file] => {
                decode_file_with_timer(String::from_str(input_file).unwrap(), String::from_str(output_file).unwrap());
            }
            ["exit"] => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid input.");
            }
        }
    }
}
