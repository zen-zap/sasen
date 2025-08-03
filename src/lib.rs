use serde::{Serialize, Deserialize};

pub mod compression;

/// Denotes a Leaf byte
pub type Leaf = u8;

/// Denotes a Node, can be a Leaf or can have (left, right) children which are both of type `Node`
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    /// hold the frequency count for the current Node
    pub cnt: usize,
    /// holds the value incase of a leaf node
    pub leaf: Option<Leaf>,
    /// holds references to left and right Nodes inside a Box
    pub children: Option<Box<(Node, Node)>>,
}

use std::cmp::Ordering;

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cnt == other.cnt
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // the comparison is reversed here to form the min-heap
        other.cnt.cmp(&self.cnt)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // delegating to Ord
    }
}

/// This is serialized into the final output file
#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedFile {
    pub tree: Node,
    pub data: Vec<u8>,
}
