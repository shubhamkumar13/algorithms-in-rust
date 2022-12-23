use std::collections::vec_deque::Iter;

pub trait Queue<Item> {
    fn enqueue(&mut self, item: Item);
    fn dequeue(&mut self) -> Option<Item>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

use std::collections::VecDeque;

#[derive(Debug)]
pub struct DQueue<Item> {
    elements: VecDeque<Item>,
}

impl<Item> DQueue<Item> {
    pub fn new() -> Self {
        DQueue {
            elements: VecDeque::new(),
        }
    }

    pub fn iter(&self) -> Iter<'_, Item> {
        self.elements.iter()
    }
}

impl<Item> Queue<Item> for DQueue<Item> {
    fn enqueue(&mut self, item: Item) {
        self.elements.push_back(item);
    }

    fn dequeue(&mut self) -> Option<Item> {
        self.elements.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn size(&self) -> usize {
        self.elements.len()
    }
}
