pub mod huffman_code {
    use core::panic;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::io::Write;
    use std::vec;

    mod huffman_tree {
        use std::cmp::Ordering;
        use std::collections::{BinaryHeap, HashMap};
        use std::fs::File;
        use std::io::Read;
        use std::ops::Deref;

        #[derive(Debug, Eq)]
        pub(crate) struct HuffmanTreeNode {
            pub(crate) left_child: Option<Box<HuffmanTreeNode>>,
            pub(crate) right_child: Option<Box<HuffmanTreeNode>>,
            pub(crate) frequency: u32,
            pub(crate) symbol: Option<u8>,
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

        pub(crate) fn frequency_table_from_file(file: &File) -> Vec<u32> {
            let mut frequency_table: Vec<u32> = vec![0; 256];

            if file.metadata().unwrap().len() == 0 {
                panic!("Input file must not be empty");
            }

            // read file byte by byte and count byte frequency
            for byte in file.bytes() {
                let index: usize = byte.unwrap().into();
                frequency_table[index] += 1;
            }

            frequency_table
        }

        pub(crate) fn frequency_table_from_text_file(file: &mut File) -> Vec<u32> {
            let mut frequency_table: Vec<u32> = vec![0; 256];

            if file.metadata().unwrap().len() == 0 {
                panic!("Input file must not be empty");
            }

            let mut file_contents = String::new();
            file.read_to_string(&mut file_contents).unwrap();

            for character in file_contents.chars() {
                let index: usize = character as u8 as usize;
                frequency_table[index] += 1;
            }

            frequency_table
        }

        pub(crate) fn frequency_table_from_string(string: &String) -> Vec<u32> {
            let mut frequency_table: Vec<u32> = vec![0; 256];

            for character in string.chars() {
                let index: usize = character as usize;
                frequency_table[index] += 1;
            }

            frequency_table
        }

        fn build_min_heap(frequency_table: &Vec<u32>) -> BinaryHeap<HuffmanTreeNode> {
            // create min heap
            let mut Q = BinaryHeap::new();

            // add all entries in frequency table to the min heap
            for (index, value) in frequency_table.iter().enumerate() {
                if *value == 0 {
                    continue;
                }
                let node = self::HuffmanTreeNode {
                    left_child: None,
                    right_child: None,
                    frequency: *value,
                    symbol: Some(index as u8),
                };
                Q.push(node);
            }
            Q
        }

        pub(crate) fn print_frequency_table(frequency_table: &Vec<u32>, as_char: bool, verbose: bool) {
            println!("Symbol\t| Frequency");
            println!("-------------------");
            for (index, element) in frequency_table.into_iter().enumerate() {
                if !verbose && *element == 0 {
                    continue;
                }
                if as_char && (index as u8 as char).is_alphanumeric() {
                    println!("{}\t| {}", index as u8 as char, element);
                } else {
                    println!("{}\t| {}", index, element);
                }
            }
        }

        pub(crate) fn build_huffman_tree(frequency_table: &Vec<u32>) -> self::HuffmanTreeNode {
            let mut Q = self::build_min_heap(&frequency_table);

            // create huffman tree in a bottom up manner by merging entries with lowest frequency |Q| - 1 times
            for _i in 0..Q.len() - 1 {
                if let (Some(left_node), Some(right_node)) = (Q.pop(), Q.pop()) {
                    let parent_frequency = left_node.frequency + right_node.frequency;
                    let mut node = self::HuffmanTreeNode {
                        left_child: Some(Box::new(left_node)),
                        right_child: Some(Box::new(right_node)),
                        frequency: 0,
                        symbol: None,
                    };
                    node.frequency = parent_frequency;
                    Q.push(node)
                }
            }

            Q.pop().unwrap()
        }
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
        output
    }

    fn print_huffman_code(huffman_code: &HashMap<u8, String>, as_char: bool) {
        let mut symbols = Vec::from_iter(huffman_code.iter());
        symbols.sort();
        println!("Symbol\t| Code Word");
        println!("-------------------");
        for (symbol, code_word) in symbols {
            if as_char && (*symbol as u8 as char).is_alphanumeric() {
                println!("{}\t| {}", *symbol as char, code_word);
            } else {
                println!("{}\t| {}", symbol, code_word);
            }
        }
    }

