use crate::char_list::CharList;
use crate::internal_data::InternalData;
use crate::leaf_data::LeafData;
use crate::Identifier;

pub enum DLBNode {
    Leaf(LeafData),
    Internal(InternalData),
}

impl DLBNode {
    pub fn insert(&mut self, pattern: CharList) -> Identifier {
        match self {
            DLBNode::Leaf(data) => {
                // Consume any if the characters in pattern which match on
                // data.bytes().
                let similarity = data.similar_bytes(pattern.clone());
                // Case 1: Full match
                // If no bytes remain, return my integer.
                if similarity == pattern.len() {
                    return data.id();
                }

                // Case 2: Partial Match

                // Case 3: No match.

                // Else:
                //    with the remaining bytes, convert self into an internal_node.
                //    Add a child with the remaining bytes.
            }
            DLBNode::Internal(data) => {}
        }

        Identifier::from(0)
    }

    pub fn similar_bytes(&self, pattern: CharList) -> usize {
        match self {
            DLBNode::Leaf(data) => data.similar_bytes(pattern),
            DLBNode::Internal(data) => data.similar_bytes(pattern),
        }
    }

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
