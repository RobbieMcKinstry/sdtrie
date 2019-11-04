use crate::dtrie::char_list::CharList;
use crate::dtrie::dlb_node::DLBNode;
use crate::dtrie::internal_data::InternalData;
use crate::dtrie::leaf_data::LeafData;
use crate::dtrie::Identifier;
use std::str::from_utf8;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct DLB {
    root: Option<DLBNode>,
    next_id: AtomicU64,
    // if the empty string is an element,
    // then this field contains it's ID.
    contains_empty: Option<Identifier>,
}

impl DLB {
    pub fn new() -> Self {
        return Self {
            root: None,
            next_id: AtomicU64::new(1),
            contains_empty: None,
        };
    }

    pub fn is_empty(&self) -> bool {
        match self.root {
            Some(_) => false,
            None => true,
        }
    }

    fn contains(&self, s: String) -> bool {
        println!("Contains called with string {}", s.clone());
        if self.is_empty() {
            println!("empty");
            return false;
        }
        println!("Not empty");

        if s.is_empty() {
            println!("String is empty!");
            return self.contains_empty.is_some();
        }

        let root_node = self.root.as_ref().unwrap();
        let byte_pattern = s.as_bytes();

        println!(
            "Passing down the byte pattern {}",
            from_utf8(byte_pattern).unwrap()
        );
        root_node.contains(byte_pattern)
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

        let root_node = self.root.as_ref().unwrap();
        let byte_pattern = s.as_bytes();
        root_node.get(byte_pattern)
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

        // Then, make a special internal node with an empty transition
        let new_empty_list = CharList::empty();
        //    Make a list of children for the new internal node about to be built
        //    It should be composed of the new leaf only.
        let new_children = vec![new_leaf];
        let new_internal_data = InternalData::new(new_empty_list, None, new_children);
        self.root = Some(DLBNode::Internal(new_internal_data));
        id
    }

    pub fn case_general(&mut self, mut bytes: CharList) -> Identifier {
        match self.root.as_mut().unwrap() {
            DLBNode::Leaf(_) => unreachable!("Root is always an internal node"),
            DLBNode::Internal(data) => {
                let (child_index, count) = data.find_best_child(bytes.clone());
                // Destructure the result, and recurse.
                if let Some(idx) = child_index {
                    // TODO fix: you can't eliminate these
                    // bytes because the count > larger
                    // than the number of bytes in
                    // the next node.
                    // let remaining = bytes.split_off(count);
                    data.insert_at_index(idx, bytes, &mut self.next_id)
                } else {
                    // Ain't nobody matching this node.
                    // Add it directly as a leaf off of the root.
                    let new_node_data = LeafData::new_from_generator(bytes, &mut self.next_id);
                    let id = new_node_data.id();
                    let new_leaf = DLBNode::Leaf(new_node_data);
                    data.add_child(new_leaf);
                    id
                }
            }
        }
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

        /*
        // TODO add a check for the root being an internal node.
        // If it's internal, then we don't need to do the similarity check.
        // If it's a leaf, then we check the similarity for the 0 case.
        // If the similarity > 0, then we can recurse. Else handle special.
        //
        // If the root is an internal node, then make sure it

        // Special case where the root has no similarity to the new string.
        if self.root.as_ref().unwrap().similar_bytes(bytes.clone()) == 0 {
            // Then, convert this root into an
            // internal node with two branches.
            //    Get the ID of the new leaf node for this pattern.
            let id = self.new_id();
            //    Make the leaf node, composing the ID and the pattern.
            let new_leaf = DLBNode::Leaf(LeafData::new(id, bytes));
            //    Make an empty list, indicating the epsilon transition to this node.
            let new_empty_list = charlist::empty();
            //    make a list of children for the new internal node about to be built
            //    it should be composed of the current root and the new leaf.
            let new_children = vec![new_leaf, self.root.unwrap()];
            let new_internal_data = internaldata::new(new_empty_list, none, new_children);
            let new_root = dlbnode::internal(new_internal_data);
            self.root = Some(new_root);
        }

        self.root.as_mut().unwrap().insert(bytes)
        */
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
        let _id = dlb.get_or_intern(string.clone());
        assert_eq!(dlb.is_empty(), false);
        let found = dlb.contains(string.clone());
        assert_eq!(found, true);
        // let found_id = dlb.get(string.clone()).unwrap();
        // assert_eq!(id, found_id);
    }
}