    fn generate_huffman_code(
        huffman_tree_root: &self::huffman_tree::HuffmanTreeNode,
    ) -> HashMap<u8, String> {
        let mut code: HashMap<u8, String> = HashMap::new();

        let mut node_fringe = vec![(huffman_tree_root, String::new())];

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
        code
    }

    pub fn compress_string(characters: &String, string_to_encode: &String) {
        let frequency_table = self::huffman_tree::frequency_table_from_string(&characters);
        println!("");
        self::huffman_tree::print_frequency_table(&frequency_table, true, false);
        let huffman_tree = self::huffman_tree::build_huffman_tree(&frequency_table);
        let huffman_code = self::generate_huffman_code(&huffman_tree);
        println!("");
        self::print_huffman_code(&huffman_code, true);
        println!("");

        // contents
        let mut output_buffer: String = String::from("");
        for character in string_to_encode.chars() {
            let code_word = match huffman_code.get(&(character as u8)) {
                Some(code_word) => code_word,
                None => panic!("Cannot encode a char which is not represented in the code"),
            };
            output_buffer.push_str(code_word);
        }

        if !string_to_encode.len() == 0 {
            println!("Input string:\t{}", string_to_encode);
            println!("Encoded string:\t{}\n", output_buffer);
        }
    }

    pub fn compress_file(input_filename: &String, output_filename: &String, binary: bool, verbose: bool) {
        let now = std::time::Instant::now();

        let mut input_file = File::open(input_filename).unwrap();
        let mut output_file = File::create(output_filename).unwrap();

        let input_file_size: f64 = input_file.metadata().unwrap().len() as f64;
        println!("Input file size = {} bytes", input_file_size);

        let mut frequency_table = match binary {
            true => self::huffman_tree::frequency_table_from_file(&input_file),
            false => self::huffman_tree::frequency_table_from_text_file(&mut input_file),
        };
        let huffman_tree = self::huffman_tree::build_huffman_tree(&frequency_table);
        let huffman_code = self::generate_huffman_code(&huffman_tree);

        if verbose {
            println!("");
            self::huffman_tree::print_frequency_table(&frequency_table, !binary, false);
            println!("");
            self::print_huffman_code(&huffman_code, !binary);
            println!("");
        }

        input_file = File::open(input_filename).unwrap();

        let mut output_buffer: String = String::from("");
        // TODO: improve this lol
        if !binary {
            let mut file_contents = String::new();
            input_file.read_to_string(&mut file_contents).unwrap();
            for character in file_contents.chars() {
                let symbol: u8 = character as u8;
                let code_word = huffman_code.get(&symbol).unwrap();
                output_buffer.push_str(code_word);
    
                while output_buffer.len() >= 8 {
                    let output_symbol = self::encode(&output_buffer[0..8]);
                    output_file.write(&[output_symbol]).unwrap();
                    output_buffer = output_buffer[8..].to_string();
                }
            }
        } else {
            for byte in input_file.bytes() {
                let symbol: u8 = byte.unwrap() as u8;
                let code_word = huffman_code.get(&symbol).unwrap();
                output_buffer.push_str(code_word);

                while output_buffer.len() >= 8 {
                    let output_symbol = self::encode(&output_buffer[0..8]);
                    output_file.write(&[output_symbol]).unwrap();
                    output_buffer = output_buffer[8..].to_string();
                }
            }
        }
        
        // remaining bits
        if output_buffer.len() > 0 {
            // pad the remaining bits
            for _i in 0..(8 - output_buffer.len()) {
                output_buffer.push('0');
            }
            let output_symbol = self::encode(&output_buffer[0..8]);
            output_file.write(&[output_symbol]).unwrap();
        }

        let output_file_size: f64 = output_file.metadata().unwrap().len() as f64;
        println!("Output file size = {} bytes", output_file_size);

        let file_size_reduction = (1.0 - output_file_size / input_file_size) * 100.0;
        println!("File size reduction = {}%", file_size_reduction);

        let elapsed = now.elapsed().as_millis();
        println!("Compression took {} millis", elapsed);
    }
}
