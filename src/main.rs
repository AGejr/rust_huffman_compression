use argparse::argparser;
use huffman::huffman_code;
use std::env;

mod argparse;
mod huffman;

fn main() {
    let config = argparser::parse_args(&env::args().collect());
    huffman_code::compress_file(&config.input_filename, &config.output_filename, config.verbose);
}