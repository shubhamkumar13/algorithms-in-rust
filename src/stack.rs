use std::slice::Iter;

pub trait Stack<Item> {
    fn push(&mut self, item: Item) -> ();
    fn pop(&mut self) -> Option<Item>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct VecStack<Item> {
    elements: Vec<Item>,
}

impl<Item> VecStack<Item> {
    pub fn new() -> VecStack<Item> {
        VecStack { elements: vec![] }
    }

    pub fn iter(&self) -> Iter<'_, Item> {
        self.elements.iter()
    }
}

impl<Item> Stack<Item> for VecStack<Item> {
    fn push(&mut self, item: Item) -> () {
        self.elements.push(item)
    }

    fn pop(&mut self) -> Option<Item> {
        self.elements.pop()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn size(&self) -> usize {
        self.elements.len()
    }
}
