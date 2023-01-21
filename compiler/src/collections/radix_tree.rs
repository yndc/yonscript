use std::{cmp::min, fmt, vec};

use super::arena::{Arena, ID};

/// Radix tree is a space-optimized prefix-tree as a key-value storage
pub struct RadixTree<T> {
    arena: Arena<Node<T>>,
    root: ID,
}

impl<T> RadixTree<T> {
    pub fn new() -> Self {
        let mut arena = Arena::new();
        let root = Node {
            key: Vec::new(),
            value: None,
            children: Vec::new(),
        };
        let root = arena.push(root);
        Self { arena, root }
    }

    /// Add a string into a node
    pub fn insert(&mut self, key: Vec<u8>, value: T) {
        let root_id = self.root;
        self.insert_to_node(root_id, key, value);
    }

    fn insert_to_node(&mut self, node_id: ID, key: Vec<u8>, value: T) {
        let node = &self.arena[node_id];
        for child_id_ref in &node.children {
            let child_id = *child_id_ref;
            let child = &self.arena[child_id];
            // input and child matched perfectly, insertion is no-op
            if child.key == key {
                return;
            }

            let common_prefix_count = find_common_prefix_count(&child.key, &key);
            let child_key_len = child.key.len();

            // child contains the input, fragment the child
            if key.len() == common_prefix_count {
                self.fragment_node(child_id, key.len(), Some(value));
                return;

            // the input contains the child, fragment the input
            } else if child_key_len == common_prefix_count {
                self.insert_to_node(child_id, key[child_key_len..].to_vec(), value);
                return;

            // both shares a common prefix
            } else if common_prefix_count > 0 {
                println!("{}", common_prefix_count);
                self.fragment_node(child_id, common_prefix_count, None);
                self.insert_to_node(child_id, key[common_prefix_count..].to_vec(), value);
                return;
            }
        }

        // no child matched with the input, create a new child
        let new_child = self.arena.push(Node {
            key,
            value: Some(value),
            children: Vec::new(),
        });
        self.arena[node_id].children.push(new_child);
    }

    /// fragment a node
    fn fragment_node(&mut self, node_id: ID, len: usize, value: Option<T>) {
        let node = &self.arena[node_id];
        let (_base, rest) = node.key.split_at(len);
        let intermediate = Node {
            children: node.children.clone(),
            value: None,
            key: rest.to_vec(),
        };

        let intermediate_id = self.arena.push(intermediate);

        let node = &mut self.arena[node_id];
        let (base, _rest) = node.key.split_at(len);
        node.children = vec![intermediate_id];
        node.value = value;
        node.key = base.to_vec();
    }
}

pub struct RadixTreeIterator<'a, T> {
    arena: &'a Arena<Node<T>>,
    ids: Vec<(ID, usize)>,
    index: usize,
}

impl<'a, T> IntoIterator for &'a RadixTree<T> {
    type Item = (&'a Node<T>, usize);
    type IntoIter = RadixTreeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut ids: Vec<(ID, usize)> = Vec::new();
        fn walk<T>(arena: &Arena<Node<T>>, ids: &mut Vec<(ID, usize)>, node_id: ID, depth: usize) {
            ids.push((node_id, depth));
            for child_id in &arena[node_id].children {
                walk(arena, ids, *child_id, depth + 1);
            }
        }
        walk(&self.arena, &mut ids, self.root, 0);

        RadixTreeIterator {
            arena: &self.arena,
            ids,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for RadixTreeIterator<'a, T> {
    type Item = (&'a Node<T>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.ids.len() {
            None
        } else {
            let node = &self.arena[self.ids[self.index].0];
            let depth = self.ids[self.index].1;
            self.index += 1;
            Some((node, depth))
        }
    }
}

impl<T> fmt::Debug for RadixTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for node in self {
            if node.1 == 1 {
                let str = std::str::from_utf8(&node.0.key).unwrap();
                write!(f, "{}", str)?;
            }
            if node.1 > 1 {
                let str = std::str::from_utf8(&node.0.key).unwrap();
                write!(f, "{}├ {}", "│ ".repeat(node.1 - 2), str)?;
            }

            write!(f, "\n")?;

            // └ ├ ─
        }
        Ok(())
    }
}

/// A node in the tree
pub struct Node<T> {
    key: Vec<u8>,
    value: Option<T>,
    children: Vec<ID>,
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
