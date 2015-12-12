
extern crate huffman;

use huffman::HuffmanNode;
use huffman::HuffmanLeaf;
use huffman::HuffmanBranch;
use std::fmt::{Debug, Formatter, Result};
use std::io;
use std::io::Write;
use std::cmp;

#[derive(PartialEq, Eq, Debug)]
struct NoCopy {
    one: i32,
    two: i32
}



#[test]
fn given_0_items_it_returns_none() {

    let v: Vec<HuffmanLeaf<NoCopy>> = Vec::new();
    let ot = huffman::code(v);
    
    match ot {
        Some(_) => assert!(false),
        None => assert!(true),
    };
    
}

#[test]
fn given_1_item_it_returns_none() {

    let v = vec!(HuffmanLeaf::new(NoCopy {one:10, two: 20}, 7)); 
    let ot = huffman::code(v);
    
    match ot {
        Some(_) => assert!(false),
        None => assert!(true),
    };
    
}

#[test]
fn given_2_items_it_works() {
    let v = vec!(HuffmanLeaf::new(NoCopy {one:1, two: 2}, 5), HuffmanLeaf::new(NoCopy {one:10, two: 20}, 7)); 
    let ot = huffman::code(v);
    
    //assert_eq!(v.len(), 0);
    
    let t = ot.unwrap();
    
    assert_eq!(match *t.left {
        HuffmanNode::HuffmanLeaf(l)     => l.item,
        _                               => NoCopy {one:0, two: 0}
    }, NoCopy {one:1, two: 2});
    
    assert_eq!(match *t.right {
        HuffmanNode::HuffmanLeaf(l)     => l.item,
        _                               => NoCopy {one:0, two: 0}
    }, NoCopy {one:10, two: 20});
    
}



#[test]
fn given_basic_input_it_works() {

/*
https://en.wikipedia.org/wiki/Huffman_coding
space 	7 	111
a 	4 	010
e 	4 	000
f 	3 	1101
h 	2 	1010
i 	2 	1000
m 	2 	0111
n 	2 	0010
s 	2 	1011
t 	2 	0110
l 	1 	11001
o 	1 	00110
p 	1 	10011
r 	1 	11000
u 	1 	00111
x 	1 	10010
*/




    let v = vec!(
        HuffmanLeaf::new(' ', 7), 
        HuffmanLeaf::new('a', 4),
        HuffmanLeaf::new('e', 4),
        HuffmanLeaf::new('f', 3),
        HuffmanLeaf::new('h', 2),
        HuffmanLeaf::new('i', 2),
        HuffmanLeaf::new('m', 2),
        HuffmanLeaf::new('n', 2),
        HuffmanLeaf::new('s', 2),
        HuffmanLeaf::new('t', 2),
        HuffmanLeaf::new('l', 1),
        HuffmanLeaf::new('o', 1),
        HuffmanLeaf::new('p', 1),
        HuffmanLeaf::new('r', 1),
        HuffmanLeaf::new('u', 1),
        HuffmanLeaf::new('x', 1)
    ); 
    

    let mut v_copy = Vec::new();
    for x in v.iter() {
        v_copy.push((x.item, x.occurrences));
    }
    
    let ot = huffman::code(v);
    
    //assert_eq!(v.len(), 0);
    
    let t = ot.unwrap();
    
    let info = to_list(&t);
    
    assert_eq!(info.iter().find(|&l| l.leaf.item == 'a').unwrap().bits.len(), 3); 
    assert_eq!(info.iter().find(|&l| l.leaf.item == 'f').unwrap().bits.len(), 4); 
    assert_eq!(info.iter().find(|&l| l.leaf.item == 'l').unwrap().bits.len(), 5); 
    
    
    let msgLen = info.iter().fold(0, |acc, ref x| acc + x.bits.len() * v_copy.iter().find(|&l| l.0 == x.leaf.item).unwrap().1 as usize);    
    assert_eq!(msgLen, 135); 
    
    //let w = BufWriter::new(
    
    write!(&mut io::stdout(), "{:?}", info);
    
}



#[test]
fn given_input_it_should_produce_shortest_depth() {

    
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


    let mut v_copy = Vec::new();
    for x in v.iter() {
        v_copy.push((x.item, x.occurrences));
    }

    let ot = huffman::code(v);

    
    let t = ot.unwrap();
    
    
    let info = to_list(&t);
    
    assert_eq!(info.iter().fold(0, |acc, ref x| cmp::max(acc, x.bits.len())), 6);
    
    
    let msgLen = info.iter().fold(0, |acc, ref x| acc + x.bits.len() * v_copy.iter().find(|&l| l.0 == x.leaf.item).unwrap().1 as usize);    
    assert_eq!(msgLen, 239); 
    
    
    write!(&mut io::stdout(), "{:?}", info);
}

struct LeafInfo<'a, T> where T: 'a {
    leaf: &'a HuffmanLeaf<T>,
    bits: Vec<bool>
}

impl<'a, T> Debug for LeafInfo<'a, T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut bits = String::new();
        for &b in self.bits.iter() {
            bits.push(if b {'1'} else {'0'});
        }
    
        write!(f, "{}: {:?}", bits, self.leaf)
    }
}

fn to_list<'a, T>(tree: &'a HuffmanBranch<T>) -> Vec<LeafInfo<'a, T>> {
    let mut result: Vec<LeafInfo<T>> = Vec::new();
    to_list_inner_branch(tree, Vec::new(), &mut result);
    result
}



fn to_list_inner_leaf<'a, T>(leaf: &'a HuffmanLeaf<T>, bits: Vec<bool>, result: & mut Vec<LeafInfo<'a, T>>) {
    result.push(LeafInfo { leaf: leaf, bits: bits });
}

fn to_list_inner_branch<'a, T>(tree: &'a HuffmanBranch<T>, bits: Vec<bool>, result: & mut Vec<LeafInfo<'a, T>>) {
    

    let mut left_bits = bits.clone();
    left_bits.push(false);
    
    match *tree.left {
        HuffmanNode::HuffmanLeaf(ref l) => to_list_inner_leaf(l, left_bits, result),
        HuffmanNode::HuffmanBranch(ref b) => to_list_inner_branch(b, left_bits, result),
    };    
    
    
    let mut right_bits = bits.clone();
    right_bits.push(true);
    
    match *tree.right {
        HuffmanNode::HuffmanLeaf(ref l) => to_list_inner_leaf(l, right_bits, result),
        HuffmanNode::HuffmanBranch(ref b) => to_list_inner_branch(b, right_bits, result),
    };
    
}