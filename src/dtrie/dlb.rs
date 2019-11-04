use crate::dtrie::char_list::CharList;
use crate::dtrie::dlb_node::DLBNode;
use crate::dtrie::internal_data::InternalData;
use crate::dtrie::leaf_data::LeafData;
use crate::dtrie::Identifier;
use std::str::from_utf8;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct DLB {
    root: Vec<DLBNode>,
    next_id: AtomicU64,
    // if the empty string is an element,
    // then this field contains it's ID.
    contains_empty: Option<Identifier>,
}

impl DLB {
    pub fn new() -> Self {
        return Self {
            root: vec![],
            next_id: AtomicU64::new(1),
            contains_empty: None,
        };
    }

    pub fn is_empty(&self) -> bool {
        self.root.len() == 0
    }

    fn contains(&self, s: String) -> bool {
        // Check if the string is empty.
        if s.is_empty() {
            println!("String is empty!");
            return self.contains_empty.is_some();
        }

        // Handle general case
        let byte_pattern = s.as_bytes();
        for child in self.root.iter() {
            if child.contains(byte_pattern) {
                return true;
            }
        }
        false
    }

    fn new_id(&mut self) -> Identifier {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        Identifier::from(id)
    }

    pub fn get(&self, s: String) -> Option<Identifier> {
        if self.is_empty() {
            return None;
        }

        if s.is_empty() {
            return self.contains_empty;
        }

        let byte_pattern = s.as_bytes();
        for child in self.root.iter() {
            if let Some(id) = child.get(byte_pattern) {
                return Some(id);
            }
        }
        None
    }

    pub fn get_or_intern(&mut self, s: String) -> Identifier {
        // Check if root is empty:
        let bytes = CharList::from(s.clone().into_bytes());
        // Special case where the input string is empty.
        if s.is_empty() {
            return self.case_empty_string();
        }

        // Special case where the trie itself is empty.
        if self.is_empty() {
            return self.case_empty_trie(bytes);
        }

        // Now we know the root isn't empty.
        // Because of our special casing above, we
        // also know the root isn't a leaf.
        self.case_general(bytes)
    }
    pub fn case_empty_string(&mut self) -> Identifier {
        match self.contains_empty {
            Some(id) => id,
            None => {
                let id = self.new_id();
                self.contains_empty = Some(id);
                id
            }
        }
    }

    pub fn case_empty_trie(&mut self, bytes: CharList) -> Identifier {
        // Make a new leaf node.
        let id = self.new_id();
        let new_node_data = LeafData::new(id, bytes);
        let new_leaf = DLBNode::Leaf(new_node_data);

        self.root.push(new_leaf);
        id
    }

    pub fn case_general(&mut self, mut bytes: CharList) -> Identifier {
        // TODO don't look for the best.
        // Just get the first which is > 0.
        let mut best_index = None;
        for (index, child) in self.root.iter().enumerate() {
            let matching = child.similar_bytes(bytes.clone());
            if matching > 0 {
                best_index = Some(index);
                break;
            }
        }

        if let None = best_index {
            // Since we found nothing, make a new leaf.
            let id = self.new_id();
            let new_leaf_data = LeafData::new(id, bytes);
            let new_leaf = DLBNode::Leaf(new_leaf_data);
            self.root.push(new_leaf);
            return id;
        }
        let index = best_index.unwrap(); // Get node with highest similarity.
        let matching = self.root[index].similar_bytes(bytes.clone()); // How much did they match by?
        let remaining = bytes.split_off(matching);
        return self.root[index].insert(remaining, &mut self.next_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let dlb = DLB::new();
        assert_eq!(dlb.is_empty(), true);
    }

    #[test]
    fn test_simple_contains() {
        let string = "foo".to_owned();
        let mut dlb = DLB::new();
        let id = dlb.get_or_intern(string.clone());
        assert_eq!(dlb.is_empty(), false);
        let found = dlb.contains(string.clone());
        assert_eq!(found, true);
        let found_id = dlb.get(string.clone()).unwrap();
        assert_eq!(id, found_id);
    }
}
