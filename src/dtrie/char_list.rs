use crate::dtrie::Matchable;
use std::fmt;

#[derive(Clone)]
pub struct CharList(Vec<u8>);

impl CharList {
    pub fn new(other: Vec<u8>) -> Self {
        Self(other)
    }

    pub fn empty() -> Self {
        let empty_vec = Vec::new();
        Self::new(empty_vec)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.0.iter()
    }

    pub fn split_off(&mut self, at: usize) -> CharList {
        let res = self.0.split_off(at);
        CharList::from(res)
    }

    pub fn append(&self, mut other: CharList) -> CharList {
        let mut res = self.0.clone();
        res.append(&mut other.0);
        Self(res)
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.0.clone()).unwrap()
    }
}

impl Matchable for CharList {
    fn similar_bytes(&self, pattern: CharList) -> usize {
        let mut count = 0;
        let i1 = self.0.iter();
        let i2 = pattern.iter();
        for (left, right) in i1.zip(i2) {
            if *left == *right {
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}

impl IntoIterator for CharList {
    type Item = u8;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for CharList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        for character in self.0.clone() {
            res = format!("{}{}", res, character);
        }
        write!(f, "{}", res)
    }
}

impl fmt::Debug for CharList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.0.clone()).unwrap())
    }
}

impl From<Vec<u8>> for CharList {
    fn from(other: Vec<u8>) -> Self {
        CharList::new(other)
    }
}
