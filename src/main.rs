use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::Read;

#[derive(Debug, Eq)]
struct Node {
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
    frequency: u32,
    key: Option<u8>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

fn node_ptr(node: Option<Node>) -> Option<Box<Node>> {
    match node {
        None => None,
        Some(n) => Some(Box::new(n)),
    }
}

// Generates Huffman Tree and returns the root node
fn generate_huffman_tree(frequency_table: &mut Vec<u32>) -> Option<Node> {
    let mut Q = BinaryHeap::new();

    for (index, value) in frequency_table.iter().enumerate() {
        if *value == 0 {
            continue;
        }
        let node = Node {
            left_child: None,
            right_child: None,
            frequency: *value,
            key: Some(index as u8),
        };
        Q.push(node);
    }

    for _i in 0..Q.len() - 1 {
        let mut node = Node {
            left_child: node_ptr(Q.pop()),
            right_child: node_ptr(Q.pop()),
            frequency: 0,
            key: None,
        };
        if let (Some(left_child), Some(right_child)) =
            (node.left_child.as_ref(), node.right_child.as_ref())
        {
            node.frequency = left_child.frequency + right_child.frequency;
        }
        Q.push(node);
    }
    Q.pop()
}

fn generate_encoding_scheme(huffman_tree_root: &Node) -> HashMap<u8, String> {
    let mut encoding_scheme: HashMap<u8, String> = HashMap::new();

    let mut node_fringe = vec![(huffman_tree_root, String::new())];

    // The Huffman tree is traversed using breadth first search
    // Entries in fringe matches the tuple (node, encoding)
    while node_fringe.len() != 0 {
        if let Some(fringe_entry) = node_fringe.pop() {
            let (node, mut encoding) = fringe_entry;
            // If current node has children, push them to the fringe
            if let (Some(left_child), Some(right_child)) =
                (node.left_child.as_ref(), node.right_child.as_ref())
            {
                encoding.push('1');
                node_fringe.push((right_child, encoding.clone()));
                encoding.pop();
                encoding.push('0');
                node_fringe.push((left_child, encoding));
            }
            // If current node has no children, then add an encoding entry
            else if let (None, None) = (node.left_child.as_ref(), node.right_child.as_ref()) {
                if let Some(key) = node.key {
                    encoding_scheme.insert(key, encoding);
                }
            }
        }
    }
    encoding_scheme
}

// Used for debugging purposes
fn _encode_string(encoding_scheme: HashMap<u8, String>, input: String) {
    for character in input.bytes() {
        let encoded_char = encoding_scheme.get(&character);
        if let Some(echar) = encoded_char {
            print!("{}", echar)
        }
    }
}

// Used for debugging purposes
fn _print_frequency_table(frequency_table: &Vec<u32>) {
    for i in 0..frequency_table.len() {
        print!("{} {}\n", i, frequency_table[i]);
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = fs::File::open(filename)?;

    let mut frequency_table: Vec<u32> = vec![0; 255];

    // read file byte by byte and count u8 frequency
    for byte in file.bytes() {
        let index: usize = byte.unwrap().into();
        frequency_table[index] += 1;
    }

    // Debug input
    /*
    let mut testfreq = vec![0;255];
    testfreq['f' as usize] = 5;
    testfreq['e' as usize] = 9;
    testfreq['c' as usize] = 12;
    testfreq['b' as usize] = 13;
    testfreq['d' as usize] = 16;
    testfreq['a' as usize] = 45;
    let huffman_tree_root = generate_huffman_tree(&mut testfreq);
    */

    let huffman_tree_root = generate_huffman_tree(&mut frequency_table);

    let mut encoding_scheme = HashMap::new();

    if let Some(huffman_tree_root) = huffman_tree_root {
        encoding_scheme = generate_encoding_scheme(&huffman_tree_root);
        println!("{:?}", encoding_scheme);
    }

    // Used to test debug input
    //_encode_string(encoding_scheme, String::from("lorem ipsum"));

    // TODO: read file and write encoded contents to new file

    // TODO: add compression percentage to console output

    Ok(())
}
