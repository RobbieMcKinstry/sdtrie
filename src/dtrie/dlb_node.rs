use crate::dtrie::char_list::CharList;
use crate::dtrie::internal_data::InternalData;
use crate::dtrie::leaf_data::LeafData;
use crate::dtrie::Identifier;
use crate::dtrie::Matchable;
use im::vector;
use std::str::from_utf8;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone)]
pub enum DLBNode {
    Leaf(LeafData),
    Internal(InternalData),
}

type NodeDescription = (Identifier, CharList);

impl DLBNode {
    pub fn count_nodes(&self) -> u64 {
        match self {
            DLBNode::Leaf(data) => {
                println!("…counting leaf pattern={}.", data.bytes().to_string());
                return 1;
            }
            DLBNode::Internal(data) => {
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
        let child = vector![leaf];
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
                let pattern_leaf = DLBNode::Leaf(LeafData::new(id, pattern_leftover));
                let existing_leaf = DLBNode::Leaf(LeafData::new(data.id(), leaf_leftover));
                let children = vector![pattern_leaf, existing_leaf];
                let internal_data = InternalData::new(pattern_copy, None, children);
                let internal_node = DLBNode::Internal(internal_data);
                *self = internal_node;
                return id;
            }
            DLBNode::Internal(data) => {
                // Consume any of the characters in pattern which match on
                // data.bytes().
                let similarity = data.similar_bytes(pattern.clone());
                let consumes_entire_pattern = similarity == pattern.len();
                let consumes_entire_bytestring = similarity >= data.bytes().len();
                let consumes_node_exactly = similarity == data.bytes().len();
                let no_match = similarity == 0;
                // Case 1: No match.
                if no_match {
                    println!("My data: {}", data.bytes());
                    println!("My pattern: {}", pattern);
                    unreachable!("Similarity required to have gotten his far");
                }
                // Case 2: Exact match
                // In this case, we just need to check the IsComplete field
                // and perhaps update it.
                if consumes_entire_pattern && consumes_node_exactly {
                    if let Some(id) = data.maybe_id() {
                        return id;
                    } else {
                        let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                        data.set_maybe_id(Some(id));
                        return id;
                    }
                }
                // Case 3: Similarity < pattern.len and the matching ends at this node.
                // In this case, we simply add a new leaf node for the rest of the pattern.
                if similarity < pattern.len() && consumes_node_exactly {
                    println!(
                        "Similary < pattern.len for interning pattern {}",
                        pattern.clone().to_string()
                    );
                    let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                    let leftover = pattern.split_off(similarity);
                    let leaf = DLBNode::Leaf(LeafData::new(id, leftover));
                    data.add_child(leaf);
                    return id;
                }

                // Case 4: Similarity < pattern.len and matching extends beyond this node.
                if similarity < pattern.len() && !consumes_entire_bytestring {
                    let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                    let mut new_internal_pattern = pattern.clone();
                    new_internal_pattern.split_off(similarity);
                    let pattern_leftovers = pattern.clone().split_off(similarity);
                    let bytestring_leftover = pattern.clone().split_off(similarity);
                    println!(
                        "Pattern leftovers={}",
                        pattern_leftovers.clone().to_string()
                    );
                    println!(
                        "bytestring leftovers={}",
                        bytestring_leftover.clone().to_string()
                    );

                    let leaf = DLBNode::Leaf(LeafData::new(id, pattern_leftovers));
                    let second_level = InternalData::new(
                        bytestring_leftover,
                        data.maybe_id(),
                        data.clone_children(),
                    );
                    let children = vector![leaf, DLBNode::Internal(second_level)];
                    let internal_data = InternalData::new(new_internal_pattern, None, children);
                    *self = DLBNode::Internal(internal_data);
                    return id;
                }

                // Now, similarity must be == to pattern.len
                // Case 4:
                // data.bytes < similarity
                // In this case, we have matched this entire node. Time to recurse!
                //     In this case, we need to pass the remaining bytes along to our children.
                //     If our children have no overlap, then we make a new leaf.
                if data.bytes().len() < similarity {
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
                // Case 4:
                // similarity < data.bytes (and similarity == pattern.len)
                // Action:
                //    Make a new internal node for the overlap.
                //    It should be IsComplete=true, and its children should be this node.
                let id = Identifier::from(next_id.fetch_add(1, Ordering::Relaxed));
                let remaining = data.bytes().clone().split_off(similarity);
                let second_level =
                    InternalData::new(remaining, data.maybe_id(), data.clone_children());
                let children = vector![DLBNode::Internal(second_level)];
                let internal_data = InternalData::new(pattern, Some(id), children);
                *self = DLBNode::Internal(internal_data);
                return id;
            }
        }
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

    pub fn resolve(&self, id: Identifier, thus_far: CharList) -> Option<String> {
        match self {
            DLBNode::Leaf(data) => {
                if data.id() == id {
                    let result = thus_far.append(data.bytes().clone());
                    return Some(result.to_string());
                }
                return None;
            }
            DLBNode::Internal(data) => {
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

impl Matchable for DLBNode {
    fn similar_bytes(&self, pattern: CharList) -> usize {
        match self {
            DLBNode::Leaf(data) => data.similar_bytes(pattern),
            DLBNode::Internal(data) => data.similar_bytes(pattern),
        }
    }
}
