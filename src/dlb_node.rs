use crate::char_list::CharList;
use crate::internal_data::InternalData;
use crate::leaf_data::LeafData;
use crate::Identifier;
use std::sync::atomic::{AtomicU64, Ordering};

pub enum DLBNode {
    Leaf(LeafData),
    Internal(InternalData),
}

impl DLBNode {
    pub fn insert(&mut self, mut pattern: CharList, next_id: &mut AtomicU64) -> Identifier {
        match self {
            DLBNode::Leaf(data) => {
                // Consume any if the characters in pattern which match on
                // data.bytes().
                let similarity = data.similar_bytes(pattern.clone());
                // Case 1: Exact match
                // If no bytes remain, return my integer.
                if similarity == pattern.len() && similarity == data.bytes().len() {
                    return data.id();
                }
                // Case 2: No match.
                if similarity == 0 {
                    unreachable!("Similarity required to have gotten his far");
                }
                // Case 3: Patterns matches fully, but pattern.len() > self.data.len()
                // TODO
                //    Create an internal node. IsComplete is true.
                //    Have it's children be a leaf with the remaining bytes.
                if similarity == data.bytes().len() && pattern.len() < data.bytes().len() {
                    let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                    let remaining = pattern.split_off(similarity);
                    let new_leaf_data = LeafData::new(id, remaining);
                    let new_leaf = DLBNode::Leaf(new_leaf_data);
                    // Build the internal node
                    let new_bytes = pattern;
                    let maybe_id = Some(data.id());
                    let children = vec![new_leaf];
                    *self = DLBNode::Internal(InternalData::new(new_bytes, maybe_id, children));
                    return id;
                }

                // Case 4: Partial Match
                //    with the remaining bytes, convert self into an internal_node.
                //    Add a child with the remaining bytes.
                let remaining = pattern.split_off(similarity);
                // Create a new leaf with the remaining bytes.
                let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                let new_leaf_data = LeafData::new(id, remaining);
                let new_leaf = DLBNode::Leaf(new_leaf_data);
                // Make a second leaf with the remaining bytes from self.
                let remaining_self_bytes: Vec<u8> =
                    data.bytes().clone().into_iter().skip(similarity).collect();
                let new_self_data = LeafData::new(data.id(), CharList::from(remaining_self_bytes));
                let self_node = DLBNode::Leaf(new_self_data);
                // Create a new internal node with the matching pattern
                // The new leaf and this node are the children.
                let new_bytes = pattern;
                let maybe_id = None;
                let children = vec![new_leaf, self_node];
                *self = DLBNode::Internal(InternalData::new(new_bytes, maybe_id, children));
                return id;
            }
            DLBNode::Internal(data) => {
                // Consume any of the characters in pattern which match on
                // data.bytes().
                let similarity = data.similar_bytes(pattern.clone());
                // Case 1: Exact match
                // In this case, we just need to check the IsComplete field
                // and perhaps update it.
                if similarity == pattern.len() && similarity == data.bytes().len() {
                    if let Some(id) = data.maybe_id() {
                        return id;
                    } else {
                        let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                        data.set_maybe_id(Some(id));
                        return id;
                    }
                }
                // Case 2: No match.
                if similarity == 0 {
                    unreachable!("Similarity required to have gotten his far");
                }
                // Case 3: Partial match, where pattern.len() > data.bytes().len()
                //     In this case, we need to pass the remaining bytes along to our children.
                //     If our children have no overlap, then we make a new leaf.
                if similarity == data.bytes().len() && data.bytes().len() < pattern.len() {
                    // Chop off the matching part.
                    let remaining = pattern.split_off(similarity);
                    let (best_index, _count) = data.find_best_child(remaining.clone());
                    match best_index {
                        Some(idx) => {
                            return data.insert_at_index(idx, remaining, next_id);
                        }
                        None => {
                            // Make a new leaf and add it as a child.
                            let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                            let new_leaf_data = LeafData::new(id, remaining);
                            let new_leaf = DLBNode::Leaf(new_leaf_data);
                            data.add_child(new_leaf);
                            return id;
                        }
                    }
                }

                // Case 4: Partial match, where similarity < data.bytes().
                //     In this case, we need to make a new internal node for the
                //     matching patern, and that internal node will point to this node,
                //     with the matching part removed.`
            }
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
