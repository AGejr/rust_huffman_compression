use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::vec;

struct HuffmanTree {
    tree_root: HuffmanTreeNode,
}

impl HuffmanTree {
    fn build_tree(file: &fs::File) -> HuffmanTree {
        let mut frequency_table: Vec<u32> = vec![0; 256];

        // read file byte by byte and count byte frequency
        for byte in file.bytes() {
            let index: usize = byte.unwrap().into();
            frequency_table[index] += 1;
        }

        // create min heap
        let mut Q = BinaryHeap::new();

        // add all entries in frequency table to the min heap
        for (index, value) in frequency_table.iter().enumerate() {
            if *value == 0 {
                continue;
            }
            let node = HuffmanTreeNode {
                left_child: None,
                right_child: None,
                frequency: *value,
                symbol: Some(index as u8),
            };
            Q.push(node);
        }

        // create huffman tree in a bottom up manner by merging entries with lowest frequency |Q| - 1 times
        for _i in 0..Q.len() - 1 {
            if let (Some(left_node), Some(right_node)) = (Q.pop(), Q.pop()) {
                let parent_frequency = left_node.frequency + right_node.frequency;
                let mut node = HuffmanTreeNode {
                    left_child: Some(Box::new(left_node)),
                    right_child: Some(Box::new(right_node)),
                    frequency: 0,
                    symbol: None,
                };
                node.frequency = parent_frequency;
                Q.push(node)
            }
        }

        // extract root of min heap
        if let Some(root_node) = Q.pop() {
            HuffmanTree {
                tree_root: root_node,
            }
        } else {
            panic!("Could not build Huffman Tree")
        }
    }
}

#[derive(Debug)]
struct HuffmanCode {
    code: HashMap<u8, String>,
}

impl std::fmt::Display for HuffmanCode {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut symbols = Vec::from_iter(self.code.iter());
        symbols.sort();
        println!("Symbol\t| Code Word");
        println!("------------------");
        for (symbol, code_word) in symbols {
            println!("{symbol}\t| {code_word}");
        }
        Ok(())
    }
}

impl HuffmanCode {
    fn from_tree(huffman_tree: &HuffmanTree) -> HuffmanCode {
        let mut code: HashMap<u8, String> = HashMap::new();

        let mut node_fringe = vec![(&huffman_tree.tree_root, String::new())];

        // The Huffman tree is traversed using left to right depth first search
        // Entries in fringe matches the tuple (node, code_word)
        while node_fringe.len() != 0 {
            if let Some(fringe_entry) = node_fringe.pop() {
                let (node, mut code_word) = fringe_entry;

                // If current node has children, push them to the fringe
                if let (Some(left_child), Some(right_child)) =
                    (node.left_child.as_ref(), node.right_child.as_ref())
                {
                    code_word.push('1');
                    node_fringe.push((right_child, code_word.clone()));
                    code_word.pop();
                    code_word.push('0');
                    node_fringe.push((left_child, code_word));
                }
                // If current node has no children, then add a code word to the code
                else if let (None, None) = (node.left_child.as_ref(), node.right_child.as_ref()) {
                    if let Some(symbol) = node.symbol {
                        code.insert(symbol, code_word);
                    }
                }
            }
        }
        HuffmanCode { code: code }
    }

    fn encode(input: &str) -> u8 {
        let mut output: u8 = 0;
        for character in input.chars() {
            output = match character {
                '0' => output << 1,
                '1' => (output << 1) | 1,
                _default => panic!(),
            };
        }
        print!("");
        output
    }

    fn _decode() {}

    fn compress(self, output_file: &mut File, input_filename: &String) -> io::Result<()> {
        let input_file = match fs::File::open(input_filename) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        // contents
        let mut output_buffer: String = String::from("");
        for byte in input_file.bytes() {
            let symbol: u8 = byte.unwrap();
            let code_word = self.code.get(&symbol).unwrap();
            output_buffer.push_str(code_word);

            while output_buffer.len() >= 8 {
                let output_symbol = HuffmanCode::encode(&output_buffer[0..8]);
                output_file.write(&[output_symbol]);
                output_buffer = output_buffer[8..].to_string();
            }
        }

        // remaining bits
        if output_buffer.len() > 0 {
            // pad the remaining bits
            for _i in 0..(8 - output_buffer.len()) {
                output_buffer.push('0');
            }
            let output_symbol = HuffmanCode::encode(&output_buffer[0..8]);
            output_file.write(&[output_symbol]);
        }

        Ok(())
    }

    fn decompress() {
        // TODO: implement file decompression
    }
}

#[derive(Debug, Eq)]
struct HuffmanTreeNode {
    left_child: Option<Box<HuffmanTreeNode>>,
    right_child: Option<Box<HuffmanTreeNode>>,
    frequency: u32,
    symbol: Option<u8>,
}

impl Ord for HuffmanTreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency).reverse()
    }
}

impl PartialOrd for HuffmanTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffmanTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let now = std::time::Instant::now();

    let filename = &args[1];

    let input_file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let input_file_size: f64 = input_file.metadata().unwrap().len() as f64;
    println!("Input file size = {} bytes", input_file_size);

    let huffman_tree = HuffmanTree::build_tree(&input_file);

    let huffman_code = HuffmanCode::from_tree(&huffman_tree);

    let compressed_file_extension = ".hfm";
    let output_filename = format!("{}{}", filename, compressed_file_extension);

    let mut output_file: File = match File::create(&output_filename) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    huffman_code.compress(&mut output_file, &filename);

    let output_file_size: f64 = output_file.metadata().unwrap().len() as f64;
    println!("Output file size = {} bytes", output_file_size);

    let file_size_reduction = ((1.0 - output_file_size / input_file_size) * 100.0);
    println!("File size reduction = {}%", file_size_reduction);

    let elapsed = now.elapsed().as_millis();
    println!("Compression took {} millis", elapsed);

    Ok(())
}