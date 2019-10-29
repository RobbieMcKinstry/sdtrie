use crate::char_list::CharList;
use crate::dlb_node::DLBNode;
use crate::is_complete::IsComplete;
use crate::Identifier;

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

    /// `find_best_child` checks all children for the
    /// one which has the highest similarity to this pattern.
    /// It returns the children with the highest value and that value.
    pub fn find_best_child(&self, pattern: CharList) -> (Option<&mut DLBNode>, usize) {
        let mut max = 0;
        let mut best = None;
        for child in self.children().iter_mut() {
            let next_count = child.similar_bytes(pattern.clone());
            if next_count > max {
                max = next_count;
                best = Some(child);
            }
        }

        (best, max)
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
