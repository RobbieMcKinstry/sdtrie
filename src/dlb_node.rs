use crate::internal_data::InternalData;
use crate::leaf_data::LeafData;
use crate::Identifier;

pub enum DLBNode {
    Leaf(LeafData),
    Internal(InternalData),
}

impl DLBNode {
    pub fn get(&self, pattern: &[u8]) -> Option<Identifier> {
        match self {
            DLBNode::Leaf(data) => {
                // Check if the list matches the rest of the elements:
                if data.bytes().as_slice() == pattern {
                    return Some(data.id());
                }
            }
            DLBNode::Internal(data) => {
                if pattern.len() == 0 {
                    return data.maybe_id();
                }
                if data.bytes().as_slice() != pattern {
                    return None;
                }
                // Else, cut off the prefix from the pattern,
                // and iterate over the children, ORing the results together.
                let match_len = data.bytes().len();
                let suffix = &pattern[match_len..];
                for child in data.children() {
                    if let Some(result) = child.get(&suffix) {
                        return Some(result);
                    }
                }
            }
        }
        None
    }

    pub fn contains(&self, pattern: &[u8]) -> bool {
        match self.get(pattern) {
            Some(_) => true,
            None => false,
        }
    }
}
