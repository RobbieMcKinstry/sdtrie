use crate::dtrie::char_list::CharList;
use crate::dtrie::dlb_node::DLBNode;
use crate::dtrie::is_complete::IsComplete;
use crate::dtrie::Identifier;
use crate::dtrie::Matchable;
use im::Vector;
use std::sync::atomic::AtomicU64;

#[derive(Clone)]
pub struct InternalData {
    bytes: CharList,
    maybe_id: IsComplete,
    children: Vector<DLBNode>,
}

impl InternalData {
    pub fn new(bytes: CharList, complete: IsComplete, children: Vector<DLBNode>) -> Self {
        Self {
            bytes,
            children,
            maybe_id: complete,
        }
    }

    pub fn set_maybe_id(&mut self, id: IsComplete) {
        self.maybe_id = id;
    }

    pub fn bytes(&self) -> &CharList {
        &self.bytes
    }

    pub fn maybe_id(&self) -> Option<Identifier> {
        self.maybe_id
    }

    pub fn children(&self) -> &Vector<DLBNode> {
        &self.children
    }

    pub fn add_child(&mut self, node: DLBNode) {
        self.children.push_back(node);
    }

    pub fn insert_at_index(
        &mut self,
        idx: usize,
        pattern: CharList,
        next_id: &mut AtomicU64,
    ) -> Identifier {
        self.children[idx].insert(pattern, next_id)
    }

    /// `find_best_child` checks all children for the
    /// one which has the highest similarity to this pattern.
    /// It returns the index of the child with the highest value, and that value.
    pub fn find_best_child(&self, pattern: CharList) -> (Option<usize>, usize) {
        let mut max = 0;
        let mut best_index = None;
        let mut index = 0;
        for child in self.children().iter() {
            let next_count = child.similar_bytes(pattern.clone());
            if next_count > max {
                best_index = Some(index);
                max = next_count;
            }
            index += 1;
        }
        (best_index, max)
    }

    pub fn clone_children(&self) -> Vector<DLBNode> {
        self.children.clone()
    }

    pub fn total_matching_bytes(&self, pattern: CharList) -> usize {
        // First, iterate over a char list and grab any similar bytes.
        let similarity = self.bytes().similar_bytes(pattern.clone());
        if similarity == self.bytes().len() {
            // Then, it has matches this entire node.
            // Chop off those bytes and keep going.
            let skippable = self.bytes().len();
            let remaining_bytes: Vec<u8> = pattern.iter().skip(skippable).collect();
            let remaining = CharList::from(remaining_bytes);
            let mut max = 0;
            for child in self.children().iter() {
                // WARNING: I can get rid of this clone by changing the similar_bytes() fn
                // to take a reference instead.
                let next_count = child.similar_bytes(remaining.clone());
                if next_count > max {
                    max = next_count;
                }
            }
            return skippable + max;
        }
        similarity
    }
}

impl Matchable for InternalData {
    fn similar_bytes(&self, pattern: CharList) -> usize {
        self.bytes().similar_bytes(pattern)
    }
}
