# rust-huffman
A pure Rust library for Huffman coding https://en.wikipedia.org/wiki/Huffman_coding

It takes a unordered list of items and their occurrences, and finds the optimal Huffman coding in O(n log(n)) time.

Sample code:
```rust

extern crate huffman;

use huffman::{HuffmanNode, HuffmanLeaf, HuffmanBranch};

// unordered vector of the items (chars in this case) and the number of times the character occurs:
let v = vec!(
    HuffmanLeaf::new(' ', 12), 
    HuffmanLeaf::new('e', 8),
    HuffmanLeaf::new('s', 6),
    HuffmanLeaf::new('l', 6),
    HuffmanLeaf::new('u', 5),
    HuffmanLeaf::new('r', 5),
    HuffmanLeaf::new('i', 4),
    HuffmanLeaf::new('a', 4),
    HuffmanLeaf::new('d', 3),
    HuffmanLeaf::new('o', 3),
    HuffmanLeaf::new('m', 2),
    HuffmanLeaf::new('p', 1),
    HuffmanLeaf::new('j', 3),
    HuffmanLeaf::new('\'', 2),
    HuffmanLeaf::new('b', 1)
); 

// perform the coding
let ot = huffman::code(v);

// which returns an Option<HuffmanBranch<T>> (None is returned when the input vector contains 0 or 1 items)
let t = ot.unwrap();

match *t.left {
    HuffmanNode::HuffmanLeaf(ref l) => /* do something with the left node. l.item contains the char. */,
    HuffmanNode::HuffmanBranch(ref b) => /* do something with the left node */,
};    

match *t.right {
    HuffmanNode::HuffmanLeaf(ref l) => /* do something with the right node. l.item contains the char. */,
    HuffmanNode::HuffmanBranch(ref b) => /* do something with the right node */,
};  
```
