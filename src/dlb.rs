use crate::char_list::CharList;
use crate::dlb_node::DLBNode;
use crate::internal_data::InternalData;
use crate::leaf_data::LeafData;
use crate::Identifier;
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
            Some(_) => true,
            None => false,
        }
    }

    fn contains(&self, s: String) -> bool {
        if self.is_empty() {
            return false;
        }

        if s.is_empty() {
            return self.contains_empty.is_some();
        }

        let root_node = self.root.as_ref().unwrap();
        let byte_pattern = s.as_bytes();
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

    pub fn get_or_intern(&mut self, s: String) -> Identifier {
        // Check if root is empty:
        let mut bytes = CharList::from(s.clone().into_bytes());
        // Special case where the input string is empty.
        if s.is_empty() {
            match self.contains_empty {
                Some(id) => return id,
                None => {
                    let id = self.new_id();
                    self.contains_empty = Some(id);
                    return id;
                }
            }
        }

        // Special case where the trie itself is empty.
        if self.is_empty() {
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
            return id;
        }

        // Now we know the root isn't empty.
        // Because of our special casing above, we
        // also know the root isn't a leaf.
        match self.root.as_mut().unwrap() {
            DLBNode::Leaf(_) => unreachable!("Root is always an internal node"),
            DLBNode::Internal(data) => {
                let (best, count) = data.find_best_child(bytes.clone());
                // Destructure the result, and recurse.
                if let Some(next) = best {
                    let remaining = bytes.split_off(count);
                    return next.insert(remaining);
                } else {
                    // Ain't nobody matching this node.
                    // Add it directly as a leaf off of the root.
                    let id = self.new_id();
                    let new_node_data = LeafData::new(id, bytes);
                    let new_leaf = DLBNode::Leaf(new_node_data);
                    data.add_child(new_leaf);
                    return id;
                }
            }
        }

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
}
