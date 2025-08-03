### Compatibility Considerations
So, we serialize it as bits first, or just characters, then do frequency counting of the
characters, then group them in a bottom-up approach? write the bits
read them as bytes so that we can work with any type of data not just strings

### Tree Formation Considerations
Initially all nodes would be leaves but let's add them as Nodes for our convenience,
we have the option anyways..
so let's convert them into (byte, cnt) and add them into a min-heap

### Node considerations
We have to use Box<> to avoid the recursive size error... since rust determines the space to allocate at compile time
This way it'll only allow space for a reference, and we can use Node within Node

### Designing the Huffman Tree

The Huffman Tree will be held within a min-heap itself.
Each nodes `Node` of the huffman tree will be pushed into the heap 
and combined as we go.

The final Huffman Tree would be within a root `Node` that we would get from the heap.

Rust has the BinaryHeap as a max-heap by default

How do you do that ? .. you use traits
.. you make the BinaryHeap use the Ord trait but provide a custom implementation for it

```rust
pub trait Ord: PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

- cmp() tells how things are compared
- Ordering is an enum: Less, Equal, Greater

### Serialization

For generating the final compressed output file, we would need to have the compressed bitstream and the huffman tree
stored in the file. We'll use bincode for this.
