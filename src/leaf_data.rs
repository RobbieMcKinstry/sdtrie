use crate::char_list::CharList;
use crate::Identifier;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct LeafData {
    bytes: CharList,
    id: Identifier,
}

impl LeafData {
    pub fn new(id: Identifier, bytes: CharList) -> Self {
        Self { id, bytes }
    }

    pub fn new_from_generator(bytes: CharList, next_id: &mut AtomicU64) -> Self {
        let id_int = next_id.fetch_add(1, Ordering::Relaxed);
        let id = Identifier::from(id_int);
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
