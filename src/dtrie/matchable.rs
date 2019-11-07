use crate::dtrie::char_list::CharList;

/// A `Matchable` is anything that lets your measure
/// the total number of bytes that match a given pattern.
pub trait Matchable {
    /// `similar_bytes` returns the number of bytes on
    /// this particular object which match the pattern.
    /// This function should not be recurse.
    fn similar_bytes(&self, pattern: CharList) -> usize;
}
