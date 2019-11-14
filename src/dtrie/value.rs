pub trait RadixValue: Eq + Into<Vec<u8>> + From<Vec<u8>> {}

impl<T: Eq + Into<Vec<u8>> + From<Vec<u8>>> RadixValue for T {}

impl RadixValue for String {}
