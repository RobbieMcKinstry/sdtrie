use crate::char_list::CharList;
use crate::dlb_node::DLBNode;
use crate::is_complete::IsComplete;
use crate::Identifier;
use std::sync::atomic::AtomicU64;

pub struct InternalData {
    bytes: CharList,
    maybe_id: IsComplete,
    children: Vec<DLBNode>,
}

impl InternalData {
    pub fn new(bytes: CharList, complete: IsComplete, children: Vec<DLBNode>) -> Self {
        Self {
            bytes,
            children,
            maybe_id: complete,
        }
    }

    pub fn bytes(&self) -> &CharList {
        &self.bytes
    }

    pub fn maybe_id(&self) -> Option<Identifier> {
        self.maybe_id
    }

    pub fn children(&self) -> &Vec<DLBNode> {
        &self.children
    }

    pub fn add_child(&mut self, node: DLBNode) {
        self.children.push(node);
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
        }
        (best_index, max)
    }

    pub fn similar_bytes(&self, pattern: CharList) -> usize {
        // First, iterate over a char list and grab any similar bytes.
        let similarity = self.bytes().similar_bytes(pattern.clone());
        if similarity == self.bytes().len() {
            // Then, it has matches this entire node.
            // Chop off those bytes and keep going.
            let skippable = self.bytes().len();
            let remaining_bytes: Vec<u8> = pattern.iter().skip(skippable).map(|b| *b).collect();
            let remaining = CharList::from(remaining_bytes);
            let mut max = 0;
            for child in self.children() {
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
