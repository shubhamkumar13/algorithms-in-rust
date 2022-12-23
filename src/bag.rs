use std::collections::hash_map::Iter;

pub trait Bag<Item> {
    fn add(&mut self, item: Item) -> ();
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

use std::collections::HashMap;

pub struct HashBag<Item> {
    elements: HashMap<usize, Item>,
}

impl<Item> HashBag<Item> {
    fn new() -> Self {
        Bag {
            elements: HashMap::new(),
        }
    }

    fn iter(&self) -> 
}

impl<Item> Bag<Item> for HashBag<Item> {
    fn add(&mut self, item: Item) {
        match self.elements.into_keys.collect::<Vec<_>>().pop() {
            None => self.elements.insert(1, item),
            Some(i) => self.elements.insert(i + 1, item),
        }
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn size(&self) -> usize {
        self.elements.len()
    }
}
