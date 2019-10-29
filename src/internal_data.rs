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
    pub fn bytes(&self) -> &CharList {
        &self.bytes
    }

    pub fn maybe_id(&self) -> Option<Identifier> {
        self.maybe_id
    }

    pub fn children(&self) -> &Vec<DLBNode> {
        &self.children
    }
}
