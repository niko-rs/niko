use std::collections::HashSet;
use crate::{
    Key,
    Context,
};

pub fn is_key_down(context: &Context, key: Key) -> bool {
    context.input.is_key_down(key)
}

pub struct Input {
    keys: HashSet<Key>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys: HashSet::new(),
        }
    }

    pub(crate) fn set_key(&mut self, key: Key) {
        self.keys.insert(key);
    }

    pub(crate) fn reset_key(&mut self, key: Key) {
        self.keys.remove(&key);
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys.contains(&key)
    }
}
