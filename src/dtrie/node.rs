use crate::dtrie::char_list::CharList;
use crate::dtrie::internal_data::InternalData;
use crate::dtrie::leaf_data::LeafData;
use crate::dtrie::value::RadixValue;
use crate::dtrie::Identifier;
use crate::dtrie::Matchable;
use im::vector;
use std::mem::size_of;
use std::str::from_utf8;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone)]
pub enum RadixNode<T: RadixValue + Clone> {
    Leaf(LeafData),
    Internal(InternalData<T>),
}

type NodeDescription = (Identifier, CharList);

impl<T: RadixValue + Clone> RadixNode<T> {
    pub fn count_nodes(&self) -> u64 {
        match self {
            RadixNode::Leaf(data) => {
                println!("…counting leaf pattern={}.", data.bytes().to_string());
                return 1;
            }
            RadixNode::Internal(data) => {
                println!(
                    "…counting intern pattern={} with {} kids.",
                    data.bytes().to_string(),
                    data.children().len()
                );
                1 + data
                    .children()
                    .iter()
                    .map(|node| node.count_nodes())
                    .fold(0, |acc, inc| acc + inc)
            }
        }
    }

    pub fn bytes(&self) -> CharList {
        match self {
            RadixNode::Leaf(data) => data.bytes().clone(),
            RadixNode::Internal(data) => data.bytes().clone(),
        }
    }

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
        let leaf = RadixNode::Leaf(leaf_data);
        // Now build the internal node.
        let child = vector![leaf];
        let internal_data = InternalData::new(smallest.1, Some(smallest.0), child);
        Self::Internal(internal_data)
    }

    pub fn insert(&mut self, mut pattern: CharList, next_id: &mut AtomicU64) -> Identifier {
        match self {
            RadixNode::Leaf(data) => {
                // Consume any if the characters in pattern which match on
                // data.bytes().
                let similarity = data.similar_bytes(pattern.clone());
                let consumes_entire_pattern = similarity == pattern.len();
                let consumes_entire_leaf = similarity == data.bytes().len();
                let no_match = similarity == 0;
                // Case 1: No match.
                if no_match {
                    println!("My data: {:?}", data.bytes());
                    println!("My pattern: {:?}", pattern);
                    unreachable!("Similarity required to have gotten his far");
                }
                // Case 2: Exact match
                // If no bytes remain, return my integer.
                if consumes_entire_pattern && consumes_entire_leaf {
                    return data.id();
                }
                // Case 3: Exactly one of the leaf or the pattern is entirely consumed
                // i.e. pattern.len() > self.data.len() OR pattern.len() < self.data.len()
                // Action:
                //    Create an internal node. IsComplete is true since it matches either
                //    the leaf entirely or the pattern entirely.
                //    Have it's children be a leaf with the remaining bytes from the match.
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
                let pattern_leaf = RadixNode::Leaf(LeafData::new(id, pattern_leftover));
                let existing_leaf = RadixNode::Leaf(LeafData::new(data.id(), leaf_leftover));
                let children = vector![pattern_leaf, existing_leaf];
                let internal_data = InternalData::new(pattern_copy, None, children);
                let internal_node = RadixNode::Internal(internal_data);
                *self = internal_node;
                return id;
            }
            RadixNode::Internal(data) => {
                // Consume any of the characters in pattern which match on
                // data.bytes().
                let similarity = data.similar_bytes(pattern.clone());
                let consumes_pattern_exactly = similarity == pattern.len();
                let consumes_bytestring_exactly = similarity == data.bytes().len();
                let no_match = similarity == 0;

                match (
                    no_match,
                    consumes_pattern_exactly,
                    consumes_bytestring_exactly,
                ) {
                    // Case 1: Exact match.
                    // In this case, we just need to check the IsComplete field
                    // and perhaps update it.
                    (false, true, true) => {
                        if let Some(id) = data.maybe_id() {
                            return id;
                        } else {
                            let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                            data.set_maybe_id(Some(id));
                            return id;
                        }
                    }
                    // Case 2: Similarity == pattern.len() && Similarity < bytestring.len()
                    // Make a new internal node for the overlapping pattern. This internal
                    // node has `IsComplete` set to true.
                    // Take the leftover bytes from the bytestring and make an internal
                    // node for that. That second internal node gets the children of this node.
                    (false, true, false) => {
                        let mut internal = data.bytes().clone();
                        let internal_leftovers = internal.split_off(similarity);

                        let second_layer_data = InternalData::new(
                            internal_leftovers,
                            data.maybe_id(),
                            data.children().clone(),
                        );
                        let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                        let second_layer = RadixNode::Internal(second_layer_data);
                        let children = vector![second_layer];
                        let internal_data = InternalData::new(pattern, Some(id), children);
                        *self = RadixNode::Internal(internal_data);
                        return id;
                    }
                    // Case 3: Similarity < pattern.len() && similarity == bytestring.len
                    // Strip pattern of the similar bytes and recurse.
                    (false, false, true) => {
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
                                let new_leaf = RadixNode::Leaf(new_leaf_data);
                                data.add_child(new_leaf);
                                return id;
                            }
                        }
                    }
                    // Case 4: Similarity < pattern.len() && similarity < bytestring.len()
                    // Make a new internal node for the overlapping pattern
                    // Take the leftover bytes from the bytestring, and make an internal
                    // node for that. That second internal node gets the childrens of this node
                    // Make a new leaf for the leftover bytes from pattern
                    (false, false, false) => {
                        let mut internal = data.bytes().clone();
                        let pattern_leftovers = pattern.split_off(similarity);
                        let internal_leftovers = internal.split_off(similarity);

                        let second_layer_data = InternalData::new(
                            internal_leftovers,
                            data.maybe_id(),
                            data.children().clone(),
                        );
                        let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                        let second_layer = RadixNode::Internal(second_layer_data);
                        let new_leaf_data = LeafData::new(id, pattern_leftovers);
                        let new_leaf = RadixNode::Leaf(new_leaf_data);
                        let children = vector![second_layer, new_leaf];
                        let internal_data = InternalData::new(pattern, None, children);
                        *self = RadixNode::Internal(internal_data);
                        return id;
                    }
                    // Case 5: No match.
                    (true, _, _) => unreachable!(), // Cannot reach this node without a match
                }
            }
        }
    }

    pub fn get(&self, pattern: Vec<u8>) -> Option<Identifier> {
        println!("Checking node…");
        match self {
            RadixNode::Leaf(data) => {
                println!("I am a leaf.");
                // Check if the list matches the rest of the elements:
                if data.bytes().as_slice() == pattern.as_slice() {
                    return Some(data.id());
                }
            }
            RadixNode::Internal(data) => {
                println!("I am internal.");

                println!(
                    "My callee pattern is {}",
                    from_utf8(pattern.as_slice()).unwrap()
                );
                let similarity = data.bytes().count_shared_prefix(pattern.as_slice());
                let consumes_pattern_exactly = similarity == pattern.len();
                let consumes_bytestring_exactly = similarity == data.bytes().len();
                let no_match = similarity == 0;

                match (
                    no_match,
                    consumes_pattern_exactly,
                    consumes_bytestring_exactly,
                ) {
                    // Case 1: No match
                    (true, _, _) => return None,
                    // Case 2: Exact match.
                    // In this case, we just need to check the IsComplete field
                    // and perhaps update it.
                    (false, true, true) => return data.maybe_id(),
                    // Case 3: Matches pattern, bytes have leftover
                    (false, true, false) => return None,
                    // Case 4: Matches bytes, pattern has leftover
                    (false, false, true) => {
                        // Else, cut off the prefix from the pattern,
                        // and iterate over the children, ORing the results together.
                        let match_len = data.bytes().len();
                        let suffix = &pattern[match_len..];
                        for child in data.children() {
                            if let Some(result) = child.get(suffix.to_vec()) {
                                return Some(result);
                            }
                        }
                        return None;
                    }
                    (false, false, false) => return None,
                }
            }
        }
        None
    }

    pub fn contains(&self, pattern: Vec<u8>) -> bool {
        match self.get(pattern) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn size_of(&self) -> usize {
        match self {
            RadixNode::Leaf(_) => size_of::<Self>(),
            RadixNode::Internal(data) => {
                size_of::<Self>()
                    + data
                        .children()
                        .iter()
                        .map(|child| child.size_of())
                        .fold(0, |x, acc| x + acc)
            }
        }
    }

    pub fn resolve(&self, id: Identifier, thus_far: CharList) -> Option<String> {
        match self {
            RadixNode::Leaf(data) => {
                if data.id() == id {
                    let result = thus_far.append(data.bytes().clone());
                    return Some(result.to_string());
                }
                return None;
            }
            RadixNode::Internal(data) => {
                if let Some(internal_id) = data.maybe_id() {
                    if internal_id == id {
                        let result = thus_far.append(data.bytes().clone());
                        return Some(result.to_string());
                    }
                }

                let next_string = thus_far.append(data.bytes().clone());
                let res = data
                    .children()
                    .iter()
                    .map(|child| child.resolve(id, next_string.clone()))
                    .filter(|x| x.is_some())
                    .next();
                match res {
                    None => None,
                    Some(x) => Some(x.unwrap()),
                }
            }
        }
    }
}

impl<T: RadixValue + Clone> Matchable for RadixNode<T> {
    fn similar_bytes(&self, pattern: CharList) -> usize {
        match self {
            RadixNode::Leaf(data) => data.similar_bytes(pattern),
            RadixNode::Internal(data) => data.similar_bytes(pattern),
        }
    }
}
