use crate::dtrie::Matchable;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct CharList {
    table: Rc<RefCell<Vec<u8>>>,
    start: usize,
    end: usize,
}

pub struct CharIterator {
    pos: usize,
    table: Vec<u8>,
}

impl CharIterator {
    fn new(table: Vec<u8>) -> Self {
        Self { pos: 0, table }
    }
}

impl Iterator for CharIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.table.len() {
            None
        } else {
            let val = self.table[self.pos];
            self.pos += 1;
            Some(val)
        }
    }
}

impl CharList {
    pub fn empty() -> Self {
        let empty_vec = Vec::new();
        Self {
            table: Rc::new(RefCell::new(empty_vec)),
            start: 0,
            end: 0,
        }
    }

    pub fn new(chars: Vec<u8>) -> Self {
        Self {
            start: 0,
            end: chars.len(),
            table: Rc::new(RefCell::new(chars)),
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn as_vec(&self) -> Vec<u8> {
        let contents = &self.table.borrow_mut()[self.start..self.end];
        Vec::from(contents)
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> {
        CharIterator::new(self.as_vec())
    }

    pub fn split_off(&mut self, at: usize) -> CharList {
        self.end = at;
        Self {
            start: at,
            end: self.end,
            table: self.table.clone(),
        }
    }

    pub fn append(&self, other: CharList) -> &CharList {
        self.table.borrow_mut().splice(self.start..self.end, other);
        self
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(Vec::from(self.as_vec())).unwrap()
    }

    pub fn count_shared_prefix(&self, bytes: &[u8]) -> usize {
        let mut count = 0;
        for (x, y) in bytes.iter().zip(self.iter()) {
            if *x != y {
                break;
            }
            count += 1
        }
        return count;
    }
}

impl Matchable for CharList {
    fn similar_bytes(&self, pattern: CharList) -> usize {
        let mut count = 0;
        let i1 = self.iter();
        let i2 = pattern.iter();
        for (left, right) in i1.zip(i2) {
            if left == right {
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
        let contents = &self.table.borrow_mut()[self.start..self.end];
        Vec::from(contents).into_iter()
    }
}

impl fmt::Display for CharList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for CharList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<Vec<u8>> for CharList {
    fn from(other: Vec<u8>) -> Self {
        CharList::new(other)
    }
}
