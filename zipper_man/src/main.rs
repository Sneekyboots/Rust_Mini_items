use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq)]
struct Node {
    freq: usize,
    ch: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// make min-heap by reversing the order
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.freq.cmp(&self.freq)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// build Huffman tree
fn build_tree(freqs: &HashMap<char, usize>) -> Node {
    let mut heap = BinaryHeap::new();
    for (&ch, &f) in freqs {
        heap.push(Node { freq: f, ch: Some(ch), left: None, right: None });
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        heap.push(Node {
            freq: left.freq + right.freq,
            ch: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        });
    }
    heap.pop().unwrap()
}

// make codes
fn make_codes(node: &Node, prefix: String, codes: &mut HashMap<char, String>) {
    if let Some(ch) = node.ch {
        codes.insert(ch, prefix);
    } else {
        if let Some(ref l) = node.left {
            make_codes(l, format!("{}0", prefix), codes);
        }
        if let Some(ref r) = node.right {
            make_codes(r, format!("{}1", prefix), codes);
        }
    }
}

fn main() {
    let text = "hello huffman";

    // count frequencies
    let mut freqs = HashMap::new();
    for ch in text.chars() {
        *freqs.entry(ch).or_insert(0) += 1;
    }

    // tree + codes
    let tree = build_tree(&freqs);
    let mut codes = HashMap::new();
    make_codes(&tree, "".into(), &mut codes);

    println!("Codes: {:?}", codes);

    // compress
    let encoded: String = text.chars().map(|c| codes[&c].clone()).collect();
    println!("Compressed: {}", encoded);

    // decompress
    let mut result = String::new();
    let mut node = &tree;
    for bit in encoded.chars() {
        node = if bit == '0' { node.left.as_ref().unwrap() }
               else { node.right.as_ref().unwrap() };

        if let Some(ch) = node.ch {
            result.push(ch);
            node = &tree;
        }
    }
    println!("Decompressed: {}", result);
}
