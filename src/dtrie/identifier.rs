use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Default, Ord, PartialOrd)]
pub struct Identifier(u64);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for Identifier {
    fn from(other: u64) -> Identifier {
        Identifier(other)
    }
}
