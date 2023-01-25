use std::{cmp::min, fmt, mem, vec};

use super::arena::{Arena, ID};

/// Radix tree is a space-optimized prefix-tree as a key-value storage
pub struct RadixTree<T> {
    arena: Arena<Exact<T>>,
    root: ID,
}

impl<T> RadixTree<T> {
    pub fn new() -> Self {
        let mut arena = Arena::new();
        let root = Exact {
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

    /// find a node by key
    // fn find_node(&self, key: Vec<u8>) -> Option<&T> {

    // }

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
        let new_child = self.arena.push(Exact {
            key,
            value: Some(value),
            children: Vec::new(),
        });
        self.arena[node_id].children.push(new_child);
    }

    /// fragment a node
    fn fragment_node(&mut self, node_id: ID, len: usize, value: Option<T>) {
        // create the intermediate node
        let node = &mut self.arena[node_id];
        let key = node.key.clone();
        let children = node.children.clone();
        let (base, rest) = key.split_at(len);

        let intermediate = Exact {
            children,
            value: mem::replace(&mut node.value, None),
            key: rest.to_vec(),
        };
        let intermediate_id = self.arena.push(intermediate);

        // update the node
        let node = &mut self.arena[node_id];
        node.children = vec![intermediate_id];
        node.value = value;
        node.key = base.to_vec();
    }

    // remove a node
    // fn remove_node(&mut self, node_id: ID) -> Option<T> {

    // }
}

/// A node in the tree
pub struct Exact<T> {
    key: Vec<u8>,
    value: Option<T>,
    children: Vec<ID>,
}

pub struct Iterator<'a, T> {
    arena: &'a Arena<Exact<T>>,
    ids: Vec<(ID, usize)>,
    index: usize,
}

impl<'a, T> IntoIterator for &'a RadixTree<T> {
    type Item = (&'a Exact<T>, usize);
    type IntoIter = Iterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut ids: Vec<(ID, usize)> = Vec::new();
        fn walk<T>(arena: &Arena<Exact<T>>, ids: &mut Vec<(ID, usize)>, node_id: ID, depth: usize) {
            ids.push((node_id, depth));
            for child_id in &arena[node_id].children {
                walk(arena, ids, *child_id, depth + 1);
            }
        }
        walk(&self.arena, &mut ids, self.root, 0);

        Iterator {
            arena: &self.arena,
            ids,
            index: 0,
        }
    }
}

impl<'a, T> core::iter::Iterator for Iterator<'a, T> {
    type Item = (&'a Exact<T>, usize);

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

impl<T> fmt::Debug for RadixTree<T>
where
    T: std::fmt::Debug,
{
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

            let value = &node.0.value;
            match value {
                None => (),
                Some(value) => write!(f, " -> {:?}", value)?,
            }

            write!(f, "\n")?;

            // └ ├ ─
        }
        Ok(())
    }
}

pub struct Predictor<'a, T> {
    tree: &'a RadixTree<T>,
    current_match: ID,
    next: Option<ID>,
    key: Vec<u8>,
    path: Vec<ID>,
    exact: bool,
    index: usize,
}

impl<'a, T> Predictor<'a, T> {
    pub fn new(tree: &RadixTree<T>) -> Predictor<T> {
        Predictor {
            tree,
            current_match: tree.root,
            next: Some(tree.root),
            key: Vec::new(),
            exact: false,
            path: Vec::new(),
            index: 0,
        }
    }

    pub fn add(&mut self, key: &Vec<u8>) {
        self.key.append(&mut key.clone());
        self.update();
    }

    pub fn predict(&self) -> Option<Vec<u8>> {
        match self.next {
            None => None,
            Some(id) => {
                let mut b: Vec<u8> = Vec::new();
                for step in &self.path {
                    b.append(&mut self.tree.arena[*step].key.clone());
                }
                b.append(&mut self.tree.arena[id].key.clone());
                Some(b)
            }
        }
    }

    fn update(&mut self) {
        let mut matched_node_id = self.current_match;
        let mut most_common_prefix_count = 0;
        let mut most_common_node_id: Option<ID>;
        let mut current_node;
        'node_iterator: loop {
            current_node = &self.tree.arena[matched_node_id];
            let key = &self.key[self.index..];
            most_common_node_id = None;

            // exact match with the current node
            if key.len() == 0 {
                self.current_match = matched_node_id;
                if !current_node.value.is_none() {
                    self.exact = true;
                    self.next = None;
                    return;
                }
            }

            for child_key in &current_node.children {
                let child_node = &self.tree.arena[*child_key];
                let common_prefix_count = find_common_prefix_count(&child_node.key, key);
                if common_prefix_count > most_common_prefix_count {
                    most_common_prefix_count = common_prefix_count;
                    most_common_node_id = Some(*child_key);
                }

                // the input fully contains the child, proceed to the next node
                if child_node.key.len() == most_common_prefix_count {
                    self.index += child_node.key.len();
                    self.path.push(*child_key);
                    matched_node_id = *child_key;
                    continue 'node_iterator;
                }
            }

            // the input doesn't fully contains any of the current child
            // we set the next node as the one with the highest common prefix
            self.exact = false;
            self.current_match = matched_node_id;
            self.next = most_common_node_id;
            break;
        }
        self.current_match = matched_node_id;
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

#[cfg(test)]
mod test {
    use crate::collections::radix_tree::{self, RadixTree};
    use rand;
    use std;

    use super::Predictor;

    #[test]
    fn push() {
        let mut t: RadixTree<()> = RadixTree::new();
        t.insert("hamster".as_bytes().to_vec(), ());
        t.insert("hamstring".as_bytes().to_vec(), ());
        t.insert("hamburger".as_bytes().to_vec(), ());
        t.insert("hamburgers".as_bytes().to_vec(), ());
        t.insert("hamburg".as_bytes().to_vec(), ());
        t.insert("hams".as_bytes().to_vec(), ());
        t.insert("ham".as_bytes().to_vec(), ());
        dbg!(&t);
        fn expect_prediction<T>(p: &Predictor<T>, str: Option<&str>) {
            match p.predict() {
                None => assert_eq!(str, None),
                Some(value) => assert_eq!(str.unwrap(), std::str::from_utf8(&value).unwrap()),
            }
        }

        let mut p = Predictor::new(&t);
        p.add(&"h".as_bytes().to_vec());
        expect_prediction(&p, Some("ham"));
        p.add(&"a".as_bytes().to_vec());
        expect_prediction(&p, Some("ham"));
        p.add(&"m".as_bytes().to_vec());
        expect_prediction(&p, None);
        p.add(&"b".as_bytes().to_vec());
        expect_prediction(&p, Some("hamburg"));
        p.add(&"u".as_bytes().to_vec());
        expect_prediction(&p, Some("hamburg"));
        p.add(&"r".as_bytes().to_vec());
        expect_prediction(&p, Some("hamburg"));
        p.add(&"g".as_bytes().to_vec());
        expect_prediction(&p, None);
        p.add(&"e".as_bytes().to_vec());
        expect_prediction(&p, Some("hamburger"));
        p.add(&"r".as_bytes().to_vec());
        expect_prediction(&p, None);
        p.add(&"s".as_bytes().to_vec());
        expect_prediction(&p, None);
    }
}
