use crate::dtrie::char_list::CharList;

pub trait Matchable {
    fn similar_bytes(&self, pattern: CharList) -> usize;
}
