use std::cmp::Ordering;
use std::collections::BinaryHeap;
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
            frequency: *value,
            key: Some(index as u8),
            left_child: None,
            right_child: None,
        };
        Q.push(node);
    }

    for i in 0..Q.len() - 1 {
        let mut node = Node {
            left_child: node_ptr(Q.pop()),
            right_child: node_ptr(Q.pop()),
            frequency: 0,
            key: None,
        };
        if let (Some(left_child), Some(right_child)) = (&node.left_child, &node.right_child) {
            node.frequency = left_child.frequency + right_child.frequency;
        }
        Q.push(node);
    }
    print!("{:?}", Q);
    Q.pop()
}

// Used for debugging purposes
fn print_frequency_table(frequency_table: &Vec<u32>) {
    for i in 0..frequency_table.len() {
        print!("{} {}\n", i, frequency_table[i]);
    }
}

fn main() -> io::Result<()> {
    // take filename as argument and open file
    let args: Vec<String> = env::args().collect();
    //let filename = &args[1];
    let file = fs::File::open("example.txt")?;

    let mut frequency_table: Vec<u32> = vec![0; 256];

    // read file byte by byte and count u8 frequency
    for byte in file.bytes() {
        let index: usize = byte.unwrap().into();
        frequency_table[index] += 1;
    }

    let mut testfreq = vec![5, 9, 12, 13, 16, 45];

    generate_huffman_tree(&mut testfreq);

    Ok(())
}