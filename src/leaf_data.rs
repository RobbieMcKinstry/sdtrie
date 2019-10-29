use crate::char_list::CharList;
use crate::Identifier;

pub struct LeafData {
    bytes: CharList,
    id: Identifier,
}

impl LeafData {
    pub fn bytes(&self) -> &CharList {
        &self.bytes
    }

    pub fn id(&self) -> Identifier {
        self.id
    }
}
