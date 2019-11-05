use crate::dtrie::char_list::CharList;
use crate::dtrie::internal_data::InternalData;
use crate::dtrie::leaf_data::LeafData;
use crate::dtrie::Identifier;
use crate::dtrie::Matchable;
use std::str::from_utf8;
use std::sync::atomic::{AtomicU64, Ordering};

pub enum DLBNode {
    Leaf(LeafData),
    Internal(InternalData),
}

type NodeDescription = (Identifier, CharList);

impl DLBNode {
    fn build_full_match(left: NodeDescription, right: NodeDescription) -> Self {
        // Order them according to length
        let (smallest, mut largest) = if left.1.len() < right.1.len() {
            (left, right)
        } else {
            (right, left)
        };
        let remaining = largest.1.split_off(smallest.1.len());
        // Make a leaf with the remaining bytes.
        let leaf_data = LeafData::new(largest.0, remaining);
        let leaf = DLBNode::Leaf(leaf_data);
        // Now build the internal node.
        let child = vec![leaf];
        let internal_data = InternalData::new(smallest.1, Some(smallest.0), child);
        Self::Internal(internal_data)
    }

    pub fn insert(&mut self, mut pattern: CharList, next_id: &mut AtomicU64) -> Identifier {
        match self {
            DLBNode::Leaf(data) => {
                // Consume any if the characters in pattern which match on
                // data.bytes().
                let similarity = data.similar_bytes(pattern.clone());
                let consumes_entire_pattern = similarity == pattern.len();
                let consumes_entire_leaf = similarity == data.bytes().len();
                let no_match = similarity == 0;
                // Case 1: Exact match
                // If no bytes remain, return my integer.
                if consumes_entire_pattern && consumes_entire_leaf {
                    return data.id();
                }
                // Case 2: No match.
                if no_match {
                    unreachable!("Similarity required to have gotten his far");
                }
                // Case 3: the leaf is entirely consumed
                // but there are still bytes left in the pattern.
                //  i.e. Patterns matches fully, but pattern.len() > self.data.len()
                // Action:
                //    Create an internal node. IsComplete is true since this was a leaf.
                //    Have it's children be a leaf with the remaining bytes from the pattern.
                // Case 4: the pattern is entirely consumed, but there are still
                // bytes left on the leaf.
                // i.e. Patterns match fully, but pattern.len() < self.data.len()
                // Action:
                //     Create an internal node with the match part of the pattern.
                //     IsComplete is true, since this represents the truncation of this
                //     leaf at the end of the pattern.
                //     The child of this internal node is what remains of this node's bytes.
                if consumes_entire_leaf || consumes_entire_pattern {
                    // get a new ID for the forcoming leaf node.
                    let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                    let left = (id, pattern);
                    let right = (data.id(), data.bytes().clone());
                    *self = Self::build_full_match(left, right);
                    return id;
                }
                // Case 5: Two roads diverge in a Yellow Wood
                // The match is complete for neither. Here, we create an internal node with
                // two children: 1 for the leaf's remaining bytes and one for the pattern's
                // remaining bytes.
                // get a new ID for the forcoming leaf node.
                let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                let mut pattern_copy = pattern.clone();
                let pattern_leftover = pattern_copy.split_off(similarity);
                let leaf_leftover = data.bytes().clone().split_off(similarity);
                let pattern_leaf = DLBNode::Leaf(LeafData::new(id, pattern_leftover));
                let existing_leaf = DLBNode::Leaf(LeafData::new(data.id(), leaf_leftover));
                let children = vec![pattern_leaf, existing_leaf];
                let internal_data = InternalData::new(pattern_copy, None, children);
                let internal_node = DLBNode::Internal(internal_data);
                *self = internal_node;
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

    pub fn get(&self, pattern: &[u8]) -> Option<Identifier> {
        println!("Checking node…");
        match self {
            DLBNode::Leaf(data) => {
                println!("I am a leaf.");
                // Check if the list matches the rest of the elements:
                if data.bytes().as_slice() == pattern {
                    return Some(data.id());
                }
            }
            DLBNode::Internal(data) => {
                println!("I am internal.");
                println!("My callee pattern is {}", from_utf8(pattern).unwrap());
                if pattern.len() == 0 {
                    return data.maybe_id();
                }
                let my_pattern = from_utf8(data.bytes().as_slice()).unwrap();
                println!("Comparing against my pattern {}…", my_pattern);
                if data.bytes().as_slice() != pattern {
                    println!("Not matching…");
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

impl Matchable for DLBNode {
    fn similar_bytes(&self, pattern: CharList) -> usize {
        match self {
            DLBNode::Leaf(data) => data.similar_bytes(pattern),
            DLBNode::Internal(data) => data.similar_bytes(pattern),
        }
    }
}
