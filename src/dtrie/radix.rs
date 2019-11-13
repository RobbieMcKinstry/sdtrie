use crate::dtrie::char_list::CharList;
use crate::dtrie::leaf_data::LeafData;
use crate::dtrie::matchable::Matchable;
use crate::dtrie::radix_node::RadixNode;
use crate::dtrie::Identifier;
use std::mem::size_of;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct RadixTrie {
    root: Vec<RadixNode>,
    next_id: AtomicU64,
    // if the empty string is an element,
    // then this field contains it's ID.
    contains_empty: Option<Identifier>,
}

impl RadixTrie {
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

    pub fn contains(&self, s: String) -> bool {
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
        println!("Getting  string {}", s);
        println!("Can't wait to see my strings!");
        if self.is_empty() {
            return None;
        }

        if s.is_empty() {
            return self.contains_empty;
        }

        let byte_pattern = s.as_bytes();
        for child in self.root.iter() {
            println!("Checking child {}", child.bytes());
            if let Some(id) = child.get(byte_pattern) {
                return Some(id);
            }
        }
        None
    }

    pub fn size_of(&self) -> usize {
        let self_size = size_of::<Self>();
        let child_size = self
            .root
            .iter()
            .map(|child| child.size_of())
            .fold(0, |x, acc| x + acc);
        self_size + child_size
    }

    pub fn count_nodes(&self) -> u64 {
        if self.is_empty() {
            return 0;
        }

        self.root
            .iter()
            .map(|node| node.count_nodes())
            .fold(0, |acc, inc| acc + inc)
    }

    pub fn get_or_intern(&mut self, s: String) -> Identifier {
        // Check if root is empty:
        let bytes = CharList::from(s.clone().into_bytes());
        // Special case where the input string is empty.
        if s.is_empty() {
            return self.intern_empty_string();
        }

        // Special case where the trie itself is empty.
        if self.is_empty() {
            return self.intern_empty_trie(bytes);
        }

        // Now we know the root isn't empty.
        // Because of our special casing above, we
        // also know the root isn't a leaf.
        self.intern(bytes)
    }

    pub fn intern_empty_string(&mut self) -> Identifier {
        match self.contains_empty {
            Some(id) => id,
            None => {
                let id = self.new_id();
                self.contains_empty = Some(id);
                id
            }
        }
    }

    pub fn intern_empty_trie(&mut self, bytes: CharList) -> Identifier {
        // Make a new leaf node.
        let id = self.new_id();
        let new_node_data = LeafData::new(id, bytes);
        let new_leaf = RadixNode::Leaf(new_node_data);

        self.root.push(new_leaf);
        id
    }

    fn find_best_match(&self, bytes: CharList) -> (Option<usize>, usize) {
        let mut matching_index = None;
        let mut match_length = 0;
        for (idx, child) in self.root.iter().enumerate() {
            let matching = child.similar_bytes(bytes.clone());
            if matching > 0 {
                matching_index = Some(idx);
                match_length = matching;
                break;
            }
        }
        return (matching_index, match_length);
    }

    fn add_new_leaf(&mut self, bytes: CharList) -> Identifier {
        let id = self.new_id();
        let new_leaf_data = LeafData::new(id, bytes);
        let new_leaf = RadixNode::Leaf(new_leaf_data);
        self.root.push(new_leaf);
        return id;
    }

    pub fn intern(&mut self, bytes: CharList) -> Identifier {
        let (matching_index, _) = self.find_best_match(bytes.clone());

        // If we found nothing, make a new leaf.
        if let None = matching_index {
            return self.add_new_leaf(bytes);
        }
        // Else, add this pattern to the longest one we have.
        let idx = matching_index.unwrap();
        return self.root[idx].insert(bytes, &mut self.next_id);
    }

    pub fn resolve(&self, id: Identifier) -> Option<String> {
        if self.is_empty() {
            return None;
        }

        if let Some(empty) = self.contains_empty {
            if empty == id {
                return Some("".to_owned());
            }
        }

        let res = self
            .root
            .iter()
            .map(|child| child.resolve(id, CharList::empty()))
            .filter(|x| x.is_some())
            .next();
        match res {
            None => None,
            Some(x) => Some(x.unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        println!("Running empty test");
        let dlb = RadixTrie::new();
        assert_eq!(dlb.is_empty(), true);
    }

    #[test]
    fn test_simple_get() {
        println!("Running simple get test");
        let string = "foo".to_owned();
        let mut dlb = RadixTrie::new();
        let id = dlb.get_or_intern(string.clone());
        assert_eq!(dlb.is_empty(), false);
        let found = dlb.contains(string.clone());
        assert_eq!(found, true);
        let found_id = dlb.get(string.clone()).unwrap();
        assert_eq!(id, found_id);
    }

    #[test]
    fn test_two_leaves() {
        println!("Running two leaves test");
        let strings = vec![String::from("foo"), String::from("boo")];
        let mut dlb = RadixTrie::new();
        strings
            .clone()
            .into_iter()
            .map(|string| (dlb.get_or_intern(string.clone()), dlb.get(string)))
            .for_each(|(orig, obs)| {
                assert_eq!(Some(orig), obs);
            });
    }

    #[test]
    fn test_not_contained() {
        println!("Running not contained test");
        let mut dlb = RadixTrie::new();
        vec!["foo", "boo", "food", "god", "goodbye"]
            .into_iter()
            .map(|x| String::from(x))
            .for_each(|x| {
                dlb.get_or_intern(x);
            });
        let not_contained = vec!["foog", "fb", "boob", "foodstuff", "fish", "goodnight"];
        not_contained
            .into_iter()
            .map(|x| String::from(x))
            .map(|x| dlb.contains(x))
            .for_each(|x| assert_eq!(x, false));
    }
}
