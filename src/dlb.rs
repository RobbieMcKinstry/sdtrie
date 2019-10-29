use crate::char_list::CharList;
use crate::dlb_node::DLBNode;
use crate::leaf_data::LeafData;
use crate::Identifier;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct DLB {
    root: Option<DLBNode>,
    next_id: AtomicU64,
}

impl DLB {
    pub fn new() -> Self {
        return Self {
            root: None,
            next_id: AtomicU64::new(1),
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

        let root_node = self.root.as_ref().unwrap();
        let byte_pattern = s.as_bytes();
        root_node.get(byte_pattern)
    }

    pub fn get_or_intern(&mut self, s: String) -> Identifier {
        // Check if root is empty:
        let bytes = CharList::from(s.into_bytes());
        if self.is_empty() {
            // Make a new leaf node.
            let id = self.new_id();
            let new_node_data = LeafData::new(id, bytes);
            let new_node = DLBNode::Leaf(new_node_data);
            self.root = Some(new_node);
            return id;
        }

        self.root.as_mut().unwrap().insert(bytes)
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
