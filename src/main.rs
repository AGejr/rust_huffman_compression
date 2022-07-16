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
        let mut frequency_table: Vec<u32> = vec![0; 255];

        // read file byte by byte and count u8 frequency
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

    fn _encode(self, input: &String) {
        for character in input.bytes() {
            let encoded_char = self.code.get(&character);
            if let Some(encoded_char) = encoded_char {
                print!("{}", encoded_char)
            }
        }
    }

    fn _decode() {}

    fn compress() {
        // TODO: implement file compression with huffman compression
    }

    fn decompress() {
        // TODO: implement file decompression with huffman compression
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

    let filename = &args[1];

    let file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let huffman_tree = HuffmanTree::build_tree(&file);

    let huffman_code = HuffmanCode::from_tree(&huffman_tree);

    print!("{}", huffman_code);

    Ok(())
}
