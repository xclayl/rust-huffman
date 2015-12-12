use std::cmp;
use std::collections::BinaryHeap;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};

/// The input vector will be empty when fn is done
/// returns None if there is 0 or 1 items in the input vector
pub fn code<T>(input: Vec<HuffmanLeaf<T>>) -> Option<HuffmanBranch<T>> {


    let mut heap = BinaryHeap::with_capacity(input.len());

    for i in input {
        heap.push(HuffmanNode::HuffmanLeaf(i));
    }
    
    
    while heap.len() >= 2 {
        let first = heap.pop().unwrap();
        let second = heap.pop().unwrap();
        
        heap.push(HuffmanNode::HuffmanBranch(HuffmanBranch::new(Box::new(first), Box::new(second))));
        
    }
    

    match heap.pop() {
        Some(b) => 
            match b { 
                HuffmanNode::HuffmanBranch(b2) => Some(b2),
                _ => None,
            },
        _ => None
    }
    
    
}

pub enum HuffmanNode<T> {
    HuffmanBranch(HuffmanBranch<T>),
    HuffmanLeaf(HuffmanLeaf<T>),
}

pub struct HuffmanLeaf<T> {
    /// The character, symbol, or item to code
    pub item: T,
    /// The number of times the item occurs (or predicted to occur)
    pub occurrences: u32,
}

pub struct HuffmanBranch<T> {
    pub left: Box<HuffmanNode<T>>,
    pub right: Box<HuffmanNode<T>>,
    longest_depth: u32,
    sum_occurrences: u32,
}


impl<T> HuffmanNode<T> {
    fn occurrences(&self) -> u32 {
        
        match *self {
            HuffmanNode::HuffmanLeaf(ref l) => l.occurrences,
            HuffmanNode::HuffmanBranch(ref l) => l.sum_occurrences,
        }
        
    }
    
    fn depth(&self) -> u32 {
        
        match *self {
            HuffmanNode::HuffmanLeaf(_) => 0,
            HuffmanNode::HuffmanBranch(ref l) => l.longest_depth,
        }
        
    }
}

impl<T> Ord for HuffmanNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.occurrences() < other.occurrences() {
            Ordering::Greater
        } else if self.occurrences() > other.occurrences() {
            Ordering::Less
        } else if self.depth() < other.depth() {
            Ordering::Greater
        } else if self.depth() > other.depth() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
        
        
    }
}


impl<T> PartialOrd for HuffmanNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))        
    }
}
impl<T> PartialEq for HuffmanNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal        
    }
}
impl<T> Eq for HuffmanNode<T> {
    
}




impl<T> HuffmanLeaf<T> {

    pub fn new(item: T, occurrences: u32) -> HuffmanLeaf<T> {
        HuffmanLeaf {
            item: item,
            occurrences: occurrences
        }
    }

}


impl<T> Debug for HuffmanLeaf<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} {:?})", self.occurrences, self.item)
    }
}

impl<T> HuffmanBranch<T> {

    fn new(left: Box<HuffmanNode<T>>, right: Box<HuffmanNode<T>>) -> HuffmanBranch<T> {
       
        let depth = cmp::max(left.depth(), right.depth()) + 1;
        let sum_occurrences = left.occurrences() + right.occurrences();
    
    
        HuffmanBranch {
            left: left,
            right: right,
            longest_depth: depth,
            sum_occurrences: sum_occurrences,
        }
    }
}