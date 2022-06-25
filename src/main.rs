use std::fs;
use std::io;
use std::io::Read;
use std::env;

fn main() -> io::Result<()> {
    // take filename as argument and open file
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = fs::File::open(filename)?;

    let mut frequency_table: [u32; 256] = [0;256];
    
    // read file byte by byte and count u8 frequency
    for byte in file.bytes() {
        let index: usize = byte.unwrap().into();
        frequency_table[index] += 1;
    }
    
    // print frequency table
    for i in 0..frequency_table.len() {
        if frequency_table[i] != 0 {
            print!("{} {}\n",i,frequency_table[i]);
        }
    }
    
    Ok(())
}