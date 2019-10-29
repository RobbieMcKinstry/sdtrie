use crate::char_list::CharList;
use crate::Identifier;

pub struct LeafData {
    bytes: CharList,
    id: Identifier,
}

impl LeafData {
    pub fn new(id: Identifier, bytes: CharList) -> Self {
        Self { id, bytes }
    }

    pub fn bytes(&self) -> &CharList {
        &self.bytes
    }

    pub fn id(&self) -> Identifier {
        self.id
    }

    pub fn similar_bytes(&self, pattern: CharList) -> usize {
        self.bytes().similar_bytes(pattern)
    }
}
