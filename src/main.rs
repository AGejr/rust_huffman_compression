use argparse::argparser;
use huffman::huffman_code;
use std::env;
use std::io;
use std::io::BufRead;

mod argparse;
mod huffman;

fn sanitize_input(input: &String) {
    for character in input.chars() {
        if !character.is_alphanumeric() {
            panic!("Only alphanumeric characters is acceptable input")
        }
    }
}

fn run_interactive_mode(){
    println!("Running in interactive mode");
    println!("Type 'q' to quit\n");
    loop {
        println!("Enter the Huffman Code characters:");
        
        let stdin = io::stdin();
        let characters = stdin.lock().lines().next().unwrap().unwrap();
        
        match characters.len() {
            0 => continue,
            1 => if characters.as_str() == "q" {return},
            _ => sanitize_input(&characters),
        }
        
        println!("Enter a string of characters to encode (leave blank if you only want the Huffman code):");
        let string_to_encode = stdin.lock().lines().next().unwrap().unwrap();

        match string_to_encode.len() {
            0 => (),
            1 => if string_to_encode.as_str() == "q" {return},
            _ => sanitize_input(&string_to_encode),
        }

        huffman_code::compress_string(&characters, &string_to_encode);
    }
}

fn main() {
    let config = argparser::parse_args(&env::args().collect());

    if config.interactive {
        run_interactive_mode();
    } else {
        huffman_code::compress_file(&config.input_filename, &config.output_filename, config.binary, config.verbose);
    }
}