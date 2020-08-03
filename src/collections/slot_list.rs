#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Key {
    index: usize,
    generation: usize,
}

impl Key {
    fn new(index: usize, generation: usize) -> Self {
        Self {
            index,
            generation,
        }
    }
}

pub struct SlotList<T> {
    items: Vec<Option<T>>,
    generations: Vec<usize>,
    empty: Vec<usize>,
}

impl<T> SlotList<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            generations: Vec::new(),
            empty: Vec::new(),
        }
    }

    /// Inserts the item into this SlotList and returns a Key to it.
    pub fn insert(&mut self, item: T) -> Key {
        if let Some(index) = self.empty.pop() {
            self.items[index] = Some(item);
            self.generations[index] += 1;
            let generation = self.generations[index];

            Key::new(index, generation)
        } else {
            let index = self.items.len();
            self.items.push(Some(item));
            self.generations.push(0);

            Key::new(index, 0)
        }
    }

    /// Removes an item from this SlotList if one was stored for the given key, invalidates the key and returns the item. Otherwise returns None.
    pub fn remove(&mut self, key: Key) -> Option<T> {
        if key.index > self.items.len() {
            return None;
        }

        if self.generations[key.index] != key.generation {
            return None;
        }

        self.generations[key.index] += 1;
        self.empty.push(key.index);
        self.items[key.index].take()
    }

    /// Returns an immutable reference to an item from this SlotList if an item is still stored in it for the given key, otherwise returns None.
    pub fn get(&self, key: Key) -> Option<&T> {
        if key.index > self.items.len() {
            return None;
        }

        if self.generations[key.index] != key.generation {
            return None;
        }

        if let Some(item) = &self.items[key.index] {
            Some(item)
        } else {
            None
        }
    }

    /// Returns a mutable reference to an item from this SlotList if an item is still stored in it for the given key, otherwise returns None.
    pub fn get_mut(&mut self, key: Key) -> Option<&mut T> {
        if key.index > self.items.len() {
            return None;
        }

        if self.generations[key.index] != key.generation {
            return None;
        }

        if let Some(item) = &mut self.items[key.index] {
            Some(item)
        } else {
            None
        }
    }
}
