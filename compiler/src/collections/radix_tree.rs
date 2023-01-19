use std::{
    borrow::{Borrow, BorrowMut},
    cmp::min, mem,
};

/// Radix tree is a space-optimized prefix-tree as a key-value storage
#[derive(Debug)]
pub struct RadixTree<T> {
    root: Node<T>,
}

impl<T> RadixTree<T> {
    pub fn new() -> Self {
        Self {
            root: Node {
                base: Vec::new(),
                value: None,
                children: Vec::new(),
            },
        }
    }

    /// Add a string into the radix tree
    pub fn insert(&mut self, key: Vec<u8>, value: T) {
        self.root.insert(key, value);
    }
}

/// A node in the tree
#[derive(Debug)]
struct Node<T> {
    base: Vec<u8>,
    value: Option<T>,
    children: Vec<Node<T>>,
}

impl<T> Node<T> {
    /// Add a string into this node
    fn insert(&mut self, key: Vec<u8>, value: T) {
        for child in &mut self.children {
            // input and child matched perfectly, insertion is no-op
            if child.base == key {
                return;
            }

            let common_prefix_count = find_common_prefix_count(&child.base, &key);

            // child contains the input, fragment the child
            if key.len() == common_prefix_count {
                child.fragment(key.len(), Some(value));
                return;

            // the input contains the child, fragment the input
            } else if child.base.len() == common_prefix_count {
                child.insert(key[child.base.len()..].to_vec(), value);
                return;

            // both shares a common prefix
            } else {
                child.fragment(common_prefix_count, None);
                child.insert(key[child.base.len()..].to_vec(), value);
                return;
            }
        }

        // no child matched with the input, create a new child
        self.children.push(Node {
            base: key,
            value: Some(value),
            children: Vec::new(),
        })
    }

    /// fragment a node
    fn fragment(&mut self, len: usize, value: Option<T>) {
        let (base, rest) = self.base.split_at(len);
        let prev_children = self.children;
        mem::replace(self, Node {
            base: base.to_vec(),
            value: value,
            children: prev_children,
        });
    }
}

/// Get the number of bytes which both given byte arrays has the same values
/// For example, "stop" and "strong" will return 2, as both has the same prefix "st"
fn find_common_prefix_count(node_base_str: &[u8], str2: &[u8]) -> usize {
    let cap = min(node_base_str.len(), str2.len());
    let mut common = 0usize;
    for i in 0..cap {
        if node_base_str[i] != str2[i] {
            break;
        }
        common += 1;
    }

    return common;
}
