use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    symbol_name: char,
    prob: f32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.prob.partial_cmp(&self.prob)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).upwrap()
    }
}

struct HuffmanTree {
    root: Node,
}

impl HuffmanTree {
    fn construct_tree() {
        // https://stanforddatacompressionclass.github.io/notes/lossless_iid/huffman.html

        // step 0
        let probs_dict = HashMap::from([('A', 0.35), ('B', 0.25), ('D', 0.12), ('E', 0.08)]);
        let node_list: Vec<Node> = probs_dict
            .iter()
            .map(|(char, prob)| Node {
                symbol_name: *char,
                prob: *prob,
                left: None,
                right: None,
            })
            .collect();

        // step 1
        a
    }
}
