use crate::Identifier;

pub struct DLB {
    root: Option<DLBNode>,
}

type IsComplete = bool;

type CharList = Vec<u8>;

enum DLBNode {
    Leaf(CharList),
    Internal(CharList, IsComplete, Vec<DLBNode>),
}

impl DLB {
    pub fn new() -> Self {
        return Self { root: None };
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

    //fn intern(&mut self, contents: String) -> Identifier {
    //     Identifier::from(0)
    //}
}

impl DLBNode {
    fn contains(&self, pattern: &[u8]) -> bool {
        match self {
            DLBNode::Leaf(list) => {
                // Check if the list matches the rest of the elements:
                return list.as_slice() == pattern;
            }
            DLBNode::Internal(list, complete, next) => {
                if *complete && pattern.len() == 0 {
                    return true;
                }
                if list.as_slice() != pattern {
                    return false;
                }
                // Else, cut off the prefix from the pattern,
                // and iterate over the children, ORing the results together.
                let match_len = list.len();
                let suffix = &pattern[match_len..];
                for child in next {
                    if child.contains(&suffix) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
