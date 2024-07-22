use bitvec::{order::Msb0, vec::BitVec};
use ordered_float::NotNan;
use std::collections::{BinaryHeap, HashMap, VecDeque};

#[derive(Clone, PartialEq, Eq)]
struct Leaf {
    symbol_name: char,
    prob: NotNan<f32>,
}

#[derive(Clone, PartialEq, Eq)]
struct Node {
    summed_prob: NotNan<f32>,
    left: Box<HuffmanTree>,
    right: Box<HuffmanTree>,
}

#[derive(PartialEq, Eq, Clone, Ord, PartialOrd)]
enum HuffmanTree {
    Leaf(Leaf),
    Node(Node),
}

impl PartialOrd for Leaf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.prob.partial_cmp(&self.prob)
    }
}

impl Ord for Leaf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.prob.cmp(&other.prob)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.summed_prob.partial_cmp(&self.summed_prob)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.summed_prob.cmp(&other.summed_prob)
    }
}

impl HuffmanTree {
    fn get_prob(&self) -> &NotNan<f32> {
        match self {
            HuffmanTree::Leaf(Leaf { prob, .. }) => prob,
            HuffmanTree::Node(Node { summed_prob, .. }) => summed_prob,
        }
    }

    // input should NOT be empty O_O
    fn construct_tree(input: &str) -> HuffmanTree {
        // https://stanforddatacompressionclass.github.io/notes/lossless_iid/huffman.html
        let mut custom_dict: HashMap<char, f32> = HashMap::new();
        let input_length = input.chars().count() as f32;

        // step 00: iter over the input
        for c in input.chars() {
            custom_dict
                .raw_entry_mut()
                .from_key(&c)
                .and_modify(|_k, v| *v += 1.0)
                .or_insert(c, 1.0);
        }

        // divide each letter by total
        for (_, val) in custom_dict.iter_mut() {
            *val /= input_length;
        }

        let mut node_list: BinaryHeap<HuffmanTree> = BinaryHeap::new();
        for (char, prob) in custom_dict.iter() {
            node_list.push(HuffmanTree::Leaf(Leaf {
                symbol_name: *char,
                prob: NotNan::new(*prob).unwrap(),
            }))
        }

        loop {
            // step 1: pop out two nodes with smallest probability
            let lowest_prob_node_1 = node_list.pop().unwrap();
            let lowest_prob_node_2 = match node_list.pop() {
                Some(node) => node,
                None => return lowest_prob_node_1,
            };

            // step 2: join two popped nodes, and then create new node as parent of these two nodes
            let new_parent = HuffmanTree::Node(Node {
                summed_prob: lowest_prob_node_1.get_prob() + lowest_prob_node_2.get_prob(),
                left: Box::new(lowest_prob_node_1),
                right: Box::new(lowest_prob_node_2),
            });
            node_list.push(new_parent);
        }
    }

    fn find_huffman_path(&self, c: char) -> Option<BitVec<u8, Msb0>> {
        let mut node_queue = VecDeque::new();
        node_queue.push_back((self, BitVec::new()));

        match self {
            HuffmanTree::Leaf(Leaf { symbol_name, .. }) => {
                let (_, mut path) = node_queue.pop_front().unwrap();
                if *symbol_name == c {
                    path.push(false);
                    return Some(path);
                }
            }
            HuffmanTree::Node(_) => {
                while let Some((popped, mut path)) = node_queue.pop_front() {
                    match popped {
                        HuffmanTree::Leaf(Leaf { symbol_name, .. }) => {
                            if *symbol_name == c {
                                return Some(path);
                            }
                        }
                        HuffmanTree::Node(Node { left, right, .. }) => {
                            let mut path_copy = path.clone();
                            path.push(false);
                            path_copy.push(true);
                            node_queue.push_back((left, path));
                            node_queue.push_back((right, path_copy));
                        }
                    }
                }
            }
        }
        None
    }

    fn encode_input(&self, input: &str) -> BitVec<u8, Msb0> {
        input
            .chars()
            .flat_map(|c| {
                self.find_huffman_path(c)
                    .expect("char to be found")
                    .into_iter()
            })
            .collect()
    }

    fn decode_file(&self, bits: BitVec<u8, Msb0>) -> String {
        let mut curr_node = self;
        let mut decoded_string = vec![];

        for b in bits {
            match curr_node {
                HuffmanTree::Leaf(Leaf { symbol_name, .. }) => {
                    decoded_string.push(symbol_name);
                    curr_node = self;
                }

                HuffmanTree::Node(Node { left, right, .. }) => {
                    if b {
                        curr_node = right;
                    } else {
                        curr_node = left;
                    }
                    match curr_node {
                        HuffmanTree::Leaf(Leaf { symbol_name, .. }) => {
                            decoded_string.push(symbol_name);
                            curr_node = self;
                        }
                        HuffmanTree::Node(_) => {}
                    }
                }
            }
        }
        decoded_string.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::HuffmanTree;

    #[quickcheck]
    fn hufffman_compress_decompress(input: String) -> bool {
        if input.chars().count() > 0 {
            let dummy_huffmantree = HuffmanTree::construct_tree(&input);
            input == dummy_huffmantree.decode_file(dummy_huffmantree.encode_input(&input))
        } else {
            true
        }
    }

    #[test]
    fn dummy_hufffman_compress_decompress() {
        // TODO: bug when only one element
        let input = "aaaaa";
        let dummy_huffmantree = HuffmanTree::construct_tree(input);
        dbg!(dummy_huffmantree.encode_input(&input).len());
        assert_eq!(
            input,
            dummy_huffmantree.decode_file(dummy_huffmantree.encode_input(&input))
        );
    }
}
